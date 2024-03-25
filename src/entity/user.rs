#![allow(unused)]
use crate::{
    auth::{hash_password, verify_password_hash},
    routes::user,
};
use anyhow::Context;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// User account that is able to login to the application.
///
/// Provides functionality to create new users in the database
/// and query the database for users.
#[derive(Debug, Clone)]
pub struct User {
    /// Uuid4 used as primary key in database
    id: Uuid,
    /// E-mail address. Validation happens in the database
    mail: String,
    username: String,
    /// PHC string. Generated via Argon2
    password_hash: String,
    /// Whether the user has verified his email address
    enabled: bool,
}

impl User {
    /// Create the user in the database with the specified values.
    ///
    /// Used when a user trys to register for the service. A new row will be created
    /// in the DB via a transaction. Returns an error when communication with the db fails.
    ///
    /// * `mail`: e-mail address of the user. gets validated at the database
    /// * `username`: username for this user. must be unique
    /// * `password`: password for this user. gets hashed before being stored
    /// * `pool`: pool to use for storage
    /// * `pepper`: pepper to use for hashing
    pub async fn create(
        mail: &str,
        username: &str,
        password: String,
        pool: &PgPool,
        pepper: Vec<u8>,
    ) -> Result<Uuid, anyhow::Error> {
        let hash = actix_web::rt::task::spawn_blocking(move || {
            hash_password(password.as_str(), pepper.as_slice())
        })
        .await??;

        let mut tx = pool.begin().await?;
        let id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO pastr.users (id, username, mail, password_hash, enabled) 
            VALUES ($1, $2, $3, $4, $5);",
        )
        .bind(id)
        .bind(username)
        .bind(mail)
        .bind(hash)
        .bind(false)
        .execute(&mut *tx)
        .await?;

        sqlx::query("INSERT INTO pastr.users_confirmations (user_id) VALUES ($1);")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(id)
    }

    /// Activate a given user, allowing him to log in to the application.
    ///
    /// Updates the user with the given `id` to be enabled. Requires that a row with the same id
    /// exists in `users_confirmations`. Function returns an error if there is no signup request present.
    ///
    /// * `id` - UUID of the user to activate
    /// * `pool` - Connection pool to use for the queries
    pub async fn activate(id: &Uuid, pool: &PgPool) -> Result<(), anyhow::Error> {
        // check if there is a valid request for the given user
        let signup_exists = sqlx::query(
            "SELECT EXISTS(
                SELECT * FROM pastr.users_confirmations
                WHERE user_id = $1
            );",
        )
        .bind(id)
        .fetch_one(pool)
        .await?
        .try_get::<bool, &str>("exists")?;

        // enable the user if there was a valid request and delete it.
        if signup_exists {
            let mut tx = pool.begin().await?;
            sqlx::query(
                "
                UPDATE pastr.users
                SET enabled = true
                WHERE id = $1;
                ",
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "
                DELETE FROM pastr.users_confirmations
                WHERE user_id = $1
                ",
            )
            .bind(id)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            Ok(())
        } else {
            // return an error if there was no request
            Err(anyhow::anyhow!(
                "no signup request exists for given user id"
            ))
        }
    }

    pub async fn login(
        username: &str,
        password: &str,
        pool: &PgPool,
        pepper: Vec<u8>,
    ) -> Result<(), anyhow::Error> {
        let password_hash =
            sqlx::query("SELECT password_hash FROM pastr.users WHERE username = $1;")
                .bind(username)
                .fetch_one(pool)
                .await
                .context("failed to retrieve password hash for user")?
                .try_get::<String, &str>("password_hash")?;

        Ok(
            verify_password_hash(password, password_hash.as_str(), pepper.as_slice())
                .context("passwords do not match")?,
        )
    }
}
