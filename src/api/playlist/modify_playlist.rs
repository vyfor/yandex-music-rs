use std::borrow::Cow;

use reqwest::Method;
use serde_json::json;

use crate::{
    api::Endpoint,
    client::request::RequestOptions,
    model::playlist::{modify::Diff, Playlist},
    YandexMusicClient,
};

/// Request for modifying a playlist by adding or removing tracks.
pub struct ModifyPlaylistOptions {
    /// The ID of the user who owns the playlist.
    pub user_id: u64,
    /// The kind (ID) of the playlist to modify.
    pub kind: u32,
    /// The diff object containing the changes to apply.
    pub diff: Diff,
    /// The current revision of the playlist.
    pub revision: u32,
}

impl ModifyPlaylistOptions {
    /// Create a new request to modify a playlist.
    pub fn new(user_id: u64, kind: u32, diff: Diff, revision: u32) -> Self {
        Self {
            user_id,
            kind,
            diff,
            revision,
        }
    }
}

impl Endpoint for ModifyPlaylistOptions {
    type Options = [(&'static str, String); 2];
    const METHOD: Method = Method::POST;

    fn path(&self) -> Cow<'static, str> {
        format!(
            "users/{}/playlists/{}/change-relative",
            self.user_id, self.kind
        )
        .into()
    }

    fn options(&self) -> RequestOptions<Self::Options> {
        let diff_str = json!({
            "diff": self.diff,
        })
        .to_string();

        RequestOptions::default().with_form_data([
            ("diff", diff_str),
            ("revision", self.revision.to_string()),
        ])
    }
}

impl YandexMusicClient {
    /// Modify a playlist by adding or removing tracks using a diff object.
    ///
    /// ### Arguments
    /// * `options` - The request options containing user ID, playlist kind, diff, and revision.
    ///
    /// ### Returns
    /// * `Result<Playlist, ClientError>` - The updated playlist or an error if the request fails.
    pub async fn modify_playlist(
        &self,
        options: &ModifyPlaylistOptions,
    ) -> Result<Playlist, crate::ClientError> {
        self.request(options).await
    }
}
