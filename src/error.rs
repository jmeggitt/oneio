use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum OneIoError {
    #[cfg(feature = "remote")]
    Reqwest(reqwest::Error),
    IoError(std::io::Error),
    Unsupported(String),
    Cache(String),
}

impl Display for OneIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "remote")]
            OneIoError::Reqwest(e) => e.fmt(f),
            OneIoError::IoError(e) => e.fmt(f),
            OneIoError::Unsupported(msg) => write!(f, "unsupported: {}", msg),
            OneIoError::Cache(msg) => write!(f, "cache error: {}", msg),
        }
    }
}

impl Error for OneIoError {}

#[cfg(feature = "remote")]
impl From<reqwest::Error> for OneIoError {
    fn from(error: reqwest::Error) -> Self {
        OneIoError::Reqwest(error)
    }
}

impl From<std::io::Error> for OneIoError {
    fn from(io_error: std::io::Error) -> Self {
        OneIoError::IoError(io_error)
    }
}
