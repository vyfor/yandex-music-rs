use crate::api::RequestPath;

pub struct SimilarTracksRequest {
    pub track_id: i32,
}

impl SimilarTracksRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for SimilarTracksRequest {
    fn path(&self) -> String {
        format!("tracks/{}/similar", self.track_id)
    }
}
