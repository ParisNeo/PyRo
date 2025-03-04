use std::fmt;
use std::io;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum PyroError {
    Custom(String),
    Reqwest(ReqwestError),
    Io(io::Error),
    Other(String),
}

impl fmt::Display for PyroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PyroError::Custom(err_str) => write!(f, "Reqwest error: {}", err_str),
            PyroError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            PyroError::Io(err) => write!(f, "IO error: {}", err),
            PyroError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for PyroError {}

impl PyroError {
    pub fn new(message: &str) -> Self {
        PyroError::Custom(message.to_string())
    }
}

impl From<ReqwestError> for PyroError {
    fn from(err: ReqwestError) -> Self {
        PyroError::Reqwest(err)
    }
}

impl From<io::Error> for PyroError {
    fn from(err: io::Error) -> Self {
        PyroError::Io(err)
    }
}

impl From<String> for PyroError {
    fn from(msg: String) -> Self {
        PyroError::Other(msg)
    }
}