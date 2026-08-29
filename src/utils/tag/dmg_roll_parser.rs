// [[/gmr 1d6 #Minutes]]{1d6 minutes}.

use crate::schema::localize_loader::ActionNames;
use capitalize::Capitalize;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub fn clean_description(
    description: &str,
    lookup: impl Fn(&str) -> Option<String>,
    actions: &HashMap<String, ActionNames>,
) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\[([^\[\]]*)]](\{(.*?)})?").unwrap());
    for curr_match in SPLIT_REGEX.captures_iter(description) {
        let dirty_match = curr_match.get(0).unwrap().as_str();
        let square_bracket_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");
        let is_r_command = square_bracket_content.trim_start().starts_with("/r");
        let is_act_command = square_bracket_content.trim_start().starts_with("/act");

        let cleaned_match = if is_r_command {
            clean_description_without_curly_brackets(square_bracket_content)
                .trim()
                .to_string()
        } else if let Some(curly_bracket_content) = curr_match.get(3).map(|x| x.as_str()) {
            curly_bracket_content.to_string()
        } else if is_act_command {
            format_act_command(square_bracket_content, &lookup, actions)
        } else {
            clean_description_without_curly_brackets(square_bracket_content)
        };

        let needs_space = (is_r_command || is_act_command)
            && desc
                .find(dirty_match)
                .and_then(|pos| desc[..pos].chars().last())
                .map(|c| c.is_alphanumeric() || c == '_')
                .unwrap_or(false);

        let replacement = if needs_space {
            format!(" {cleaned_match}")
        } else {
            cleaned_match
        };

        desc = desc.replacen(dirty_match, &replacement, 1);
    }
    desc
}

pub fn clean_description_without_curly_brackets(description: &str) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"/(?:gmr|r) ([0-9]+d[0-9]+[+\-]?[0-9]*)(?:\s.*)?").unwrap());
    for curr_match in SPLIT_REGEX.captures_iter(description) {
        let dirty_match = curr_match.get(0).unwrap().as_str();
        let cleaned_match = curr_match.get(1).map(|x| x.as_str()).unwrap_or(dirty_match);
        desc = desc.replacen(dirty_match, cleaned_match, 1);
    }
    desc
}

