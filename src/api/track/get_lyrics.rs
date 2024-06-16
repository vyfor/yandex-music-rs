use crate::api::{utils::add_param, RequestPath};

pub struct LyricsRequest {
    pub track_id: i32,
    pub format: Option<String>,
    pub timestamp: Option<String>,
    pub sign: Option<String>,
}

impl LyricsRequest {
    pub fn new(track_id: i32) -> Self {
        Self {
            track_id,
            format: None,
            timestamp: None,
            sign: None,
        }
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    pub fn timestamp(mut self, timestamp: String) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn sign(mut self, sign: String) -> Self {
        self.sign = Some(sign);
        self
    }
}

impl RequestPath for LyricsRequest {
    fn path(&self) -> String {
        let mut base_path = format!("tracks/{}/lyrics?", self.track_id);

        add_param(&mut base_path, "format", &self.format);
        add_param(&mut base_path, "timestamp", &self.timestamp);
        add_param(&mut base_path, "sign", &self.sign);

        if base_path.ends_with('?') {
            base_path.pop();
        }

        base_path
    }
}
