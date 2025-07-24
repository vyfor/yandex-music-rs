use error::ClientError;

use crate::client::builder::ClientBuilder;

pub mod api;
pub mod client;
pub mod error;
pub mod model;

pub const API_PATH: &str = "https://api.music.yandex.net:443/";
pub const DEFAULT_CLIENT_ID: &str = "YandexMusicAndroid/24023621";

use crate::model::user::UserId;

/// A client to interact with the Yandex Music API.
pub struct YandexMusicClient {
    /// Internal reqwest client.
    pub inner: reqwest::Client,
}

impl YandexMusicClient {
    /// Create a client builder.
    pub fn builder<'a>(token: &'a str) -> ClientBuilder<'a> {
        ClientBuilder::new(token)
    }

    /// Create a client from a reqwest client.
    /// This will override all other options. You will have to provide all the headers yourself.
    pub fn from_client(client: reqwest::Client) -> Self {
        Self { inner: client }
    }
}
