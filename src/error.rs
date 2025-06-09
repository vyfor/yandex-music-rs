use std::{error::Error, fmt::Display};

use serde::Deserialize;
use serde_json::Value;

/// A generic client error.
#[derive(Debug)]
pub enum ClientError {
    /// Error during HTTP request
    RequestError {
        error: reqwest::Error,
    },
    /// Error parsing JSON response
    JsonParseError {
        error: serde_json::Error,
    },
    /// Error parsing XML response
    XmlParseError {
        error: serde_xml_rs::Error,
    },
    /// Invalid header value
    InvalidHeader {
        error: reqwest::header::InvalidHeaderValue,
    },
    /// An error returned by the Yandex Music API
    YandexMusicError {
        error: YandexMusicError,
    },
}

/// An error returned by the Yandex Music API.
#[derive(Debug, Deserialize)]
pub struct YandexMusicError {
    pub name: String,
    pub message: Option<String>,
}

impl Error for YandexMusicError {}

impl Display for YandexMusicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}. message: {:?}", self.name, self.message)
    }
}

impl From<Value> for YandexMusicError {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ClientError::RequestError { error } => Some(error),
            ClientError::JsonParseError { error } => Some(error),
            ClientError::XmlParseError { error } => Some(error),
            ClientError::InvalidHeader { error } => Some(error),
            ClientError::YandexMusicError { error } => Some(error),
        }
    }
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ClientError::RequestError { error } => write!(f, "{error}"),
            ClientError::JsonParseError { error } => write!(f, "{error}"),
            ClientError::XmlParseError { error } => write!(f, "{error}"),
            ClientError::InvalidHeader { error } => write!(f, "{error}"),
            ClientError::YandexMusicError { error } => write!(f, "{error}"),
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
        ClientError::JsonParseError { error }
    }
}

impl From<serde_xml_rs::Error> for ClientError {
    fn from(error: serde_xml_rs::Error) -> Self {
        ClientError::XmlParseError { error }
    }
}

impl From<reqwest::header::InvalidHeaderValue> for ClientError {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        ClientError::InvalidHeader { error }
    }
}

impl From<YandexMusicError> for ClientError {
    fn from(error: YandexMusicError) -> Self {
        ClientError::YandexMusicError { error }
    }
}
