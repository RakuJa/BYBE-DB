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
            let check_type = if !is_unsolvable_tag && curly_content.is_empty() {
                get_content_inside_square_brackets(square_content, "type:").capitalize()
            } else {
                "".to_string()
            };
            let dc_value = if !is_unsolvable_tag && !dc.is_empty() {
                let dc_value = dc.trim();//.parse::<i64>().unwrap();
                /* we should handle substitutions
                .unwrap_or_else(|_| {
                    0 //convert string with tags into value (substitution))}
                });
                 */
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
            let check_data = format!("{dc_value}{dc_type}{check_type}{curly_content}")
                .trim()
                .to_string();
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
}
