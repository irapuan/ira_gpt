use std::fmt;
use std::error::Error;
use serde_json::Error as serde_error;
use good_lp::ResolutionError;
use dialoguer::Error as dialoguer_error;

#[derive(Debug)]
pub enum AppError {
    LoadPlayersFile(std::io::Error),
    LoadPlayersDeserialize(serde_error),
    Infeasible(ResolutionError),
    Other(dialoguer_error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::LoadPlayersFile(err) => write!(f, "IO error: {}", err),
            AppError::LoadPlayersDeserialize(err) => write!(f, "JSON parse error: {}", err),
            AppError::Infeasible(err) => write!(f, "Infeasible: {}", err),
            AppError::Other(err) => write!(f, "Dialogue error: {}", err),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::LoadPlayersFile(err)
    }
}

impl From<ResolutionError> for AppError {
    fn from(err: ResolutionError) -> Self {
        AppError::Infeasible(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::LoadPlayersDeserialize(err)
    }
}

impl From<dialoguer_error> for AppError {
    fn from(err: dialoguer_error) -> Self {
        AppError::Other(err)
    }
}
