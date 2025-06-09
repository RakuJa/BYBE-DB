// [[/gmr 1d6 #Minutes]]{1d6 minutes}.

use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\[([^\[\]]*)]](\{(.*)})?").unwrap());
    for el in SPLIT_REGEX.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = SPLIT_REGEX.captures(el) {
            let dirty_match = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let cleaned_match =
                if let Some(curly_bracket_content) = curr_match.get(3).map(|x| x.as_str()) {
                    curly_bracket_content.to_string()
                } else if let Some(square_bracket_content) = curr_match.get(1).map(|x| x.as_str()) {
                    clean_description_without_curly_brackets(square_bracket_content)
                } else {
                    dirty_match.to_string()
                };
            desc = desc.replacen(dirty_match, cleaned_match.as_str(), 1);
        }
    }
    desc
}

pub fn clean_description_without_curly_brackets(description: &str) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"/gmr ([0-9]+d[0-9]+).*").unwrap());
    for el in SPLIT_REGEX.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = SPLIT_REGEX.captures(el) {
            let dirty_match = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let cleaned_match =
                if let Some(curly_bracket_content) = curr_match.get(1).map(|x| x.as_str()) {
                    curly_bracket_content
                } else {
                    dirty_match
                };
            desc = desc.replacen(dirty_match, cleaned_match, 1);
        }
    }
    desc
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
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
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    #[rstest]
    #[case("[[[/gmr 1d6 #Minutes]]{1d6 minutes}.", "[1d6 minutes.")]
    fn clean_dirty_description(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
    #[rstest]
    #[case("1d6 minutes", "1d6 minutes")]
    fn clean_without_tags(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
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
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
}
