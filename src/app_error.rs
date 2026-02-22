use std::fmt;
use std::error::Error;
use serde_json::Error as serde_error;

#[derive(Debug)]
pub enum AppError {
    LoadPlayersFile(std::io::Error),
    LoadPlayersDeserialize(serde_error),
    ErrorWrittingFile,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::LoadPlayersFile(err) => write!(f, "IO error: {}", err),
            AppError::LoadPlayersDeserialize(err) => write!(f, "JSON parse error: {}", err),
            AppError::ErrorWrittingFile => write!(f, "Error!"),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::LoadPlayersFile(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> AppError {
        AppError::LoadPlayersDeserialize(err)
    }
}

