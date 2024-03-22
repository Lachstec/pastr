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
}

impl User {
    pub async fn create(
        mail: &str,
        username: &str,
        password: &str,
        pool: &PgPool,
        pepper: &[u8],
    ) -> Result<(), anyhow::Error> {
        let hash = hash_password(password, pepper)?;

        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO pastr.users (id, username, mail, password_hash) 
            VALUES ($1, $2, $3, $4);",
        )
        .bind(Uuid::new_v4())
        .bind(username)
        .bind(mail)
        .bind(hash)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}
