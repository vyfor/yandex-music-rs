use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub struct PlaylistId {
    pub uid: u64,
    pub kind: u32,
}
