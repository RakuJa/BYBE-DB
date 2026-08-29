use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MovementData {
    /// Normalized movement type ("fly", "land", "climb", "swim", "burrow"). Foundry data
    /// inconsistently suffixes this as e.g. both "fly" and "fly-speed" for the same thing.
    pub movement_type: String,
    pub value: i64,
}

/// Extracts every `"key": "BaseSpeed"` rule element out of an item's raw `system.rules` array,
/// These grant a creature a movement speed only while some condition holds
///
///
/// `value` is a plain integer for the overwhelming majority of real entries; a handful resolve
/// it dynamically at runtime (e.g. `max(20,@actor.system.movement.speeds.land.value)`), currently dropped
pub fn extract_movement_changes(rules_json: &[Value]) -> Vec<MovementData> {
    rules_json
        .iter()
        .filter(|rule| rule.get("key").and_then(Value::as_str) == Some("BaseSpeed"))
        .filter_map(parse_movement_change)
        .collect()
}

fn parse_movement_change(json: &Value) -> Option<MovementData> {
    let movement_type = json
        .get("selector")
        .and_then(Value::as_str)?
        .trim_end_matches("-speed")
        .to_string();
    let value = json.get("value").and_then(Value::as_i64)?;
    Some(MovementData {
        movement_type,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_a_conditional_fly_speed() {
        // Shadowforged Guardian "Twilight Aura" shape
        let rules = vec![json!({
            "key": "BaseSpeed",
            "predicate": ["lighting:dim-light"],
            "selector": "fly",
            "value": 30
        })];
        assert_eq!(
            vec![MovementData {
                movement_type: "fly".to_string(),
                value: 30,
            }],
            extract_movement_changes(&rules)
        );
    }

    #[test]
    fn normalizes_the_speed_suffixed_selector() {
        // Mummy Prophet of Set "Sand Form" shape
        let rules = vec![json!({
            "key": "BaseSpeed",
            "predicate": ["sand-form"],
            "selector": "fly-speed",
            "value": 40
        })];
        let changes = extract_movement_changes(&rules);
        assert_eq!("fly", changes[0].movement_type);
        assert_eq!(40, changes[0].value);
    }

    #[test]
    fn drops_a_dynamic_value_formula() {
        // "Spell Effect: Fly" shape: speed depends on the actor's own land speed
        let rules = vec![json!({
            "key": "BaseSpeed",
            "selector": "fly",
            "value": "max(20,@actor.system.movement.speeds.land.value)"
        })];
        assert!(extract_movement_changes(&rules).is_empty());
    }

    #[test]
    fn supports_multiple_changes_granted_by_the_same_action() {
        let rules = vec![
            json!({"key": "BaseSpeed", "selector": "climb", "value": 40}),
            json!({"key": "BaseSpeed", "selector": "swim", "value": 20}),
        ];
        assert_eq!(2, extract_movement_changes(&rules).len());
    }

    #[test]
    fn ignores_non_base_speed_rule_elements() {
        let rules = vec![json!({"key": "Aura", "radius": 30})];
        assert!(extract_movement_changes(&rules).is_empty());
    }
}
