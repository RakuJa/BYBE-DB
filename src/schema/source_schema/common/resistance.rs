use crate::schema::source_schema::rules::{Iwr, Rule};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Resistance {
    pub name: String,
    pub value: i64,
    pub double_vs: Vec<String>,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ResistanceParserError {
    #[error("Resistance value is not a valid number")]
    ValueIsNaN,
    #[error("Type field in json, containing name, could not be parsed")]
    Name,
    #[error("Element parsed is not a valid string")]
    DoubleOrExceptionElementIsNotAString,
}

impl TryFrom<&Rule> for Resistance {
    type Error = ();
    fn try_from(rule: &Rule) -> Result<Self, Self::Error> {
        if rule.key != Iwr::Resistance {
            Err(())
        } else {
            Ok(Self {
                name: rule.name.clone(),
                value: rule.value,
                double_vs: rule.double_vs.clone(),
                exceptions: rule.exceptions.clone(),
            })
        }
    }
}

impl TryFrom<&Value> for Resistance {
    type Error = ResistanceParserError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(Self {
            name: json
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .ok_or(ResistanceParserError::Name)?,
            value: json
                .get("value")
                .and_then(|x| x.as_i64())
                .ok_or(ResistanceParserError::ValueIsNaN)?,
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
