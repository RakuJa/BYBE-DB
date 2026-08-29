use crate::schema::bybe_metadata_enum::SizeEnum;
use serde_json::Value;
use std::str::FromStr;

/// Extracts every `"key": "CreatureSize"` rule element out of an item's raw `system.rules` array.
/// These change a creature's size only while some condition holds
///
/// A single action can grant more than one (e.g. "Adjust Shape" offering Medium, Large, or
/// Huge depending on the chosen form), hence `Vec<SizeEnum>` rather than `Option<SizeEnum>`.
///
/// `value` is a literal size name for the overwhelming majority of real entries; a couple
/// resolve it dynamically at runtime from the item's own flags (a player-chosen option), dropped
pub fn extract_size_changes(rules_json: &[Value]) -> Vec<SizeEnum> {
    rules_json
        .iter()
        .filter(|rule| rule.get("key").and_then(Value::as_str) == Some("CreatureSize"))
        .filter_map(|rule| rule.get("value").and_then(Value::as_str))
        .filter_map(|value| SizeEnum::from_str(value).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_a_conditional_size_change() {
        // Waldgeist "Possess Tree" shape
        let rules = vec![json!({
            "key": "CreatureSize",
            "predicate": ["possess-tree"],
            "value": "huge"
        })];
        assert_eq!(vec![SizeEnum::Huge], extract_size_changes(&rules));
    }

    #[test]
    fn normalizes_an_abbreviated_size() {
        let rules = vec![json!({"key": "CreatureSize", "value": "grg"})];
        assert_eq!(vec![SizeEnum::Gargantuan], extract_size_changes(&rules));
    }

    #[test]
    fn drops_a_dynamic_value_reference() {
        // Viscous Black Pudding "Adjust Shape" shape: size is a player-chosen option
        // resolved at runtime from the item's own flags, not a static value.
        let rules = vec![json!({
            "key": "CreatureSize",
            "predicate": ["adjust-shape"],
            "value": "{item|flags.system.rulesSelections.adjustShape}"
        })];
        assert!(extract_size_changes(&rules).is_empty());
    }

    #[test]
    fn supports_multiple_sizes_offered_by_the_same_action() {
        let rules = vec![
            json!({"key": "CreatureSize", "value": "medium"}),
            json!({"key": "CreatureSize", "value": "large"}),
            json!({"key": "CreatureSize", "value": "huge"}),
        ];
        assert_eq!(
            vec![SizeEnum::Medium, SizeEnum::Large, SizeEnum::Huge],
            extract_size_changes(&rules)
        );
    }

    #[test]
    fn ignores_non_creature_size_rule_elements() {
        let rules = vec![json!({"key": "BaseSpeed", "selector": "fly", "value": 30})];
        assert!(extract_size_changes(&rules).is_empty());
    }
}
