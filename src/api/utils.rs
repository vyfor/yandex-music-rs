use base64::engine::general_purpose::STANDARD;
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
    let sign = url_encode(STANDARD.encode(hmac.into_bytes()));

    (timestamp, sign)
}

fn url_encode(input: String) -> String {
    let mut output = String::with_capacity(input.len() * 3);
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    for b in input.into_bytes() {
        match b {
            b'-' | b'_' | b'.' | b'~' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                output.push(b as char)
            }
            _ => {
                output.push('%');
                output.push(HEX[(b >> 4) as usize] as char);
                output.push(HEX[(b & 0x0F) as usize] as char);
            }
        }
    }

    output
}
