use crate::{api::RequestPath, YandexMusicClient};

pub struct PlayAudioRequest {}

impl RequestPath for PlayAudioRequest {
    fn path(&self) -> String {
        String::from("play-audio")
    }
}

impl YandexMusicClient {
    /// Send sending the current state of the track being listened to.
    ///
    /// ### Arguments
    /// * `track_id` - The ID of the track.
    /// * `album_id` - The ID of the album.
    /// * `playlist_id` - The ID of the playlist.
    /// * `play_id` - The ID of the play.
    /// * `from` - The source of the play.
    /// * `from_cache` - Whether the track is played from the cache.
    /// * `uid` - The ID of the user.
    /// * `track_length_seconds` - The length of the track in seconds.
    /// * `total_played_seconds` - The total played time in seconds.
    /// * `end_position_seconds` - The end position in seconds.
    /// * `timestamp` - The timestamp of the play.
    /// * `client_now` - The current date time in ISO format.
    ///
    /// ### Returns
    /// * [ClientError](crate::ClientError) - If the request fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn play_audio(
        &self,
        track_id: String,
        album_id: String,
        playlist_id: Option<String>,
        play_id: String,
        from: String,
        from_cache: bool,
        uid: Option<i32>,
        track_length_seconds: i32,
        total_played_seconds: i32,
        end_position_seconds: i32,
        timestamp: String,
        client_now: String,
    ) -> Result<(), crate::ClientError> {
        let mut form: Vec<(&str, String)> = Vec::with_capacity(12);
        form.push(("track_id", track_id));
        form.push(("album_id", album_id));
        form.push(("play_id", play_id));
        form.push(("from", from));
        form.push(("from_cache", from_cache.to_string()));
        form.push(("track_length_seconds", track_length_seconds.to_string()));
        form.push(("total_played_seconds", total_played_seconds.to_string()));
        form.push(("end_position_seconds", end_position_seconds.to_string()));
        form.push(("timestamp", timestamp));
        form.push(("client_now", client_now));
        if let Some(playlist_id) = playlist_id {
            form.push(("playlist_id", playlist_id));
        }
        if let Some(uid) = uid {
            form.push(("uid", uid.to_string()));
        }

        self.post_with_form(&PlayAudioRequest {}.path(), form)
            .await?;

        Ok(())
    }
}
