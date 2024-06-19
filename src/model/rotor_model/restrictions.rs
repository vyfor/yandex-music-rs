use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationRestrictions {
    pub language: StationValues,
    pub diversity: StationValues,
    pub mood: Option<StationDiscreteScale>,
    pub mood_energy: Option<StationValues>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationValues {
    pub r#type: String,
    pub name: String,
    pub possible_values: Vec<StationValue>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationDiscreteScale {
    pub r#type: String,
    pub name: String,
    pub min: StationValue,
    pub max: StationValue,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationValue {
    pub value: String,
    pub name: String,
}
