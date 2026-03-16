use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct RawSaves {
    pub fortitude: Option<i64>,
    pub fortitude_detail: String,
    pub reflex: Option<i64>,
    pub reflex_detail: String,
    pub will: Option<i64>,
    pub will_detail: String,
}

#[derive(Debug, Error)]
pub enum SaveParsingError {
    #[error("Fortitude save is NaN")]
    Fortitude,
    #[error("Fortitude detail field could not be parsed")]
    FortitudeDetail,
    #[error("Reflex save is NaN")]
    Reflex,
    #[error("Reflex detail field could not be parsed")]
    ReflexDetail,
    #[error("Will save is NaN")]
    Will,
    #[error("Will detail field could not be parsed")]
    WillDetail,
}

impl TryFrom<&Value> for RawSaves {
    type Error = SaveParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let parse_save = |key: &str, val_err: SaveParsingError, detail_err: SaveParsingError| {
            let json = json_utils::get_field_from_json(json, key);
            let value = {
                let v = json_utils::get_field_from_json(&json, "value");
                if v.is_null() {
                    Ok(None)
                } else {
                    v.as_i64().map(Some).ok_or(val_err)
                }
            };
            let detail = json
                .get("saveDetail")
                .and_then(|x| x.as_str())
                .map(String::from)
                .ok_or(detail_err);
            if let Err(e) = &detail {
                warn!("{key} detail field could not be parsed: {}", e);
            }
            Ok((value?, detail.unwrap_or_default()))
        };

        let (fortitude, fortitude_detail) = parse_save(
            "fortitude",
            SaveParsingError::Fortitude,
            SaveParsingError::FortitudeDetail,
        )?;
        let (reflex, reflex_detail) = parse_save(
            "reflex",
            SaveParsingError::Reflex,
            SaveParsingError::ReflexDetail,
        )?;
        let (will, will_detail) =
            parse_save("will", SaveParsingError::Will, SaveParsingError::WillDetail)?;

        Ok(RawSaves {
            fortitude,
            fortitude_detail,
            reflex,
            reflex_detail,
            will,
            will_detail,
        })
    }
}
