use crate::api::RequestPath;

pub struct DislikesPlaylistRequest {
    pub user_id: i32,
}

impl DislikesPlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for DislikesPlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/dislikes/tracks", self.user_id)
    }
}
