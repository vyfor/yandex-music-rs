use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub struct PlaylistId {
    pub uid: i32,
    pub kind: i32,
}
