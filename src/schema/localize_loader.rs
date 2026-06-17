use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::{env, fs};

static LANG_DATA: OnceLock<Value> = OnceLock::new();
static SF2E_OVERRIDES: OnceLock<Value> = OnceLock::new();

static TRAITS_ASSOCIATION_FILE: OnceLock<String> = OnceLock::new();

static PARSED_TRAITS: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();

fn base_path() -> String {
    env::var("SOURCE_DOWNLOAD_PATH").expect(
        "DOWNLOAD PATH NOT SET.. Aborting. Hint: set SOURCE_DOWNLOAD_PATH environmental variable",
    )
}

pub fn lang_data() -> &'static Value {
    LANG_DATA.get_or_init(|| {
        let base_path = base_path();
        let file_content = fs::read_to_string(format!("{base_path}/static/lang/en.json"))
            .expect("failed to read en.json");
        serde_json::from_str(&file_content).expect("failed to parse en.json")
    })
}

pub fn sf2e_data() -> &'static Value {
    SF2E_OVERRIDES.get_or_init(|| {
        let base_path = base_path();
        let file_content =
            fs::read_to_string(format!("{base_path}/static/lang/sf2e-overrides-en.json"))
                .expect("failed to read sf2e-overrides-en.json");
        serde_json::from_str(&file_content).expect("failed to parse sf2e-overrides-en.json")
    })
}

fn traits_ts_association_file() -> &'static String {
    TRAITS_ASSOCIATION_FILE.get_or_init(|| {
        let base_path = base_path();
        let x = format!("{base_path}/src/scripts/config/traits.ts");
        fs::read_to_string(x).expect("failed to read sf2e-overrides-en.json")
    })
}

pub fn parsed_traits() -> &'static HashMap<String, HashMap<String, String>> {
    PARSED_TRAITS.get_or_init(|| parse_ts_traits(traits_ts_association_file()))
}

/// Walks a dot-separated path through a JSON value, returning the string at that path if present.
pub fn lookup_path(json_data: &Value, path: &str) -> Option<String> {
    let mut current = json_data;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    current.as_str().map(String::from)
}

