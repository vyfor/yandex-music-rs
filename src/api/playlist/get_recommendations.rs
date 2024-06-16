use crate::api::RequestPath;

pub struct RecommendationsPlaylistRequest {
    pub user_id: i32,
    pub kind: i32,
}

impl RecommendationsPlaylistRequest {
    pub fn new(user_id: i32, kind: i32) -> Self {
        Self { user_id, kind }
    }
}

impl RequestPath for RecommendationsPlaylistRequest {
    fn path(&self) -> String {
        format!(
            "/users/{}/playlists/{}/recommendations",
            self.user_id, self.kind
        )
    }
}
