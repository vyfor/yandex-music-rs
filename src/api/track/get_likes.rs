use crate::api::RequestPath;

pub struct LikesPlaylistRequest {
    pub user_id: i32,
}

impl LikesPlaylistRequest {
    pub fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

impl RequestPath for LikesPlaylistRequest {
    fn path(&self) -> String {
        format!("/users/{}/likes/tracks", self.user_id)
    }
}
