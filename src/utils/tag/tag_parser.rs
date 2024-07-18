use crate::utils::tag::{
    check_tag_parser, compendium_tag_parser, dmg_tag_parser, template_tag_parser,
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
    for el in RE.find_iter(description).map(|x| x.as_str()) {
        if let Some(curr_match) = RE.captures(el) {
            let raw_descr = curr_match.get(0).map(|x| x.as_str()).unwrap();
            let clean_data = if let Some(curly_content) = curr_match.get(2).map(|x| x.as_str()) {
                curly_content
            } else if let Some(base_content) = curr_match.get(1).map(|x| x.as_str()) {
                base_content
            } else {
                raw_descr
            };
            clean_description = clean_description.replace(raw_descr, clean_data);
        }
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
        "".to_string()
    }
}

pub fn clean_description_from_all_tags(description: &str, item_lvl: Option<i64>) -> String {
    clean_description_from_generic_bracket(
        template_tag_parser::clean_description(
            check_tag_parser::clean_description(
                dmg_tag_parser::clean_description(
                    compendium_tag_parser::clean_description(description).as_str(),
                    item_lvl,
                )
                .as_str(),
            )
            .as_str(),
        )
        .as_str(),
    )
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
}