/// Best-effort fallback for when `slug` has no entry
fn slug_to_title_case(slug: &str) -> String {
    const SMALL_WORDS: &[&str] = &["a", "an", "in", "of", "the", "to"];
    slug.split('-')
        .enumerate()
        .map(|(i, word)| {
            if i != 0 && SMALL_WORDS.contains(&word) {
                word.to_string()
            } else {
                word.capitalize()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Resolves an action or variant slug to its real display name via the system's own
/// action-macro data, falling back to title-casing the slug only when that
/// data doesn't have an entry for it
fn resolve_action_name(
    slug: &str,
    loc_key: Option<&str>,
    lookup: &impl Fn(&str) -> Option<String>,
) -> String {
    loc_key
        .and_then(lookup)
        .unwrap_or_else(|| slug_to_title_case(slug))
}

/// Formats a `/act` command (e.g. `/act escape dc=20`) into a human-readable label when it
/// has no explicit `{curly label}` override
fn format_act_command(
    square_bracket_content: &str,
    lookup: &impl Fn(&str) -> Option<String>,
    actions: &HashMap<String, ActionNames>,
) -> String {
    let rest = square_bracket_content
        .trim_start()
        .strip_prefix("/act")
        .unwrap_or("")
        .trim();
    let mut tokens = rest.split_whitespace();
    let slug = tokens.next().unwrap_or("");

    let mut dc: Option<&str> = None;
    let mut statistic: Option<&str> = None;
    let mut variant: Option<&str> = None;
    for token in tokens {
        if let Some((key, value)) = token.split_once('=') {
            match key {
                "dc" | "difficulty-class" => dc = Some(value),
                "skill" | "statistic" | "stat" => statistic = Some(value),
                "variant" => variant = Some(value),
                _ => {}
            }
        }
    }

    let known = actions.get(slug);
    let action_name = if slug.is_empty() {
        String::new()
    } else {
        resolve_action_name(slug, known.map(|a| a.name.as_str()), lookup)
    };
    let base = match variant {
        Some(v) => {
            let variant_loc_key = known.and_then(|a| a.variants.get(v)).map(String::as_str);
            let variant_name = resolve_action_name(v, variant_loc_key, lookup);
            if action_name.is_empty() {
                variant_name
            } else {
                format!("{action_name} - {variant_name}")
            }
        }
        None => action_name,
    };

    let details = match (dc, statistic) {
        (Some(dc), Some(stat)) => Some(format!("DC {dc} {}", slug_to_title_case(stat))),
        (Some(dc), None) => Some(format!("DC {dc}")),
        (None, Some(stat)) => Some(slug_to_title_case(stat)),
        (None, None) => None,
    };

    match details {
        Some(d) if base.is_empty() => format!("({d})"),
        Some(d) => format!("{base} ({d})"),
        None => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    fn no_lookup(_: &str) -> Option<String> {
        None
    }

    /// Mimics the real slug -> loc-key data `localize_loader::parsed_actions()` would produce
    /// from the system's action-macro TS files, for the handful of slugs these tests exercise.
    #[fixture]
    fn mock_actions() -> HashMap<String, ActionNames> {
        let entry = |name: &str| ActionNames {
            name: name.to_string(),
            variants: HashMap::new(),
        };
        let mut actions = HashMap::from([
            ("escape".to_string(), entry("PF2E.Actions.Escape.Title")),
            (
                "demoralize".to_string(),
                entry("PF2E.Actions.Demoralize.Title"),
            ),
            (
                "disable-device".to_string(),
                entry("PF2E.Actions.DisableDevice.Title"),
            ),
            ("seek".to_string(), entry("PF2E.Actions.Seek.Title")),
            ("track".to_string(), entry("PF2E.Actions.Track.Title")),
        ]);
        actions.insert(
            "make-an-impression".to_string(),
            ActionNames {
                name: "PF2E.Actions.MakeAnImpression.Title".to_string(),
                variants: HashMap::from([(
                    "performance".to_string(),
                    "PF2E.Actions.MakeAnImpression.Performance.Title".to_string(),
                )]),
            },
        );
        actions
    }

    fn mock_lookup(path: &str) -> Option<String> {
        match path {
            "PF2E.Actions.Escape.Title" => Some("Escape"),
            "PF2E.Actions.Demoralize.Title" => Some("Demoralize"),
            "PF2E.Actions.DisableDevice.Title" => Some("Disable a Device"),
            "PF2E.Actions.Seek.Title" => Some("Seek"),
            "PF2E.Actions.Track.Title" => Some("Track"),
            "PF2E.Actions.MakeAnImpression.Title" => Some("Make an Impression"),
            "PF2E.Actions.MakeAnImpression.Performance.Title" => Some("Performance"),
            _ => None,
        }
        .map(str::to_string)
    }

    #[rstest]
    #[case("[[/gmr 1d6 #Minutes]]{1d6 minutes}.", "1d6 minutes.")]
    #[case(
        "before disappearing after [[/gmr 1d6 #Minutes]]{1d6 minutes}.</p>",
        "before disappearing after 1d6 minutes.</p>"
    )]
    #[case("[[/gmr 1d4 #rounds]]{1d4 rounds}", "1d4 rounds")]
    #[case("[[/gmr 1d4 #Recharge Breath Weapon]]{1d4 rounds}", "1d4 rounds")]
    #[case("[[/gmr 1d4 #Recharge Kobold Breath]]{1d4 rounds}", "1d4 rounds")]
    #[case("[[/gmr 1d20+16 #Councteract]]{+16}", "+16")]
    #[case("[[/gmr 1d20+21 #Counteract]]{+21}", "+21")]
    #[case("[[/gmr 1d4 #Recharge Jangle the Chain]]{1d4 rounds}", "1d4 rounds")]
    #[case("[[/gmr 1d6 #Rounds]]{1d6 rounds}", "1d6 rounds")]
    fn simple_clean(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("[[[/gmr 1d6 #Minutes]]{1d6 minutes}.", "[1d6 minutes.")]
    fn clean_dirty_description(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }
    #[rstest]
    #[case("[[/r 1d6]]", "1d6")]
    #[case("[[/r 2d8+3]]", "2d8+3")]
    #[case("[[/r 1d20-2]]", "1d20-2")]
    fn clean_bare_r_roll(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("[[/gmr 1d4+2 #Recharge]]{1d4+2 rounds}", "1d4+2 rounds")]
    #[case("[[/gmr 2d6+1 #Duration]]{2d6+1 minutes}", "2d6+1 minutes")]
    fn clean_gmr_with_modifier(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("1d6 minutes", "1d6 minutes")]
    fn clean_without_tags(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("[[/gmr 1d6 #Minutes]]{1d6 minutes}.", "1d6 minutes.")]
    #[case(
        "before disappearing after [[/gmr 1d6 #Minutes]]{1d6 minutes}.</p>",
        "before disappearing after 1d6 minutes.</p>"
    )]
    #[case("[[/gmr 1d4 #Alchemical Rupture]]", "1d4")]
    #[case("([[/gmr 1d4]] minutes)", "(1d4 minutes)")]
    fn clean_without_curly_brackets(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "<p>Initiative modifier is [[/r 1d20+5 #Initiative]]{+5}</p>",
        "<p>Initiative modifier is 1d20+5</p>"
    )]
    #[case(
        "jaws[[/r 1d20+16 #Jaws]]{+16}/[[/r 1d20+11 #Jaws]]{+11}/[[/r 1d20+6 #Jaws]]{+6}",
        "jaws 1d20+16/1d20+11/1d20+6"
    )]
    #[case(
        "there is a 1 in 4 chance of hitting you (1 on [[/r 1d4]]).",
        "there is a 1 in 4 chance of hitting you (1 on 1d4)."
    )]
    fn clean_roll_slash_r_check(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("([[/act escape dc=20]]{DC 20})", "(DC 20)")]
    #[case("[[/act escape dc=20]]{Escape}", "Escape")]
    fn clean_act_command_with_curly(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    // /act commands with no {curly label}: the action name comes from the real
    // slug -> loc-key -> string chain (mock_actions + mock_lookup), not a guess
    #[rstest]
    #[case(
        "A creature can [[/act seek dc=22]] to learn clues.",
        "A creature can Seek (DC 22) to learn clues."
    )]
    #[case(
        "(requiring [[/act disable-device dc=28]])",
        "(requiring Disable a Device (DC 28))"
    )]
    #[case("until it can [[/act escape dc=16]].", "until it can Escape (DC 16).")]
    #[case(
        "the creature is Immobilized ([[/act escape dc=22]]).",
        "the creature is Immobilized (Escape (DC 22))."
    )]
    #[case("[[/act demoralize]]", "Demoralize")]
    #[case("[[/act track skill=religion]]", "Track (Religion)")]
    #[case(
        "[[/act disable-device skill=thievery dc=32]]",
        "Disable a Device (DC 32 Thievery)"
    )]
    #[case(
        "[[/act make-an-impression variant=performance]]",
        "Make an Impression - Performance"
    )]
    fn clean_act_command_without_curly(
        mock_actions: HashMap<String, ActionNames>,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, mock_lookup, &mock_actions);
        assert_eq!(expected, parsed_description);
    }

    // a slug absent from the action data
    // must still fall back to a readable title-cased label, never raw "/act" text
    #[rstest]
    #[case("[[/act some-brand-new-action]]", "Some Brand New Action")]
    fn clean_act_command_unknown_slug_falls_back_to_heuristic(
        mock_actions: HashMap<String, ActionNames>,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, mock_lookup, &mock_actions);
        assert_eq!(expected, parsed_description);
    }

    // an /act tag glued directly to the preceding word must not merge into it,
    // mirroring the existing behavior for glued /r roll tags
    #[rstest]
    #[case("jaws[[/act escape dc=20]]", "jaws Escape (DC 20)")]
    #[case("word[[/act demoralize]] end", "word Demoralize end")]
    fn clean_act_command_glued_to_word(
        mock_actions: HashMap<String, ActionNames>,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, mock_lookup, &mock_actions);
        assert_eq!(expected, parsed_description);
    }

    // malformed data: no action slug given at all
    #[rstest]
    #[case("[[/act]]", "")]
    #[case("[[/act ]]", "")]
    fn clean_act_command_missing_slug(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }

    // Real snippet from Glue Bullet
    #[rstest]
    #[case(
        "for [[/r 2d4 #rounds]]{2d4 rounds}, or until it [[/act escape show-dc=all dc=18]]{Escapes} against a DC of 18.",
        "for 2d4, or until it Escapes against a DC of 18."
    )]
    fn clean_act_command_with_unknown_key_and_curly_label(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input, no_lookup, &HashMap::new());
        assert_eq!(expected, parsed_description);
    }
}
