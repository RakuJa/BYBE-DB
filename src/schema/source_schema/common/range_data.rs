use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct RangeData {
    pub value: String,
    pub increment: Option<String>,
    pub max: Option<String>,
}

impl Default for RangeData {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            increment: None,
            max: None,
        }
    }
}

#[derive(Debug, Error)]
pub enum RangeParsingError {
    #[error("value field missing")]
    ValueNaN,
}

impl TryFrom<&Value> for RangeData {
    type Error = RangeParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        match json {
            Value::Number(n) => {
                Ok(Self {
                    value: n.as_i64().unwrap().to_string(),
                    increment: None,
                    max: None,
                })
            }
            Value::String(s) => Ok(Self {
                value: s.to_string(),
                increment: None,
                max: None,
            }),
            Value::Object(o) => Ok(Self {
                value: "".to_string(),
                increment: o.get("increment").map(|v| v.to_string()),
                max: o.get("max").map(|v| v.to_string()),
            }),
            _ => Err(RangeParsingError::ValueNaN),
        }
    }
}
