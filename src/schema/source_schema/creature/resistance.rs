use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Resistance {
    pub name: String,
    pub value: i64,
    pub double_vs: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ResistanceParserError {
    #[error("Speed value is not a valid number")]
    SpeedValueIsNaN,
    #[error("Type field in json, containing name, could not be parsed")]
    ResistanceNameError,
    #[error("Element parsed is not a valid string")]
    DoubleOrExceptionElementIsNotAString,
}

impl TryFrom<&Value> for Resistance {
    type Error = ResistanceParserError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(Self {
            name: json
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .ok_or(ResistanceParserError::ResistanceNameError)?,
            value: json
                .get("value")
                .and_then(|x| x.as_i64())
                .ok_or(ResistanceParserError::SpeedValueIsNaN)?,
            double_vs: json
                .get("doubleVs")
                .and_then(|x| x.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(|x| x.to_string())
                        .ok_or(ResistanceParserError::DoubleOrExceptionElementIsNotAString)
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
                        .ok_or(ResistanceParserError::DoubleOrExceptionElementIsNotAString)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
