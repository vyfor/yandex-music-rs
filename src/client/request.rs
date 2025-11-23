use std::borrow::Cow;
use std::marker::PhantomData;

use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{
    api::{Endpoint, Response},
    error::{ClientError, YandexMusicError},
    API_PATH,
};

/// Request body, either form data or JSON data
pub enum RequestBody<T = ()> {
    Form(T),
    Json(Value),
    Empty,
}

/// Request options for API calls
pub struct RequestOptions<T> {
    /// Additional headers to include with the request
    pub headers: Option<HeaderMap>,
    /// Body data for POST requests
    pub body: RequestBody<T>,
}

impl<T> Default for RequestOptions<T> {
    fn default() -> Self {
        Self {
            headers: None,
            body: RequestBody::Empty,
        }
    }
}

impl<T> RequestOptions<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn with_form_data(mut self, form_data: T) -> Self {
        self.body = RequestBody::Form(form_data);
        self
    }

    pub fn with_json_data(mut self, json_data: Value) -> Self {
        self.body = RequestBody::Json(json_data);
        self
    }
}

/// Generic form parameter builder for API requests.
pub struct FormParams<T> {
    params: Vec<(Cow<'static, str>, T)>,
    _marker: PhantomData<T>,
}

impl<T> FormParams<T> {
    /// Create a new empty form parameter collection.
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// Add a parameter to the collection.
    pub fn param<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<T>,
    {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Add an optional parameter to the collection.
    pub fn opt_param<K, V>(mut self, key: K, value: Option<V>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<T>,
    {
        if let Some(v) = value {
            self.params.push((key.into(), v.into()));
        }
        self
    }

    /// Build the parameters into a vector.
    pub fn build(self) -> Vec<(Cow<'static, str>, T)> {
        self.params
    }
}

impl<T> Default for FormParams<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper trait for converting types to API parameters.
pub trait IntoApiParam<T> {
    fn into_param(self) -> T;
}

/// Helper functions for sending API requests
pub(crate) async fn send_request<P, T>(
    client: &reqwest::Client,
    endpoint: &P,
    url: Option<String>,
) -> Result<T, ClientError>
where
    P: Endpoint,
    T: DeserializeOwned,
    <P as Endpoint>::Options: Serialize,
{
    let method = <P as Endpoint>::METHOD;
    let options = endpoint.options();
    let url = if let Some(url) = url {
        url
    } else {
        format!("{}{}", API_PATH, endpoint.path())
    };
    let mut request_builder = client.request(method, url);

    if let Some(headers) = options.headers {
        request_builder = request_builder.headers(headers);
    }

    match options.body {
        RequestBody::Form(form) => {
            request_builder = request_builder.form(&form);
        }
        RequestBody::Json(json) => {
            request_builder = request_builder.json(&json);
        }
        RequestBody::Empty => {}
    }

    let response = request_builder.send().await?;
    let status_code = response.status();

    if !status_code.is_success() {
        if let Ok(res) = response.json::<Response>().await {
            if let Some(err) = res.error {
                return Err(err.into());
            }
        }

        return Err(ClientError::YandexMusicError {
            error: YandexMusicError {
                name: "RequestFailed".to_string(),
                message: Some(format!("Request failed with status code: {status_code}")),
            },
        });
    }

    let response: Response = response.json().await?;

    if let Some(error) = response.error {
        return Err(error.into());
    }

    let result = response
        .result
        .ok_or_else(|| ClientError::YandexMusicError {
            error: YandexMusicError {
                name: "MissingResult".to_string(),
                message: Some("API response contains no result".to_string()),
            },
        })?;

    Ok(serde_json::from_value(result)?)
}

/// Helper function for sending API requests directly parsing the response into the desired type.
pub(crate) async fn send_request_direct<P, T>(
    client: &reqwest::Client,
    endpoint: &P,
    url: Option<String>,
) -> Result<T, ClientError>
where
    P: Endpoint,
    T: DeserializeOwned,
    <P as Endpoint>::Options: Serialize,
{
    let method = <P as Endpoint>::METHOD;
    let options = endpoint.options();

    let url = if let Some(url) = url {
        url
    } else {
        format!("{}{}", API_PATH, endpoint.path())
    };

    let mut request_builder = client.request(method, url);

    if let Some(headers) = options.headers {
        request_builder = request_builder.headers(headers);
    }

    match options.body {
        RequestBody::Form(form) => {
            request_builder = request_builder.form(&form);
        }
        RequestBody::Json(json) => {
            request_builder = request_builder.json(&json);
        }
        RequestBody::Empty => {}
    }

    let response = request_builder.send().await?;
    let status_code = response.status();

    if !status_code.is_success() {
        return Err(ClientError::YandexMusicError {
            error: YandexMusicError {
                name: "RequestFailed".to_string(),
                message: Some(format!("Request failed with status code: {status_code}")),
            },
        });
    }

    Ok(response.json::<T>().await?)
}
