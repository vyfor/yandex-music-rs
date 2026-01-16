use serde::Serialize;
use serde_json::Value;

use crate::{
    api::Endpoint, client::request::RequestOptions, model::collection::Collection,
    YandexMusicClient,
};

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSyncOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_albums: Option<CollectionSyncOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_artists: Option<CollectionSyncOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_clips: Option<CollectionSyncOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_playlists: Option<CollectionSyncOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_tracks: Option<CollectionSyncOption>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSyncOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_values_required: Option<bool>,
}

impl CollectionSyncOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all() -> Self {
        Self {
            liked_albums: Some(CollectionSyncOption::new()),
            liked_artists: Some(CollectionSyncOption::new()),
            liked_clips: Some(CollectionSyncOption::new()),
            liked_playlists: Some(CollectionSyncOption::new()),
            liked_tracks: Some(CollectionSyncOption::new()),
        }
    }

    pub fn liked_albums(mut self, option: CollectionSyncOption) -> Self {
        self.liked_albums = Some(option);
        self
    }

    pub fn liked_artists(mut self, option: CollectionSyncOption) -> Self {
        self.liked_artists = Some(option);
        self
    }

    pub fn liked_clips(mut self, option: CollectionSyncOption) -> Self {
        self.liked_clips = Some(option);
        self
    }

    pub fn liked_playlists(mut self, option: CollectionSyncOption) -> Self {
        self.liked_playlists = Some(option);
        self
    }

    pub fn liked_tracks(mut self, option: CollectionSyncOption) -> Self {
        self.liked_tracks = Some(option);
        self
    }
}

impl CollectionSyncOption {
    pub fn new() -> Self {
        Self {
            revision: None,
            all_values_required: None,
        }
    }

    pub fn revision(mut self, revision: u64) -> Self {
        self.revision = Some(revision);
        self
    }

    pub fn all_values_required(mut self, all_values_required: bool) -> Self {
        self.all_values_required = Some(all_values_required);
        self
    }
}

impl Endpoint for CollectionSyncOptions {
    type Options = ();
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "collection/sync".into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        RequestOptions::default().with_json_data(serde_json::to_value(self).unwrap())
    }
}

impl YandexMusicClient {
    /// Syncs user's collection.
    ///
    /// ### Arguments
    /// * `options` - The request options.
    ///
    /// ### Returns
    /// * `Result<Collection, ClientError>` - The synced collection or an error if the request fails.
    pub async fn collection_sync(
        &self,
        options: &CollectionSyncOptions,
    ) -> Result<Collection, crate::ClientError> {
        let mut response = self.request::<Value, _>(options).await?;

        Ok(serde_json::from_value::<Collection>(
            response["values"].take(),
        )?)
    }
}
