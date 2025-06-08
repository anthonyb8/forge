use strum::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Custom Error: {0}")]
    CustomError(String),
    #[error("Inquire Error: {0}")]
    InguireError(#[from] inquire::InquireError),
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),
    #[error("Io Erro: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Toml Ser Error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Toml De Error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("Fmt Error: {0}")]
    FmtError(#[from] std::fmt::Error),
}

#[macro_export]
macro_rules! error{
    ($variant:ident, $($arg:tt)*) => {
        Error::$variant(format!($($arg)*))
    };
}

pub type Result<T> = std::result::Result<T, Error>;
