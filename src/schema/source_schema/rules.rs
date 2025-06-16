use serde_json::Value;
use thiserror::Error;
#[derive(PartialEq, Debug, Clone)]
pub enum Iwr {
    Immunity,
    Weakness,
    Resistance,
}

impl TryFrom<String> for Iwr {
    type Error = RuleParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<Option<&str>> for Iwr {
    type Error = RuleParseError;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        Self::try_from(value.unwrap_or(""))
    }
}

impl TryFrom<&str> for Iwr {
    type Error = RuleParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "immunity" => Ok(Iwr::Immunity),
            "weakness" => Ok(Iwr::Weakness),
            "resistance" => Ok(Iwr::Resistance),
            _ => Err(RuleParseError::UnsupportedKeyValue),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub key: Iwr,
    pub name: String,
    pub value: i64,
    pub exceptions: Vec<String>,
    pub double_vs: Vec<String>,
}

#[derive(Debug, Error)]
pub enum RuleParseError {
    #[error("Iwr value is not a valid number")]
    ValueIsNaN,
    #[error("Value field is missing")]
    ValueIsMissing,
    #[error("Type field in json, containing name, could not be parsed")]
    Name,
    #[error("Element parsed is not a valid string")]
    DoubleOrExceptionElementIsNotAString,
    #[error("Key value is not a valid string (Resistance, Immunity, Weakness)")]
    KeyValueNotAString,
    #[error("Key value is not a one of the supported values")]
    UnsupportedKeyValue,
}

impl TryFrom<&Value> for Rule {
    type Error = RuleParseError;

    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let value = json.get("value").ok_or(RuleParseError::ValueIsMissing)?;
        Ok(Rule {
            key: json
                .get("key")
                .and_then(Value::as_str)
                .ok_or(RuleParseError::KeyValueNotAString)
                .and_then(Iwr::try_from)?,
            name: json
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .ok_or(RuleParseError::Name)?,
            value: value
                .as_i64()
                .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
                .ok_or(RuleParseError::ValueIsNaN)?,
            double_vs: json
                .get("doubleVs")
                .and_then(|x| x.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(|x| x.to_string())
                        .ok_or(RuleParseError::DoubleOrExceptionElementIsNotAString)
                })
                .collect::<Result<Vec<_>, _>>()?,
            exceptions: json
                .get("exceptions")
                .and_then(|x| x.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(|x| x.to_string())
                        .ok_or(RuleParseError::DoubleOrExceptionElementIsNotAString)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
