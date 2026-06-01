use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"@UUID(\[Compendium\..*?])(\{.*?})?").unwrap());
    let mut clean_description = String::from(description);
    for curr_match in RE.captures_iter(description) {
        let raw_descr = curr_match.get(0).unwrap().as_str();
        let replacement = if let Some(curly) = curr_match.get(2).map(|x| x.as_str()) {
            curly.replace(&['{', '}'][..], "")
        } else {
            let inner = curr_match.get(1).map_or("", |m| m.as_str()).replace(&['[', ']'][..], "");
            match inner.rsplit_once('.') {
                Some((_, last)) => last.to_string(),
                None => inner,
            }
        };
        clean_description = clean_description.replace(raw_descr, &replacement);
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
