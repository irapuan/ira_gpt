use std::fmt;
use std::error::Error;
use serde_json::Error as serde_error;
use good_lp::ResolutionError;

#[derive(Debug)]
pub enum AppError {
    LoadPlayersFile(std::io::Error),
    LoadPlayersDeserialize(serde_error),
    Infeasible(ResolutionError),
    SelectionSetup(String),
    SelectionAborted,
    SelectionEmpty,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::LoadPlayersFile(err) => write!(f, "IO error: {}", err),
            AppError::LoadPlayersDeserialize(err) => write!(f, "JSON parse error: {}", err),
            AppError::Infeasible(err) => write!(f, "Infeasible: {}", err),
            AppError::SelectionSetup(err) => write!(f, "Selection setup error: {}", err),
            AppError::SelectionAborted => write!(f, "Selection aborted by user"),
            AppError::SelectionEmpty => write!(f, "No players selected"),
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
