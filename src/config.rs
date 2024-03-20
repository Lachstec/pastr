use std::convert::TryFrom;

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
