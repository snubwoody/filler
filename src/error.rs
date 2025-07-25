use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

// TODO change into a struct
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Invalid path")]
    InvalidPath,
}
