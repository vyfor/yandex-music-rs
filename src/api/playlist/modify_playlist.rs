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
pub struct ModifyPlaylistOptions<'a> {
    /// The ID of the user who owns the playlist.
    pub user_id: i32,
    /// The kind (ID) of the playlist to modify.
    pub kind: i32,
    /// The diff object containing the changes to apply.
    pub diff: &'a Diff,
    /// The current revision of the playlist.
    pub revision: i32,
}

impl<'a> ModifyPlaylistOptions<'a> {
    /// Create a new request to modify a playlist.
    pub fn new(user_id: i32, kind: i32, diff: &'a Diff, revision: i32) -> Self {
        Self {
            user_id,
            kind,
            diff,
            revision,
        }
    }
}

impl<'a> Endpoint for ModifyPlaylistOptions<'a> {
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

        RequestOptions::default()
            .with_form_data([("diff", diff_str), ("revision", self.revision.to_string())])
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
        options: &ModifyPlaylistOptions<'_>,
    ) -> Result<Playlist, crate::ClientError> {
        self.request::<Playlist, _>(options).await
    }
}
