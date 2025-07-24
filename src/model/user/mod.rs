use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct UserId(u64);

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: UserId,
    pub login: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub full_name: Option<String>,
    pub sex: Option<String>,
    pub verified: Option<bool>,
    #[serde(default)]
    pub regions: Vec<i32>,
}

impl std::fmt::Display for UserId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = std::num::ParseIntError;

    fn from_str(parsee: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Self ( parsee.parse()? ))
    }
}
