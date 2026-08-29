use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct GrantedStrike {
    pub slug: Option<String>,
    pub label: Option<String>,
    pub attack_modifier: Option<i64>,
    /// Weapon proficiency category (unarmed/simple/martial/advanced); absent should be unarmed
    pub category: Option<String>,
    pub group: Option<String>,
    pub traits: Vec<String>,
    pub dmg_type: Option<String>,
    pub n_of_dice: Option<i64>,
    pub die_size: Option<i64>,
    pub bonus_dmg: i64,
    pub range_increment: Option<i64>,
    pub max_range: Option<i64>,
}

/// Extracts every `"key": "Strike"` rule element out of an item's raw `system.rules` array.
/// These grant a creature an attack that's entirely separate from its regular weapon items.
/// (e.g. a werecreature's "Change Shape" granting a fist Strike) or
/// a stance's alternate attack (e.g. a "Red Ruin Stance" granting a claw Strike).
///
/// A single action can grant more than one Strike (e.g. a full "Change Shape" granting fist,
/// claw and jaws for different forms), hence `Vec`.
pub fn extract_granted_strikes(
    rules_json: &[Value],
    resolve_label: impl Fn(&str) -> Option<String>,
) -> Vec<GrantedStrike> {
    rules_json
        .iter()
        .filter(|rule| rule.get("key").and_then(Value::as_str) == Some("Strike"))
        .filter_map(|rule| parse_granted_strike(rule, &resolve_label))
        .collect()
}

fn parse_granted_strike(
    json: &Value,
    resolve_label: &impl Fn(&str) -> Option<String>,
) -> Option<GrantedStrike> {
    let base = json.get("damage").and_then(|d| d.get("base"))?;
    let traits_json = json_utils::get_field_from_json(json, "traits");
    let range_json = json.get("range");

    Some(GrantedStrike {
        slug: json.get("slug").and_then(Value::as_str).map(String::from),
        label: json
            .get("label")
            .and_then(Value::as_str)
            .map(|raw| resolve_label(raw).unwrap_or_else(|| raw.to_string())),
        attack_modifier: json.get("attackModifier").and_then(Value::as_i64),
        category: json
            .get("category")
            .and_then(Value::as_str)
            .map(String::from),
        group: json.get("group").and_then(Value::as_str).map(String::from),
        traits: traits_json
            .as_array()
            .map(|arr| json_utils::from_json_vec_of_str_to_vec_of_str(arr))
            .unwrap_or_default(),
        dmg_type: base
            .get("damageType")
            .and_then(Value::as_str)
            .map(String::from),
        n_of_dice: base.get("dice").and_then(Value::as_i64),
        die_size: base
            .get("die")
            .and_then(Value::as_str)
            .and_then(|s| s.trim_start_matches('d').parse::<i64>().ok()),
        bonus_dmg: base.get("modifier").and_then(Value::as_i64).unwrap_or(0),
        range_increment: range_json
            .and_then(|r| r.get("increment"))
            .and_then(Value::as_i64),
        max_range: range_json
            .and_then(|r| r.get("max"))
            .and_then(Value::as_i64),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn no_lookup(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn extracts_a_shapeshifted_fist_strike() {
        // Jaul Mezmin "Change Shape" (werecreature humanoid form) shape
        let rules = vec![json!({
            "attackModifier": 16,
            "damage": {
                "base": { "damageType": "bludgeoning", "dice": 1, "die": "d6", "modifier": 7 }
            },
            "key": "Strike",
            "label": "PF2E.Strike.Fist.Label",
            "predicate": ["change-shape:humanoid"],
            "slug": "fist",
            "traits": ["unarmed", "agile"]
        })];
        let strikes = extract_granted_strikes(&rules, |raw| Some(format!("resolved:{raw}")));
        assert_eq!(
            vec![GrantedStrike {
                slug: Some("fist".to_string()),
                label: Some("resolved:PF2E.Strike.Fist.Label".to_string()),
                attack_modifier: Some(16),
                category: None,
                group: None,
                traits: vec!["unarmed".to_string(), "agile".to_string()],
                dmg_type: Some("bludgeoning".to_string()),
                n_of_dice: Some(1),
                die_size: Some(6),
                bonus_dmg: 7,
                range_increment: None,
                max_range: None,
            }],
            strikes
        );
    }

    #[test]
    fn falls_back_to_the_raw_label_when_it_does_not_resolve() {
        let rules = vec![json!({
            "attackModifier": 20,
            "category": "unarmed",
            "damage": {
                "base": { "damageType": "slashing", "dice": 2, "die": "d10", "modifier": 7 }
            },
            "group": "brawling",
            "key": "Strike",
            "label": "PF2E.NPCAbility.Murschen.RedRuinStance.RedClawLabel",
            "predicate": ["red-ruin-stance"],
            "slug": "red-claw",
            "traits": ["finesse", "sweep", "unarmed"]
        })];
        let strikes = extract_granted_strikes(&rules, no_lookup);
        assert_eq!(1, strikes.len());
        assert_eq!(
            Some("PF2E.NPCAbility.Murschen.RedRuinStance.RedClawLabel".to_string()),
            strikes[0].label
        );
        assert_eq!(Some("unarmed".to_string()), strikes[0].category);
        assert_eq!(Some("brawling".to_string()), strikes[0].group);
        assert_eq!(20, strikes[0].attack_modifier.unwrap());
    }

    #[test]
    fn extracts_a_thrown_natural_attack_with_range() {
        // Real Jitterbone Contortionist "Osseous Defense" shape
        let rules = vec![json!({
            "attackModifier": 14,
            "damage": {
                "base": { "damageType": "piercing", "dice": 2, "die": "d6", "modifier": 2 }
            },
            "key": "Strike",
            "label": "PF2E.BattleForm.Attack.BoneDart",
            "predicate": ["osseous-defense"],
            "range": { "increment": 20 },
            "slug": "bone-dart",
            "traits": ["agile", "thrown"]
        })];
        let strikes = extract_granted_strikes(&rules, no_lookup);
        assert_eq!(Some(20), strikes[0].range_increment);
        assert_eq!(None, strikes[0].max_range);
    }

    #[test]
    fn supports_multiple_strikes_granted_by_the_same_action() {
        // Jaul Mezmin "Change Shape" grants fist/claw/jaws across its different forms
        let rules = vec![
            json!({
                "key": "Strike", "slug": "fist",
                "damage": {"base": {"damageType": "bludgeoning", "dice": 1, "die": "d6"}}
            }),
            json!({
                "key": "Strike", "slug": "claw",
                "damage": {"base": {"damageType": "slashing", "dice": 1, "die": "d6"}}
            }),
            json!({
                "key": "Strike", "slug": "jaws",
                "damage": {"base": {"damageType": "piercing", "dice": 2, "die": "d8"}}
            }),
        ];
        let strikes = extract_granted_strikes(&rules, no_lookup);
        assert_eq!(3, strikes.len());
        assert_eq!(Some("jaws".to_string()), strikes[2].slug);
    }

    #[test]
    fn ignores_non_strike_rule_elements() {
        let rules = vec![
            json!({"key": "Aura", "radius": 30}),
            json!({"key": "FastHealing", "value": 5}),
        ];
        assert!(extract_granted_strikes(&rules, no_lookup).is_empty());
    }

    #[test]
    fn drops_a_strike_missing_a_damage_instance() {
        let rules = vec![json!({"key": "Strike", "slug": "ghost-strike"})];
        assert!(extract_granted_strikes(&rules, no_lookup).is_empty());
    }
}
