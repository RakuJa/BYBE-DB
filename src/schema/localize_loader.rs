use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{env, fs};

static LANG_DATA: OnceLock<Value> = OnceLock::new();
static SF2E_OVERRIDES: OnceLock<Value> = OnceLock::new();
static ACTION_LANG_DATA: OnceLock<Value> = OnceLock::new();

static TRAITS_ASSOCIATION_FILE: OnceLock<String> = OnceLock::new();

static PARSED_TRAITS: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();
static PARSED_ACTIONS: OnceLock<HashMap<String, ActionNames>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionNames {
    /// Localization key(e.g. "PF2E.Actions.Escape.Title")
    pub name: String,
    /// variant slug -> localization key (e.g. "acting" -> "PF2E.Actions.Perform.Acting.Title")
    pub variants: HashMap<String, String>,
}

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

pub fn action_lang_data() -> &'static Value {
    ACTION_LANG_DATA.get_or_init(|| {
        let base_path = base_path();
        let file_content = fs::read_to_string(format!("{base_path}/static/lang/action-en.json"))
            .expect("failed to read action-en.json");
        serde_json::from_str(&file_content).expect("failed to parse action-en.json")
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

pub fn parsed_actions() -> &'static HashMap<String, ActionNames> {
    PARSED_ACTIONS.get_or_init(|| {
        let base_path = base_path();
        parse_ts_actions_dir(Path::new(&format!(
            "{base_path}/src/module/system/action-macros"
        )))
    })
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

/// Recursively collects `.ts` files under `dir`, skipping the barrel/helper files that don't
/// define an action themselves
fn collect_action_macro_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_action_macro_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "ts") {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !matches!(file_name, "index.ts" | "helpers.ts" | "types.ts") {
                out.push(path);
            }
        }
    }
}

/// Resolves `${IDENT}` template-literal interpolations using a file-local `const IDENT = "...";`
/// table. Left as-is if the referenced const isn't found (best-effort).
fn resolve_template_literal(raw: &str, consts: &HashMap<String, String>) -> String {
    static INTERPOLATION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{(\w+)}").unwrap());
    INTERPOLATION_RE
        .replace_all(raw, |caps: &regex::Captures| {
            consts
                .get(&caps[1])
                .cloned()
                .unwrap_or_else(|| caps[0].to_string())
        })
        .to_string()
}

/// Parses a single action-macro source file (e.g. `thievery/disable-device.ts`) into its slug
/// and localization keys. Each file defines exactly one top-level action (optionally with
/// variants), always as a `name: ...` field immediately followed, somewhere later in the same
/// object, by a `slug: "..."` field - this holds for both the action itself and each of its
/// variant entries, so pairing them up in file order recovers the full slug/variant structure.
fn parse_ts_action_file(content: &str) -> Option<(String, ActionNames)> {
    static CONST_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"const\s+(\w+)\s*=\s*"([^"]+)";"#).unwrap());
    static NAME_OR_SLUG_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"name:\s*"([^"]+)"|name:\s*`([^`]+)`|slug:\s*"([\w-]+)""#).unwrap()
    });

    let consts: HashMap<String, String> = CONST_RE
        .captures_iter(content)
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .collect();

    let mut pending_name: Option<String> = None;
    let mut pairs: Vec<(String, String)> = Vec::new();
    for caps in NAME_OR_SLUG_RE.captures_iter(content) {
        if let Some(raw_name) = caps.get(1).or_else(|| caps.get(2)) {
            pending_name = Some(resolve_template_literal(raw_name.as_str(), &consts));
        } else if let Some(slug) = caps.get(3)
            && let Some(name) = pending_name.take()
        {
            pairs.push((slug.as_str().to_string(), name));
        }
    }

    if pairs.is_empty() {
        return None;
    }
    let (root_slug, root_name) = pairs.remove(0);
    Some((
        root_slug,
        ActionNames {
            name: root_name,
            variants: pairs.into_iter().collect(),
        },
    ))
}

fn parse_ts_actions_dir(dir: &Path) -> HashMap<String, ActionNames> {
    let mut files = Vec::new();
    collect_action_macro_files(dir, &mut files);

    let mut result = HashMap::new();
    for file in files {
        if let Ok(content) = fs::read_to_string(&file)
            && let Some((slug, action_names)) = parse_ts_action_file(&content)
        {
            result.insert(slug, action_names);
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
    
    #[rstest]
    fn parse_action_file_simple_slug() {
        let ts = r#"
            const action = new SingleCheckAction({
                cost: 2,
                name: "PF2E.Actions.DisableDevice.Title",
                section: "skill",
                slug: "disable-device",
                statistic: "thievery",
            });
        "#;
        let (slug, names) = parse_ts_action_file(ts).expect("should parse an action");
        assert_eq!(slug, "disable-device");
        assert_eq!(names.name, "PF2E.Actions.DisableDevice.Title");
        assert!(names.variants.is_empty());
    }

    #[rstest]
    fn parse_action_file_with_prefix_const_and_variants() {
        let ts = r#"
            const PREFIX = "PF2E.Actions.Perform";
            const action = new SingleCheckAction({
                name: `${PREFIX}.Title`,
                slug: "perform",
                variants: [
                    {
                        name: `${PREFIX}.Acting.Title`,
                        slug: "acting",
                    },
                    {
                        name: `${PREFIX}.Dance.Title`,
                        slug: "dance",
                    },
                ],
            });
        "#;
        let (slug, names) = parse_ts_action_file(ts).expect("should parse an action");
        assert_eq!(slug, "perform");
        assert_eq!(names.name, "PF2E.Actions.Perform.Title");
        assert_eq!(
            names.variants.get("acting").map(String::as_str),
            Some("PF2E.Actions.Perform.Acting.Title")
        );
        assert_eq!(
            names.variants.get("dance").map(String::as_str),
            Some("PF2E.Actions.Perform.Dance.Title")
        );
    }
    
    #[rstest]
    fn parse_action_file_without_slug_returns_none() {
        let ts = r#"
            const action = new SingleCheckAction({
                name: "PF2E.Actions.Craft.Title",
            });
        "#;
        assert_eq!(parse_ts_action_file(ts), None);
    }
}
