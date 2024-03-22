use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::TryFrom;

/// Contains general config for the application.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub base_url: String,
    pub pepper: Secret<String>,
    pub sendgrid_key: Secret<String>,
}

/// Config for the database connection.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseConfig {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: Secret<String>,
    pub database: String,
    pub use_tls: bool,
}

impl DatabaseConfig {
    pub fn as_connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.use_tls {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

/// Attempt to retrieve configuration from a config file.
///
/// Attempt to locate a config file in the location $working_dir/{environment}.yml. The filename gets detemined by the
/// `APP ENV` environment variable. For valid options see [`Enviroment`]. Returns an error if the file can not
/// be located or if `APP ENV` has an invalid value.
pub fn get_config() -> Result<Config, config::ConfigError> {
    let cwd = std::env::current_dir().expect("failed to determine current working directory");
    let env: Environment = std::env::var("APP_ENV")
        .unwrap_or("dev".into())
        .try_into()
        .expect("failed to determine app environment");

    let filename = format!("{}.yaml", env.as_str());

    let cfg = config::Config::builder()
        .add_source(config::File::from(cwd.join(filename)))
        .build()?;

    cfg.try_deserialize::<Config>()
}

/// Environment configuration specifying what config file should be used.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Environment {
    Dev,
    Prod,
}

impl Environment {
    /// Return a string representation of the enum value.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Dev => "dev",
            Self::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            other => Err(format!(
                "{} is not a valid environment configuration. valid values are 'prod' and 'dev'\n",
                other
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn config_from_file_when_prod() {
        env::set_var("APP_ENV", "prod");
        let _cfg = get_config().expect("error reading prod config file");
    }

    #[test]
    fn config_from_file_when_dev() {
        env::set_var("APP_ENV", "dev");
        let _cfg = get_config().expect("error reading dev config file");
    }
}
