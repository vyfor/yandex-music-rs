use crate::api::Endpoint;
use crate::client::request::RequestOptions;
use crate::model::album::Album;
use crate::YandexMusicClient;
use reqwest::Method;
use std::borrow::Cow;

pub struct GetAlbumsOptions {
    album_ids: Vec<i32>,
}

impl GetAlbumsOptions {
    pub fn new<I>(album_ids: I) -> Self
    where
        I: IntoIterator<Item = i32>,
    {
        Self {
            album_ids: album_ids.into_iter().collect(),
        }
    }
}

impl Endpoint for GetAlbumsOptions {
    type Options = [(&'static str, String); 1];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        "albums".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_form_data([(
            "album-ids",
            self.album_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        )])
    }
}

impl YandexMusicClient {
    /// Get a list of albums.
    ///
    /// ### Arguments
    /// * `options` - The request options containing the album IDs.
    ///
    /// ### Returns
    /// * `Result<Vec<Album>, ClientError>` - A list of albums or an error if the request fails.
    pub async fn get_albums(
        &self,
        options: &GetAlbumsOptions,
    ) -> Result<Vec<Album>, crate::ClientError> {
        self.request::<Vec<Album>, _>(options).await
    }
}
