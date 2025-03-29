use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    _clean_description_from_compendium_tag(description)
}

fn _clean_description_from_compendium_tag(description: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"@UUID(\[Compendium\..*?])(\{.*?})?").unwrap());
    let mut clean_description = String::from(description);
    for el in RE.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = RE.captures(el) {
            let raw_compendium_descr = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let compendium_descr =
                if let Some(curly_brackets_content) = curr_match.get(2).map(|x| x.as_str()) {
                    curly_brackets_content.replace(&['{', '}'][..], "")
                } else {
                    let mut descr = String::new();
                    if let Some(inner_descr) = curr_match.get(1).map(|x| x.as_str()) {
                        let clean_inner_descr = inner_descr.replace(&['[', ']'][..], "");
                        match clean_inner_descr.rsplit_once('.') {
                            None => descr.push_str(clean_inner_descr.as_str()),
                            Some(v) => descr.push_str(v.1),
                        }
                    }
                    descr
                };
            clean_description =
                clean_description.replace(raw_compendium_descr, compendium_descr.as_str());
        }
    }
    clean_description
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "@UUID[Compendium.pf2e.equipment-srd.Item.Thieves' Toolkit]",
        "Thieves' Toolkit"
    )]
    #[case(
        "@UUID[Compendium.pf2e.actionspf2e.Item.Disable a Device]",
        "Disable a Device"
    )]
    #[case("@UUID[Compendium.pf2e.actionspf2e.Item.Pick a Lock]", "Pick a Lock")]
    #[case("@UUID[Compendium.pf2e.feats-srd.Item.Shield Block]", "Shield Block")]
    fn clean_compendium_with_only_square_brackets(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@UUID[Compendium.pf2e.actionspf2e.Item.Pick a Lock]{Pick the Lock}",
        "Pick the Lock"
    )]
    fn clean_compendium_with_curly_brackets(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case(
        "@UUID[Compendium.pf2e.actionspf2e.Item.Pick a Lock]{Pick the Lock} @UUID[Compendium.pf2e.actionspf2e.Item.Pick a Lock]{Pick the Lock}",
        "Pick the Lock Pick the Lock"
    )]
    fn clean_compendium_with_curly_brackets_and_multiple_compendium_tag(
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
}
