use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ClientError {
    RequestError { error: reqwest::Error },
    ParseError { error: serde_json::Error },
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClientError::RequestError { error } => Some(error),
            ClientError::ParseError { error } => Some(error),
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ClientError::RequestError { error } => write!(f, "{error}"),
            ClientError::ParseError { error } => write!(f, "{error}"),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::RequestError { error }
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        ClientError::ParseError { error }
    }
}
