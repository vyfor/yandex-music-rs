use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::SystemTime;

pub const SIGN_KEY: &str = "p93jhgh689SBReK6ghtw62";

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

pub fn create_file_info_sign(
    track_id: &str,
    quality: &str,
    codec: &str,
    transport: &str,
) -> (String, String) {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let sign_str = format!("{}{}{}{}{}", timestamp, track_id, quality, codec, transport);

    let mut mac = Hmac::<Sha256>::new_from_slice(SIGN_KEY.as_bytes()).unwrap();
    mac.update(sign_str.as_bytes());
    let result = mac.finalize();
    let sign = STANDARD.encode(result.into_bytes());
    let sign = sign[..sign.len() - 1].to_string();

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
