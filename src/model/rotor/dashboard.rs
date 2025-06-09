use super::Rotor;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationsDashboard {
    pub dashboard_id: String,
    pub pumpkin: bool,
    pub stations: Vec<Rotor>,
}
