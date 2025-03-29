use crate::utils::tag::tag_parser::get_content_inside_square_brackets;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    _clean_description_from_compendium_tag(description)
}

fn _clean_description_from_compendium_tag(description: &str) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Template\[(\S*)](\{.*?})?").unwrap());

    let mut clean_description = String::from(description);
    for el in RE.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = RE.captures(el) {
            let raw_compendium_descr = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let square_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");
            let curly_content = curr_match
                .get(2)
                .map(|x| x.as_str())
                .unwrap_or("")
                .replace(&['{', '}'][..], "");
            let clean_data = if curly_content.is_empty() {
                let range_type = get_content_inside_square_brackets(square_content, "type:");
                let distance = get_content_inside_square_brackets(square_content, "distance:");
                // descriptions seems to clash with this, (ex: 10width it already says in the description
                // that the line extends in the left or right square)
                let _width = get_content_inside_square_brackets(square_content, "width:");
                let traits = get_content_inside_square_brackets(square_content, "traits:");
                let mut template_data = format!("{distance}-foot {range_type}",);
                if !traits.is_empty() {
                    template_data.push_str(" [");
                    template_data.push_str(traits.as_str());
                    template_data.push_str(" ]");
                }
                template_data
            } else {
                curly_content
            };

            clean_description =
                clean_description.replace(raw_compendium_descr, clean_data.as_str());
        }
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
}
