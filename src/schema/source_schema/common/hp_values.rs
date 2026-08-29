use crate::utils::json_utils::get_field_from_json;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct RawHpValues {
    pub hp: i64,
    pub hp_details: Option<String>, // items have no details
}

impl Default for RawHpValues {
    fn default() -> Self {
        Self {
            hp: -2,
            hp_details: None,
        }
    }
}

impl From<&Value> for RawHpValues {
    fn from(json: &Value) -> Self {
        let fallback_hp = get_field_from_json(json, "value");
        // `max` can be explicitly `null` (e.g. shields with "can't be damaged")
        let hp = json
            .get("max")
            .filter(|v| !v.is_null())
            .unwrap_or(&fallback_hp);
        let hp_details = json
            .get("details")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        if let Some(hp_val) = hp.as_i64() {
            RawHpValues {
                hp: hp_val,
                hp_details,
            }
        } else {
            RawHpValues {
                hp: hp.as_str().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
                hp_details,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_normal_max_and_value() {
        let hp = RawHpValues::try_from(&json!({"details": "", "max": 10, "value": 10, "temp": 0}))
            .unwrap();
        assert_eq!(10, hp.hp);
        assert_eq!(Some("".to_string()), hp.hp_details);
    }

    // "Defender's Shield (Can't Be Damaged)" carried by the Grand
    // Defender (data/packs/pf2e/lost-omens-bestiary/highhelm/grand-defender.json):
    // `max` is explicitly JSON `null`
    #[test]
    fn explicit_null_max_falls_back_to_zero_instead_of_erroring() {
        let hp = RawHpValues::try_from(&json!({"max": null, "value": null})).unwrap();
        assert_eq!(0, hp.hp);
    }

    #[test]
    fn missing_max_falls_back_to_value() {
        let hp = RawHpValues::try_from(&json!({"value": 8})).unwrap();
        assert_eq!(8, hp.hp);
    }

    #[test]
    fn missing_everything_defaults_to_zero() {
        let hp = RawHpValues::try_from(&json!({})).unwrap();
        assert_eq!(0, hp.hp);
    }
}
