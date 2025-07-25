use serde::Serialize;

use crate::model::track::TrackShort;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Diff {
    #[serde(flatten)]
    pub op: DiffOp,
    pub tracks: Vec<TrackShort>,
}

impl Diff {
    pub fn new(op: DiffOp, tracks: Vec<TrackShort>) -> Self {
        Self { op, tracks }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum DiffOp {
    Insert { at: usize },
    Delete { from: usize, to: usize },
}

impl DiffOp {
    pub fn insert(at: usize) -> Self {
        Self::Insert { at }
    }

    pub fn delete(from: usize, to: usize) -> Self {
        Self::Delete { from, to }
    }
}
