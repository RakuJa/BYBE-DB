// [[/gmr 1d6 #Minutes]]{1d6 minutes}.

use once_cell::sync::Lazy;
use regex::Regex;

pub fn clean_description(description: &str) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[\[([^\[\]]*)]](\{(.*?)})?").unwrap());
    for el in SPLIT_REGEX.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = SPLIT_REGEX.captures(el) {
            let dirty_match = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let square_bracket_content = curr_match.get(1).map(|x| x.as_str()).unwrap_or("");
            let is_r_command = square_bracket_content.trim_start().starts_with("/r");

            let cleaned_match = if is_r_command {
                clean_description_without_curly_brackets(square_bracket_content)
                    .trim()
                    .to_string()
            } else if let Some(curly_bracket_content) = curr_match.get(3).map(|x| x.as_str()) {
                curly_bracket_content.to_string()
            } else {
                clean_description_without_curly_brackets(square_bracket_content)
            };

            let match_pos = desc.find(dirty_match);
            let needs_space = is_r_command
                && match_pos
                    .and_then(|pos| desc[..pos].chars().last())
                    .map(|c| c.is_alphanumeric() || c == '_')
                    .unwrap_or(false);

            let replacement = if needs_space {
                format!(" {cleaned_match}")
            } else {
                cleaned_match
            };

            desc = desc.replacen(dirty_match, replacement.as_str(), 1);
        }
    }
    desc
}

pub fn clean_description_without_curly_brackets(description: &str) -> String {
    let mut desc = String::from(description);
    static SPLIT_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"/(?:gmr|r) ([0-9]+d[0-9]+[+\-]?[0-9]*)(?:\s.*)?").unwrap());
    for el in SPLIT_REGEX.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = SPLIT_REGEX.captures(el) {
            let dirty_match = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let cleaned_match = if let Some(dice_content) = curr_match.get(1).map(|x| x.as_str()) {
                dice_content
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
    // minimal /r roll — no label, no curly bracket
    #[rstest]
    #[case("[[/r 1d6]]", "1d6")]
    #[case("[[/r 2d8+3]]", "2d8+3")]
    #[case("[[/r 1d20-2]]", "1d20-2")]
    fn clean_bare_r_roll(#[case] input: &str, #[case] expected: &str) {
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }

    // /gmr with a numeric modifier in the curly label
    #[rstest]
    #[case("[[/gmr 1d4+2 #Recharge]]{1d4+2 rounds}", "1d4+2 rounds")]
    #[case("[[/gmr 2d6+1 #Duration]]{2d6+1 minutes}", "2d6+1 minutes")]
    fn clean_gmr_with_modifier(#[case] input: &str, #[case] expected: &str) {
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
        let parsed_description = clean_description(input);
        assert_eq!(expected, parsed_description);
    }
}
