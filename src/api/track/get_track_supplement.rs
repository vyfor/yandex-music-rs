use crate::api::RequestPath;

pub struct TrackSupplementRequest {
    pub track_id: i32,
}

impl TrackSupplementRequest {
    pub fn new(track_id: i32) -> Self {
        Self { track_id }
    }
}

impl RequestPath for TrackSupplementRequest {
    fn path(&self) -> String {
        format!("tracks/{}/supplement", self.track_id)
    }
}
