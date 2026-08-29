use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct FastHealing {
    /// Foundry distinguishes "fast-healing" from "regeneration"
    pub healing_type: Option<String>,
    pub value: i64,
    /// Damage types that suppress regeneration, e.g. `["holy", "silver"]`. Empty for plain
    /// fast healing, which can't be deactivated.
    pub deactivated_by: Vec<String>,
}

/// Extracts the `"key": "FastHealing"` rule element out of an item's raw `system.rules` array.
///
/// Actions carry zero or one `FastHealing` rule element, hence `Option`.
impl FastHealing {
    pub fn init_from_json(rules_json: &[Value]) -> Option<Self> {
        rules_json
            .iter()
            .find(|rule| rule.get("key").and_then(Value::as_str) == Some("FastHealing"))
            .and_then(Self::parse_fast_healing)
    }

    fn parse_fast_healing(json: &Value) -> Option<Self> {
        // Every real instance has a static integer value; if that ever isn't true, there's nothing
        // useful to keep (same "drop rather than guess" rule structured damage extraction follows).
        let value = json.get("value").and_then(Value::as_i64)?;
        let deactivated_by_json = json_utils::get_field_from_json(json, "deactivatedBy");
        Some(Self {
            healing_type: json.get("type").and_then(Value::as_str).map(String::from),
            value,
            deactivated_by: deactivated_by_json
                .as_array()
                .map(|arr| json_utils::from_json_vec_of_str_to_vec_of_str(arr))
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_plain_fast_healing() {
        // Real Volluk Azrinae "Fast Healing 7" shape
        let rules = vec![json!({"key": "FastHealing", "type": "fast-healing", "value": 7})];
        assert_eq!(
            Some(FastHealing {
                healing_type: Some("fast-healing".to_string()),
                value: 7,
                deactivated_by: vec![],
            }),
            FastHealing::init_from_json(&rules)
        );
    }

    #[test]
    fn extracts_regeneration_with_deactivation_conditions() {
        // Real Cratonys "Regeneration 10 (Deactivated by Holy or Silver)" shape
        let rules = vec![json!({
            "deactivatedBy": ["holy", "silver"],
            "key": "FastHealing",
            "type": "regeneration",
            "value": 10
        })];
        assert_eq!(
            Some(FastHealing {
                healing_type: Some("regeneration".to_string()),
                value: 10,
                deactivated_by: vec!["holy".to_string(), "silver".to_string()],
            }),
            FastHealing::init_from_json(&rules)
        );
    }

    #[test]
    fn extracts_value_even_without_a_type_or_number_in_the_name() {
        // Real Ysondkhelir "Planar Fast Healing" shape: value only lives in the rule element,
        // not in the action name or its (pure-prose) description.
        let rules = vec![json!({
            "key": "FastHealing",
            "predicate": ["planar-fast-healing"],
            "value": 5
        })];
        assert_eq!(
            Some(FastHealing {
                healing_type: None,
                value: 5,
                deactivated_by: vec![],
            }),
            FastHealing::init_from_json(&rules)
        );
    }

    #[test]
    fn ignores_non_fast_healing_rule_elements() {
        let rules = vec![
            json!({"key": "Resistance", "type": "fire", "value": 5}),
            json!({"key": "Aura", "radius": 30}),
        ];
        assert_eq!(None, FastHealing::init_from_json(&rules));
    }

    #[test]
    fn returns_none_when_rules_array_is_empty() {
        assert_eq!(None, FastHealing::init_from_json(&[]));
    }
}
