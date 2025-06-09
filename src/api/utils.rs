use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::SystemTime;

const SIGN_KEY: &str = "p93jhgh689SBReK6ghtw62";

pub fn create_sign(track_id: &str) -> (u64, String) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut mac = Hmac::<Sha256>::new_from_slice(SIGN_KEY.as_bytes()).unwrap();
    let data = format!("{track_id}{timestamp}");
    mac.update(data.as_bytes());
    let hmac = mac.finalize();
    let sign = base64::engine::general_purpose::STANDARD.encode(hmac.into_bytes());

    (timestamp, sign)
}
