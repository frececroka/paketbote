use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Input/output error")]
    Io(#[from] std::io::Error),

    #[error("Database error")]
    Diesel(#[from] diesel::result::Error),

    #[error("Serde error")]
    Serde(#[from] serde_json::error::Error),

    #[error("Encoding error")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Parse error")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Alpm error")]
    Alpm(#[from] alpm::Error),

    #[error("{0}")]
    Generic(String)
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        message.to_owned().into()
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Error::Generic(message)
    }
}
