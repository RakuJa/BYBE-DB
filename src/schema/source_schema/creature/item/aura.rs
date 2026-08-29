use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct AuraEffect {
    pub name: Option<String>,
    pub affects: String,
    pub events: Vec<String>,
    pub removes_on_exit: bool,
    pub includes_self: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aura {
    pub slug: Option<String>,
    pub radius: Option<i64>,
    pub traits: Vec<String>,
    pub effects: Vec<AuraEffect>,
}

/// Extracts every Foundry `"key": "Aura"` rule element out of an item's raw `system.rules` array
///
/// `radius` is a static number for the overwhelming majority of auras; a handful resolve it
/// dynamically at roll time (e.g. `ternary(lte(@actor.hitPoints.value, 125), 60, 30)`), which
/// has no static value here and is left `None` rather than dropping the whole aura, since the
/// traits/effects are still meaningful without it.
pub fn extract_auras(rules_json: &[Value]) -> Vec<Aura> {
    rules_json
        .iter()
        .filter(|rule| rule.get("key").and_then(Value::as_str) == Some("Aura"))
        .map(parse_aura)
        .collect()
}

fn parse_aura(json: &Value) -> Aura {
    let traits_json = json_utils::get_field_from_json(json, "traits");
    let effects_json = json_utils::get_field_from_json(json, "effects");
    Aura {
        slug: json.get("slug").and_then(Value::as_str).map(String::from),
        radius: json.get("radius").and_then(Value::as_i64),
        traits: traits_json
            .as_array()
            .map(|arr| json_utils::from_json_vec_of_str_to_vec_of_str(arr))
            .unwrap_or_default(),
        effects: effects_json
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(parse_aura_effect)
            .collect(),
    }
}

fn parse_aura_effect(json: &Value) -> AuraEffect {
    let events_json = json_utils::get_field_from_json(json, "events");
    AuraEffect {
        name: json
            .get("uuid")
            .and_then(Value::as_str)
            .and_then(effect_name_from_uuid),
        affects: json
            .get("affects")
            .and_then(Value::as_str)
            .unwrap_or("all")
            .to_string(),
        events: events_json
            .as_array()
            .map(|arr| json_utils::from_json_vec_of_str_to_vec_of_str(arr))
            .unwrap_or_else(|| vec!["enter".to_string()]),
        removes_on_exit: json
            .get("removeOnExit")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        includes_self: json
            .get("includesSelf")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    }
}

/// A transmitted effect is referenced by Foundry UUID, e.g.
/// `Compendium.pf2e.bestiary-effects.Item.Effect: Despair`.
fn effect_name_from_uuid(uuid: &str) -> Option<String> {
    uuid.split_once(".Item.").map(|(_, name)| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_a_simple_radius_and_traits() {
        // graveknight "Sacrilegious Aura" shape
        let rules = vec![
            json!({
                "itemId": "{item|id}",
                "key": "ItemAlteration",
                "mode": "override",
                "property": "name",
                "value": "PF2E.NPCAbility.Graveknight.SacrilegiousAura.Name"
            }),
            json!({
                "key": "Aura",
                "radius": 30,
                "slug": "sacrilegious-aura",
                "traits": ["divine", "void"]
            }),
        ];
        let auras = extract_auras(&rules);
        assert_eq!(
            vec![Aura {
                slug: Some("sacrilegious-aura".to_string()),
                radius: Some(30),
                traits: vec!["divine".to_string(), "void".to_string()],
                effects: vec![],
            }],
            auras
        );
    }

    #[test]
    fn extracts_a_transmitted_effect_with_a_readable_name() {
        // "Despair" aura shape (Chafkhem)
        let rules = vec![json!({
            "effects": [
                {
                    "affects": "all",
                    "events": ["enter"],
                    "includesSelf": false,
                    "predicate": ["target:mode:living"],
                    "uuid": "Compendium.pf2e.bestiary-effects.Item.Effect: Despair"
                }
            ],
            "key": "Aura",
            "radius": 30,
            "slug": "despair",
            "traits": ["divine", "emotion", "fear", "incapacitation", "mental"]
        })];
        let auras = extract_auras(&rules);
        assert_eq!(1, auras.len());
        assert_eq!(
            vec![AuraEffect {
                name: Some("Effect: Despair".to_string()),
                affects: "all".to_string(),
                events: vec!["enter".to_string()],
                removes_on_exit: true,
                includes_self: false,
            }],
            auras[0].effects
        );
    }

    #[test]
    fn extracts_multiple_effects_targeting_different_sides() {
        // "Harmonizing Aura" shape (angelic chorus): no explicit events/removeOnExit
        let rules = vec![json!({
            "effects": [
                {
                    "affects": "allies",
                    "includesSelf": false,
                    "uuid": "Compendium.pf2e.bestiary-effects.Item.Effect: Harmonizing Aura (Allies)"
                },
                {
                    "affects": "enemies",
                    "includesSelf": false,
                    "uuid": "Compendium.pf2e.bestiary-effects.Item.Effect: Harmonizing Aura (Enemies)"
                }
            ],
            "key": "Aura",
            "radius": 20,
            "slug": "harmonizing-aura",
            "traits": ["divine", "sonic"]
        })];
        let auras = extract_auras(&rules);
        assert_eq!(2, auras[0].effects.len());
        assert_eq!(
            Some("Effect: Harmonizing Aura (Allies)".to_string()),
            auras[0].effects[0].name
        );
        assert_eq!("allies", auras[0].effects[0].affects);
        // events defaults to ["enter"] when omitted, matching the PF2E rule element schema
        assert_eq!(vec!["enter".to_string()], auras[0].effects[0].events);
        assert_eq!("enemies", auras[0].effects[1].affects);
    }

    #[test]
    fn keeps_traits_and_slug_when_radius_is_a_dynamic_formula() {
        // "tormented-burning" shape: radius depends on the acting creature's current HP
        let rules = vec![json!({
            "key": "Aura",
            "radius": "ternary(lte(@actor.hitPoints.value, 125), 60, 30)",
            "slug": "tormented-burning",
            "traits": ["fire"]
        })];
        let auras = extract_auras(&rules);
        assert_eq!(
            vec![Aura {
                slug: Some("tormented-burning".to_string()),
                radius: None,
                traits: vec!["fire".to_string()],
                effects: vec![],
            }],
            auras
        );
    }

    #[test]
    fn ignores_non_aura_rule_elements() {
        let rules = vec![
            json!({"key": "Resistance", "type": "fire", "value": 5}),
            json!({"key": "FlatModifier", "value": 2}),
        ];
        assert!(extract_auras(&rules).is_empty());
    }

    #[test]
    fn handles_missing_slug_and_traits() {
        let rules = vec![json!({"key": "Aura", "radius": 10})];
        let auras = extract_auras(&rules);
        assert_eq!(
            vec![Aura {
                slug: None,
                radius: Some(10),
                traits: vec![],
                effects: vec![],
            }],
            auras
        );
    }

    #[test]
    fn supports_more_than_one_aura_on_the_same_action() {
        // "Unsettled Aura" (gosreg) shape: two independent Aura rule elements
        let rules = vec![
            json!({"key": "Aura", "radius": 30, "slug": "unsettled-aura-a"}),
            json!({"key": "Aura", "radius": 60, "slug": "unsettled-aura-b"}),
        ];
        let auras = extract_auras(&rules);
        assert_eq!(2, auras.len());
        assert_eq!(Some("unsettled-aura-a".to_string()), auras[0].slug);
        assert_eq!(Some("unsettled-aura-b".to_string()), auras[1].slug);
    }
}
