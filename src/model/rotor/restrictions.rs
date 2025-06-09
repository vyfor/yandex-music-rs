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
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub possible_values: Vec<StationValue>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationDiscreteScale {
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: String,
    pub min: StationValue,
    pub max: StationValue,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationValue {
    pub name: String,
    #[serde(deserialize_with = "crate::model::utils::number_to_string")]
    pub value: String,
}
