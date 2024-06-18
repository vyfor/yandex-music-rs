use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackLyrics {
  pub id: i32,
  pub lyrics: String,
  pub full_lyrics: String,
  pub has_rights: bool,
  pub text_language: String,
  pub show_translations: bool,
}