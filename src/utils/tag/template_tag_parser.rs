use crate::utils::tag::tag_parser::get_content_inside_square_brackets;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Template\[([^\]]*)\](\{.*?})?").unwrap());

    let mut clean_description = String::from(description);
    for curr_match in RE.captures_iter(description) {
        let raw_descr = curr_match.get(0).unwrap().as_str();
        let square_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");
        let curly_content = curr_match
            .get(2)
            .map(|x| x.as_str())
            .unwrap_or("")
            .replace(&['{', '}'][..], "");

        let clean_data = if curly_content.is_empty() {
            // Prefer explicit "type:" key; fall back to the bare first pipe-segment for
            // newer data that omits the key (e.g. @Template[emanation|distance:20]).
            let range_type = {
                let t = get_content_inside_square_brackets(square_content, "type:");
                if !t.is_empty() {
                    t
                } else {
                    square_content.split('|').next().unwrap_or("").to_string()
                }
            };
            let distance = get_content_inside_square_brackets(square_content, "distance:");
            let traits = get_content_inside_square_brackets(square_content, "traits:");
            let mut template_data = format!("{distance}-foot {range_type}");
            if !traits.is_empty() {
                template_data.push_str(" [");
                template_data.push_str(&traits);
                template_data.push_str(" ]");
            }
            template_data
        } else {
            curly_content
        };

        clean_description = clean_description.replace(raw_descr, &clean_data);
    }
    clean_description
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("@Template[type:emanation|distance:10]", "10-foot emanation")]
    #[case("@Template[type:burst|distance:30]", "30-foot burst")]
    #[case("@Template[type:burst|distance:5]", "5-foot burst")]
    #[case("@Template[type:line|distance:60]", "60-foot line")]
    #[case("@Template[type:cone|distance:40]", "40-foot cone")]
    fn clean_compendium_with_type_and_distance(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("@Template[type:emanation|distance:10]end", "10-foot emanationend")]
    #[case("beg @Template[type:burst|distance:30] end", "beg 30-foot burst end")]
    #[case("beg @Template[type:burst|distance:5] end", "beg 5-foot burst end")]
    #[case("beg @Template[type:line|distance:60] end", "beg 60-foot line end")]
    #[case("beg @Template[type:cone|distance:40] end", "beg 40-foot cone end")]
    fn clean_compendium_with_type_and_distance_and_dirty_description(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Template[type:burst|distance:5] you in a @Template[type:line|distance:60].",
        "5-foot burst you in a 60-foot line."
    )]
    fn clean_compendium_with_type_and_distance_and_multiple_template_tag(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Template[type:line|distance:120|width:10]{120-foot lines}",
        "120-foot lines"
    )]
    #[case("@Template[type:burst|distance:10]{10 feet}", "10 feet")]
    fn clean_compendium_with_curly_brackets(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@Template[type:burst|distance:5]{5-foot radius} @Template[type:burst|distance:10]{10 feet}",
        "5-foot radius 10 feet"
    )]
    #[case(
        "@Template[type:burst|distance:5]{5-foot radius}@Template[type:burst|distance:10]{10 feet}",
        "5-foot radius10 feet"
    )]
    fn clean_compendium_with_multiple_curly_brackets(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    // traits field appends a bracketed list when no curly label is present
    #[rstest]
    #[case("@Template[type:cone|distance:40|traits:fire]", "40-foot cone [fire ]")]
    #[case(
        "@Template[type:burst|distance:10|traits:cold,water]",
        "10-foot burst [cold,water ]"
    )]
    fn clean_template_with_traits(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
    
    #[rstest]
    #[case("@Template[emanation|distance:20]", "20-foot emanation")]
    #[case("@Template[burst|distance:30]", "30-foot burst")]
    #[case("@Template[emanation|distance:40]", "40-foot emanation")]
    #[case("@Template[emanation|distance:60]", "60-foot emanation")]
    fn clean_template_bare_type(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
    
    #[rstest]
    #[case(
        "@Template[burst|distance:20|name:Black Tentacles|traits:hazard,complex,magical,mechanical,trap,conjuration,occult]",
        "20-foot burst [hazard,complex,magical,mechanical,trap,conjuration,occult ]"
    )]
    #[case(
        "@Template[line|distance:30|traits:hazard,complex,magical,mechanical,trap,evocation,occult,shadow|width:5]",
        "30-foot line [hazard,complex,magical,mechanical,trap,evocation,occult,shadow ]"
    )]
    #[case(
        "@Template[burst|distance:10|traits:hazard,complex,magical,mechanical,trap,evocation,occult,sonic]",
        "10-foot burst [hazard,complex,magical,mechanical,trap,evocation,occult,sonic ]"
    )]
    fn clean_template_with_name_and_spaces(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
}
