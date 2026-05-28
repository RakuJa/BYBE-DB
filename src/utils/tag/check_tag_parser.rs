use crate::utils::tag::tag_parser::get_content_inside_square_brackets;
use capitalize::Capitalize;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    _clean_description_from_check_tag(description)
}

fn _clean_description_from_check_tag(description: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Check\[([\w\W]*?)](\{.*})?").unwrap());

    let mut clean_description = String::from(description);
    for el in RE.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = RE.captures(el) {
            let raw_check_descr = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let square_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");

            let dc = get_content_inside_square_brackets(square_content, "dc:");
            let is_unsolvable_tag = dc.contains("resolve"); //|| dc.contains("@");

            let curly_content = if is_unsolvable_tag {
                "".to_string()
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
                    (raw_check_type.capitalize(), "".to_string())
                } else {
                    let bare = square_content
                        .split('|')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    ("".to_string(), bare)
                }
            } else {
                ("".to_string(), "".to_string())
            };

            let dc_value = if !is_unsolvable_tag && !dc.is_empty() {
                let dc_value = dc.trim();
                format!("DC {dc_value} ")
            } else {
                "".to_string()
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
                    .trim()
                    .to_string()
            } else {
                format!("{dc_value}{dc_type}{check_type}{curly_content}")
                    .trim()
                    .to_string()
            };

            clean_description = clean_description.replace(raw_check_descr, check_data.as_str());
        }
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
}
