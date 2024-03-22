use anyhow::Context;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordVerifier, Version};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// Create an Argon2 instance that uses a specified `pepper` value for hashing and verification.
fn argon2_with_pepper(pepper: &[u8]) -> Result<Argon2, AuthError> {
    match Argon2::new_with_secret(
        pepper,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    ) {
        Ok(arg) => Ok(arg),
        Err(_) => Err(AuthError::UnexpectedError(anyhow::anyhow!(
            "failed to initalize password hasher"
        ))),
    }
}

/// Compute the Argon2 hash value of the given password using a salt and the given pepper.
///
/// Try to compute the Argon2 hash of the given password utilizing a randomly generated salt string
/// and a pepper set in [Config][c]. Returns an error Argon2 encounters an unexpected error.
///
/// * `password` - password to hash
/// * `pepper` - constant value that gets added to password. for further information see [here](https://de.wikipedia.org/wiki/Salt_(Kryptologie)#Pepper)
///
/// [c]: crate::config::Config
pub fn hash_password(password: &str, pepper: &[u8]) -> Result<String, AuthError> {
    let argon2 = argon2_with_pepper(pepper)?;

    let salt = SaltString::generate(&mut OsRng);
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(_) => Err(AuthError::UnexpectedError(anyhow::anyhow!(
            "failed to compute password hash"
        ))),
    }
}

/// Verify a password against a salted and peppered hash value.
///
/// Uses Argon2 to validate a password against a PHC String encoded, salted and peppered hash value.
/// Returns Ok(()) on successful verification or an error if password and hash do not match.
///
/// * `password`: password to verify
/// * `hash`: PHC String encoded hash that the password should be verified against
/// * `pepper`: Pepper value that was used in creating `hash``
pub fn verify_password_hash(password: &str, hash: &str, pepper: &[u8]) -> Result<(), AuthError> {
    let argon2 = argon2_with_pepper(pepper)?;
    let expected = PasswordHash::new(hash).context("failed to parse PHC string")?;

    argon2
        .verify_password(password.as_bytes(), &expected)
        .context("invalid password")
        .map_err(AuthError::InvalidCredentials)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_hash() {
        let password = "p4ssw0rd1";
        let pepper = "long and bad pepper value - not use in production";

        let hash = hash_password(password, pepper.as_bytes()).expect("hashing of password failed");
        let result = verify_password_hash(password, &hash, pepper.as_bytes());

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_password() {
        let password = "p4ssw0rd1";
        let pepper = "long and bad pepper value - not use in production";

        let hash = hash_password(password, pepper.as_bytes()).expect("hashing of password failed");
        let result = verify_password_hash("p4ssw0rd1337", &hash, pepper.as_bytes());

        assert!(result.is_err());
    }

    #[test]
    fn empty_password() {
        let password = "p4ssw0rd1";
        let pepper = "long and bad pepper value - not use in production";

        let hash = hash_password(password, pepper.as_bytes()).expect("hashing of password failed");
        let result = verify_password_hash("", &hash, pepper.as_bytes());

        assert!(result.is_err());
    }
}
