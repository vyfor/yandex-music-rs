use serde::Deserialize;
use crate::UserId;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
pub struct PlaylistId {
    pub uid: UserId,
    pub kind: i32,
}
