use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug)]
pub struct RawTraits {
    pub rarity: String,
    pub size: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Error)]
pub enum TraitParsingError {
    #[error("Rarity field could not be parsed")]
    Rarity,
    #[error("Size field could not be parsed")]
    Size,
    #[error("Traits field could not be parsed")]
    Traits,
}

impl TryFrom<&Value> for RawTraits {
    type Error = TraitParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        Ok(RawTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .map(String::from)
                .ok_or(TraitParsingError::Rarity)?,
            size: json_utils::get_field_from_json(
                &json_utils::get_field_from_json(json, "size"),
                "value",
            )
            .as_str()
            .map(String::from)
            .ok_or(TraitParsingError::Size)?,
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .ok_or(TraitParsingError::Traits)?,
            ),
        })
    }
}
