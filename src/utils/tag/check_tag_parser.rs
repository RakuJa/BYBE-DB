use crate::utils::tag::tag_parser::get_content_inside_square_brackets;
use capitalize::Capitalize;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    // Disallow [ and ] in content so a nested @Check tag is never consumed by the outer match.
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Check\[([^\[\]]*)\](\{[^}]*\})?").unwrap());
    // Fallback for data where the closing ] is missing: match @Check[skill|dc:NUMBER followed by
    // a character that is not another digit, | or ] (i.e. the tag was never closed).
    // The trailing character is captured so it can be preserved in the replacement.
    static BROKEN_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"@Check\[([^|\[\]]+)\|dc:(\d+)([^\d|\]])").unwrap());

    let mut clean_description = String::from(description);

    // First pass: well-formed @Check[...] tags.
    for curr_match in RE.captures_iter(description) {
        let raw_check_descr = curr_match.get(0).unwrap().as_str();
        let square_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");

        let dc = get_content_inside_square_brackets(square_content, "dc:");
        let is_unsolvable_tag = dc.contains("resolve");

        let curly_content = if is_unsolvable_tag {
            String::new()
        } else {
            curr_match
                .get(2)
                .map(|x| x.as_str())
                .unwrap_or("")
                .replace(&['{', '}'][..], "")
        };

        let raw_check_type = get_content_inside_square_brackets(square_content, "type:");
        // If no "type:" key exists, the first pipe-separated segment is the bare skill name
        let (check_type, bare_skill) = if !is_unsolvable_tag && curly_content.is_empty() {
            if !raw_check_type.is_empty() {
                (raw_check_type.capitalize(), String::new())
            } else {
                let bare = square_content
                    .split('|')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                (String::new(), bare)
            }
        } else {
            (String::new(), String::new())
        };

        let dc_value = if !is_unsolvable_tag && !dc.is_empty() {
            format!("DC {} ", dc.trim())
        } else {
            String::new()
        };
        // If curly brackets are present we do not need this information, it's redundant
        let dc_type = if !is_unsolvable_tag
            && curly_content.is_empty()
            && get_content_inside_square_brackets(square_content, "basic:") == "true"
        {
            "basic "
        } else {
            ""
        };

        // For bare skill (no "type:" prefix): skill name comes before DC
        let check_data = if !bare_skill.is_empty() {
            format!("{bare_skill} {dc_value}{dc_type}{check_type}{curly_content}")
        } else {
            format!("{dc_value}{dc_type}{check_type}{curly_content}")
        }
        .trim()
        .to_string();

        clean_description = clean_description.replace(raw_check_descr, &check_data);
    }

    // Second pass: broken tags where the closing ] is absent (malformed source data).
    // After the first pass the inner @Check tags are already resolved, so only the unclosed
    // outer tag remains. Extract just the skill name and DC value from it.
    let intermediate = clean_description.clone();
    for curr_match in BROKEN_RE.captures_iter(&intermediate) {
        let raw_match = curr_match.get(0).unwrap().as_str();
        let skill = curr_match.get(1).map(|x| x.as_str()).unwrap_or("").trim();
        let dc = curr_match.get(2).map(|x| x.as_str()).unwrap_or("");
        let trailing = curr_match.get(3).map(|x| x.as_str()).unwrap_or("");
        clean_description =
            clean_description.replacen(raw_match, &format!("{skill} DC {dc}{trailing}"), 1);
    }

    clean_description
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("@Check[type:reflex|dc:16]", "DC 16 Reflex")]
    #[case("@Check[type:will|dc:20]", "DC 20 Will")]
    #[case("@Check[type:fortitude|dc:25]", "DC 25 Fortitude")]
    fn clean_check_with_type_and_dc(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Check[type:reflex|dc:23|basic:true]", "DC 23 basic Reflex")]
    #[case("@Check[type:reflex|dc:23|basic:false]", "DC 23 Reflex")]
    #[case("@Check[type:reflex|dc:23|basic:something]", "DC 23 Reflex")]
    fn clean_check_with_type_and_dc_and_basic(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Check[type:reflex|dc:23|basic:true]{basic Reflex save}",
        "DC 23 basic Reflex save"
    )]
    #[case(
        "@Check[type:reflex|dc:23|basic:false]{basic Reflex save}",
        "DC 23 basic Reflex save"
    )]
    #[case(
        "@Check[type:reflex|dc:23|basic:something]{basic Reflex save}",
        "DC 23 basic Reflex save"
    )]
    fn clean_check_with_type_and_dc_and_basic_and_curly_bracket(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Check[type:reflex|dc:27|basic:true|name:Sizzlers Follypop|traits:uncommon,electricity|showDC:all]",
        "DC 27 basic Reflex"
    )]
    fn clean_check_with_type_and_dc_and_basic_and_curly_bracket_and_name_with_blank_spaces(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
    // bare skill name with |dc: — no "type:" prefix
    #[rstest]
    #[case("@Check[will|dc:18]", "will DC 18")]
    #[case("@Check[perception|dc:22]", "perception DC 22")]
    fn clean_check_bare_skill_with_dc(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    // unsolvable DCs containing "resolve" are dropped entirely
    #[rstest]
    #[case("@Check[type:reflex|dc:resolve(@actor.spellDC)]", "")]
    #[case("@Check[type:will|dc:resolve(@actor.attributes.spellDC.value)]", "")]
    fn clean_check_unsolvable_dc(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "<p>@Check[arcana|dc:40] (master) to unweave the magic left in the wake of the Trinity Star's passage, @Check[thievery|dc:43] (expert) to etch temporary runes to siphon away the magic</p>",
        "<p>arcana DC 40 (master) to unweave the magic left in the wake of the Trinity Star's passage, thievery DC 43 (expert) to etch temporary runes to siphon away the magic</p>"
    )]
    fn clean_check_with_two_skills(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "<p>@Check[Ocean Lore|dc:36] (expert) or @Check[nature|dc:38] or @Check[survival|dc:38] (master) to chart the flood path, @Check[crafting|dc:38] (master) to reinforce a shelter, or @Check[athletics|dc:40] (expert) to quickly ferry people or supplies to higher ground,</p>",
        "<p>Ocean Lore DC 36 (expert) or nature DC 38 or survival DC 38 (master) to chart the flood path, crafting DC 38 (master) to reinforce a shelter, or athletics DC 40 (expert) to quickly ferry people or supplies to higher ground,</p>"
    )]
    fn clean_megatsunami_check(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    // malformed data: first @Check tag is missing its closing ], causing the description text
    // and the second well-formed tag to be nested inside it in the raw source
    #[rstest]
    #[case(
        "<p>@Check[thievery|dc:28 (expert) to pick apart the pebbles, or @Check[warfare-lore|dc:24] (trained) to recognize and recreate the battle</p>",
        "<p>thievery DC 28 (expert) to pick apart the pebbles, or warfare-lore DC 24 (trained) to recognize and recreate the battle</p>"
    )]
    fn clean_check_with_missing_closing_bracket(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    // two adjacent @Check tags with curly labels on the same line must not
    // swallow both tags in one match
    #[rstest]
    #[case(
        "@Check[athletics|dc:20]{Athletics} check or @Check[reflex|dc:20]{Reflex} save against DC 20",
        "DC 20 Athletics check or DC 20 Reflex save against DC 20"
    )]
    #[case(
        "@Check[will|dc:24]{Will} save or @Check[fortitude|dc:24]{Fortitude} save",
        "DC 24 Will save or DC 24 Fortitude save"
    )]
    fn clean_check_two_adjacent_curly(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
}