fn parse_ts_traits(ts_content: &str) -> HashMap<String, HashMap<String, String>> {
    // Strip R.pick(...) and R.omit(...) assignments since we can't evaluate them
    let rpick_omit_re = Regex::new(r"const\s+\w+\s*=\s*R\.(pick|omit)\([^;]+;").unwrap();
    let cleaned = rpick_omit_re.replace_all(ts_content, "");
    let block_re = Regex::new(r"const\s+(\w+)\s*(?::[^=]*)?\s*=\s*\{([^}]*)\}").unwrap();
    let pair_re = Regex::new(r#""?([\w-]+)"?\s*:\s*"([^"]+)""#).unwrap();
    let spread_re = Regex::new(r"\.\.\.([\w]+)").unwrap();

    let mut raw_blocks: HashMap<String, (Vec<String>, HashMap<String, String>)> = HashMap::new();

    for block_cap in block_re.captures_iter(&cleaned) {
        let block_name = block_cap[1].to_string();
        let block_body = &block_cap[2];
        let spreads: Vec<String> = spread_re
            .captures_iter(block_body)
            .map(|c| c[1].to_string())
            .collect();
        let mut pairs: HashMap<String, String> = HashMap::new();
        for pair_cap in pair_re.captures_iter(block_body) {
            pairs.insert(pair_cap[1].to_string(), pair_cap[2].to_string());
        }
        raw_blocks.insert(block_name, (spreads, pairs));
    }

    // Resolve spreads
    let mut resolved: HashMap<String, HashMap<String, String>> = HashMap::new();
    loop {
        let mut changed = false;
        for (name, (spreads, own_pairs)) in &raw_blocks {
            if resolved.contains_key(name) {
                continue;
            }
            // If a spread references an R.pick/omit block that was stripped,
            // it won't be in raw_blocks, so we treat it as empty and skip it
            if !spreads
                .iter()
                .all(|s| resolved.contains_key(s) || !raw_blocks.contains_key(s))
            {
                continue;
            }
            let mut merged: HashMap<String, String> = HashMap::new();
            for spread in spreads {
                if let Some(spread_pairs) = resolved.get(spread) {
                    merged.extend(spread_pairs.clone());
                }
            }
            merged.extend(own_pairs.clone());
            resolved.insert(name.clone(), merged);
            changed = true;
        }
        if !changed {
            break;
        }
    }

    // Flatten in topological order so that blocks resolved later (which override
    // spreads) win over earlier ones via plain insert
    let mut ordered_names: Vec<String> = Vec::new();
    let mut remaining: Vec<String> = raw_blocks.keys().cloned().collect();
    while !remaining.is_empty() {
        let before = remaining.len();
        remaining.retain(|name| {
            let (spreads, _) = &raw_blocks[name];
            let ready = spreads
                .iter()
                .all(|s| ordered_names.contains(s) || !raw_blocks.contains_key(s));
            if ready {
                ordered_names.push(name.clone());
            }
            !ready
        });
        if remaining.len() == before {
            // Cycle or unresolvable, just append the rest
            ordered_names.append(&mut remaining);
        }
    }

    // Flatten into { "haunt": { "trait": "PF2E.TraitHaunt", "description": "PF2E.TraitDescriptionHaunt" } }
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    for name in &ordered_names {
        if let Some(pairs) = resolved.get(name) {
            for (key, value) in pairs {
                let kind = if value.contains("Description") {
                    "description"
                } else {
                    "trait"
                };
                result
                    .entry(key.clone())
                    .or_default()
                    .insert(kind.to_string(), value.clone());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn simple_ts() -> &'static str {
        r#"
            const kingmakerDescriptions = {
                cavalry: "PF2E.Kingmaker.TraitDescription.cavalry",
                infantry: "PF2E.Kingmaker.TraitDescription.infantry",
            };
            const preciousMaterialDescriptions = {
                abysium: "PF2E.PreciousMaterialAbysiumDescription",
                "cold-iron": "PF2E.PreciousMaterialColdIronDescription",
            };
            const traitDescriptions = {
                ...kingmakerDescriptions,
                ...preciousMaterialDescriptions,
                aasimar: "PF2E.TraitDescriptionAasimar",
            };
            const armorTraits = {
                alchemical: "PF2E.TraitAlchemical",
            };
        "#
    }

    #[fixture]
    fn chained_ts() -> &'static str {
        r#"
            const base = {
                fire: "PF2E.TraitFire",
            };
            const middle = {
                ...base,
                water: "PF2E.TraitWater",
            };
            const top = {
                ...middle,
                earth: "PF2E.TraitEarth",
            };
        "#
    }

    #[fixture]
    fn override_ts() -> &'static str {
        r#"
            const base = {
                alchemical: "PF2E.TraitAlchemical.Old",
            };
            const derived = {
                ...base,
                alchemical: "PF2E.TraitAlchemical.New",
            };
        "#
    }

    #[fixture]
    fn json_data() -> Value {
        serde_json::json!({
            "PF2E": {
                "TraitFire": "Fire",
                "TraitWater": "Water",
                "Kingmaker": {
                    "Trait": {
                        "cavalry": "Cavalry"
                    }
                }
            }
        })
    }

    #[rstest]
    // Description block keys are stored under "description"
    #[case("aasimar", "description", Some("PF2E.TraitDescriptionAasimar"))]
    #[case(
        "cavalry",
        "description",
        Some("PF2E.Kingmaker.TraitDescription.cavalry")
    )]
    #[case(
        "abysium",
        "description",
        Some("PF2E.PreciousMaterialAbysiumDescription")
    )]
    #[case(
        "cold-iron",
        "description",
        Some("PF2E.PreciousMaterialColdIronDescription")
    )]
    // Trait block keys are stored under "trait"
    #[case("alchemical", "trait", Some("PF2E.TraitAlchemical"))]
    // Unknown keys return None
    #[case("unknown", "trait", None)]
    #[case("unknown", "description", None)]
    fn test_spread_resolution(
        simple_ts: &'static str,
        #[case] key: &str,
        #[case] kind: &str,
        #[case] expected: Option<&str>,
    ) {
        let result = parse_ts_traits(simple_ts);
        assert_eq!(
            result
                .get(key)
                .and_then(|m| m.get(kind))
                .map(String::as_str),
            expected
        );
    }

    #[rstest]
    // Chained spread keys all end up under "trait" since no block name contains "description"
    #[case("fire", "trait", Some("PF2E.TraitFire"))]
    #[case("water", "trait", Some("PF2E.TraitWater"))]
    #[case("earth", "trait", Some("PF2E.TraitEarth"))]
    fn test_chained_spreads(
        chained_ts: &'static str,
        #[case] key: &str,
        #[case] kind: &str,
        #[case] expected: Option<&str>,
    ) {
        let result = parse_ts_traits(chained_ts);
        assert_eq!(
            result
                .get(key)
                .and_then(|m| m.get(kind))
                .map(String::as_str),
            expected
        );
    }

    #[rstest]
    #[case("alchemical", "trait", Some("PF2E.TraitAlchemical.New"))]
    fn test_own_key_overrides_spread(
        override_ts: &'static str,
        #[case] key: &str,
        #[case] kind: &str,
        #[case] expected: Option<&str>,
    ) {
        let result = parse_ts_traits(override_ts);
        assert_eq!(
            result
                .get(key)
                .and_then(|m| m.get(kind))
                .map(String::as_str),
            expected
        );
    }

    #[rstest]
    #[case("PF2E.TraitFire", Some("Fire"))]
    #[case("PF2E.TraitWater", Some("Water"))]
    #[case("PF2E.Kingmaker.Trait.cavalry", Some("Cavalry"))]
    #[case("PF2E.Unknown", None)]
    fn test_lookup_path(json_data: Value, #[case] path: &str, #[case] expected: Option<&str>) {
        assert_eq!(lookup_path(&json_data, path).as_deref(), expected);
    }
}
