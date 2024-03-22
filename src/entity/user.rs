#![allow(unused)]
use crate::auth::hash_password;
use sqlx::PgPool;
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
    ) -> Result<(), anyhow::Error> {
        let hash = actix_web::rt::task::spawn_blocking(move || {
            hash_password(password.as_str(), pepper.as_slice())
        })
        .await??;

        let mut tx = pool.begin().await?;
        let id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO pastr.users (id, username, mail, password_hash) 
            VALUES ($1, $2, $3, $4);",
        )
        .bind(id)
        .bind(username)
        .bind(mail)
        .bind(hash)
        .execute(&mut *tx)
        .await?;

        sqlx::query("INSERT INTO pastr.users_confirmations (user_id) VALUES ($1);")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
