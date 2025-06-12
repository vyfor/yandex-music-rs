use std::borrow::Cow;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use crate::{error::ClientError, YandexMusicClient, DEFAULT_CLIENT_ID};

/// Configuration for creating a YandexMusicClient.
pub struct ClientBuilder<'a> {
    /// OAuth token for authentication
    token: Cow<'a, str>,
    /// Custom client identifier
    client_id: Cow<'a, str>,
    /// Optional proxy configuration
    proxy: Option<reqwest::Proxy>,
    /// Custom reqwest client (overrides other options if provided)
    custom_client: Option<reqwest::Client>,
}

impl<'a> ClientBuilder<'a> {
    /// Create new client builder with default values.
    pub fn new(token: impl Into<Cow<'a, str>>) -> Self {
        Self {
            token: token.into(),
            client_id: Cow::Borrowed(DEFAULT_CLIENT_ID),
            proxy: None,
            custom_client: None,
        }
    }

    /// Set a custom client identifier.
    pub fn client_id(mut self, client_id: impl Into<Cow<'a, str>>) -> Self {
        self.client_id = client_id.into();
        self
    }

    /// Set a proxy for the client.
    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    /// Use a custom reqwest client instead of building one.
    /// This will override all other options. You will have to provide all the headers yourself.
    pub fn custom_client(mut self, client: reqwest::Client) -> Self {
        self.custom_client = Some(client);
        self
    }

    /// Build the YandexMusicClient.
    pub fn build(self) -> Result<YandexMusicClient, ClientError> {
        if let Some(client) = self.custom_client {
            return Ok(YandexMusicClient::from_client(client));
        }

        let mut headers = HeaderMap::with_capacity(2);

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("OAuth {}", self.token))?,
        );

        headers.insert(
            "X-Yandex-Music-Client",
            HeaderValue::from_str(self.client_id.as_ref())?,
        );

        let mut builder = reqwest::Client::builder().default_headers(headers);

        if let Some(proxy) = self.proxy {
            builder = builder.proxy(proxy);
        }

        let client = builder.build()?;

        Ok(YandexMusicClient::from_client(client))
    }
}
