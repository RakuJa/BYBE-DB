use crate::utils::tag::{
    check_tag_parser, compendium_tag_parser, dm_roll_parser, dmg_tag_parser, localize_tag_parser,
    template_tag_parser,
};
use {once_cell::sync::Lazy, regex::Regex};

pub fn remove_all_dices_from_description(description: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]*d[1-9][0-9]*").unwrap());
    RE.replace_all(description, "").to_string()
}

pub fn get_dice_inside_string(input: &str) -> Option<String> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]*d[1-9][0-9]*").unwrap());
    RE.find(input).map(|x| x.as_str().to_string())
}

fn clean_description_from_generic_bracket(description: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"\[\[/b?r \(?\{?(\d\*?\d*d?[\d\s\-+]*\d*),?\d*}?\)?[\w\s]*\[?#?[\w\s,]*]]]?(?:\{([\w\s\-+;]*)})?").unwrap()
    });

    let mut clean_description = String::from(description);
    for m in RE.captures_iter(description) {
        let raw_descr = m.get(0).unwrap().as_str();
        let clean_data = m
            .get(2)
            .or_else(|| m.get(1))
            .map(|x| x.as_str())
            .unwrap_or(raw_descr);
        clean_description = clean_description.replace(raw_descr, clean_data);
    }
    clean_description
}

/// Gets the content inside a typical foundry square bracket tag.
/// It ignores everything before the start delimiter and stops when it finds | or ]
/// If the delimiter is not found it returns empty.
/// # Examples
/// ``` rust
/// use get_content_inside_square_brackets as goisb;
/// assert_eq!(goisb("\[start:hi_osi]", "start:"), "hi_osi");
/// assert_eq!(goisb("\[atag:hi_osi|", "start:"), "");
/// assert_eq!(goisb("\[atag|start:hi_osi|something]", "start:"), "hi_osi");
/// ```
pub fn get_content_inside_square_brackets(content: &str, start_delimiter: &str) -> String {
    if let Some(x) = content.split_once(start_delimiter) {
        x.1.chars().take_while(|&c| c != ']' && c != '|').collect()
    } else {
        String::new()
    }
}

/// Checks a cleaned description for tag patterns that survived the parsing pipeline.
/// Returns a list of human-readable issue descriptions.
#[cfg(feature = "dry-run")]
pub fn find_remaining_tags(cleaned_description: &str) -> Vec<String> {
    static AT_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"@[A-Za-z]+\[").unwrap());
    static DOUBLE_BRACKET: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\[").unwrap());

    let mut issues = Vec::new();
    for m in AT_TAG.find_iter(cleaned_description) {
        issues.push(format!("unparsed tag '{}'", m.as_str()));
    }
    if DOUBLE_BRACKET.is_match(cleaned_description) {
        issues.push("unparsed double-bracket '[[' tag".to_string());
    }
    issues
}

pub fn clean_description_from_all_tags(description: &str, item_lvl: Option<i64>) -> String {
    let desc = compendium_tag_parser::clean_description(description);
    let desc = dmg_tag_parser::clean_description(&desc, item_lvl);
    let desc = check_tag_parser::clean_description(&desc);
    let desc = template_tag_parser::clean_description(&desc);
    let desc = dm_roll_parser::clean_description(&desc);
    let desc = clean_description_from_generic_bracket(&desc);
    localize_tag_parser::clean_description(&desc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "<p><strong>Cantrips</strong></p><p>@UUID[Compendium.pf2e.spells-srd.Item.Telekinetic Projectile]</p><p>@UUID[Compendium.pf2e.spells-srd.Item.Daze]</p><p>@UUID[Compendium.pf2e.spells-srd.Item.Detect Magic]</p><p>@UUID[Compendium.pf2e.spells-srd.Item.Light]</p><p>@UUID[Compendium.pf2e.spells-srd.Item.Telekinetic Hand]</p><hr />",
        "<p><strong>Cantrips</strong></p><p>Telekinetic Projectile</p><p>Daze</p><p>Detect Magic</p><p>Light</p><p>Telekinetic Hand</p><hr />"
    )]
    #[case(
        "<p>Small, @Damage[(1d12 + 3)[bludgeoning]], Rupture 10</p>\n<hr />\n<p>@Localize[PF2E.NPC.Abilities.Glossary.SwallowWhole]</p>",
        "<p>Small, 1d12 + 3 bludgeoning, Rupture 10</p>\n<hr />\n<p></p>"
    )]
    fn clean_check_uuid(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description_from_all_tags(input, None);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("[[/r {1d20+30} #Counteract]]{1d20+30}", "1d20+30")]
    #[case("[[/r 3d6[healing]]]", "3d6")]
    fn clean_from_generic_tags(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description_from_generic_bracket(input);
        assert_eq!(expected, parsed_description);
    }

    #[test]
    fn clean_spider_gun_adjacent_checks_and_act_command() {
        let input = "it must attempt an @Check[athletics|dc:20]{Athletics} check or @Check[reflex|dc:20]{Reflex} save against DC 20. On a critical failure, it's @UUID[Compendium.pf2e.conditionitems.Item.Immobilized] for 1 round or until it Escapes ([[/act escape dc=20]]{DC 20}) or destroys the webbing.";
        let result = clean_description_from_all_tags(input, None);
        assert!(
            !result.contains("@Check"),
            "unparsed @Check tag remained: {result}"
        );
        assert!(
            !result.contains("[["),
            "unparsed double-bracket remained: {result}"
        );
        assert!(
            result.contains("DC 20 Athletics"),
            "expected 'DC 20 Athletics' in: {result}"
        );
        assert!(
            result.contains("DC 20 Reflex"),
            "expected 'DC 20 Reflex' in: {result}"
        );
        assert!(
            result.contains("Immobilized"),
            "expected UUID to be resolved: {result}"
        );
        assert!(
            result.contains("DC 20"),
            "expected [[/act]] curly label in: {result}"
        );
    }
}
