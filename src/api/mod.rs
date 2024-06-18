use serde::Deserialize;
use serde_json::Value;

pub mod album;
pub mod playlist;
pub mod track;
pub mod utils;

pub trait RequestPath {
    fn path(&self) -> String;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvocationInfo {
    #[serde(deserialize_with = "deseralize_exec_duration_millis")]
    pub exec_duration_millis: Option<i64>,
    pub hostname: String,
    pub req_id: String,
}

fn deseralize_exec_duration_millis<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    let millis = match value {
        Some(Value::String(s)) => s.parse::<i64>().ok(),
        Some(Value::Number(n)) => n.as_i64(),
        _ => None,
    };
    Ok(millis)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub invocation_info: InvocationInfo,
    pub result: Value,
}
