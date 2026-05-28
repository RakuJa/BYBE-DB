use {once_cell::sync::Lazy, regex::Regex};

pub fn clean_description(description: &str) -> String {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"@Localize\[[^\]]*\]").unwrap());
    RE.replace_all(description, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("@Localize[PF2E.NPC.Abilities.Glossary.Grab]", "")]
    #[case("@Localize[PF2E.AttackEffectGrab]", "")]
    fn remove_bare_tag(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, clean_description(input));
    }

    #[rstest]
    #[case(
        "<p>@Localize[PF2E.NPC.Abilities.Glossary.SwallowWhole]</p>",
        "<p></p>"
    )]
    fn remove_tag_sole_paragraph_content(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, clean_description(input));
    }

    #[rstest]
    #[case(
        "You take @Localize[PF2E.PersistentDamage.Bleed1.success].",
        "You take ."
    )]
    #[case(
        "<strong>text</strong> @Localize[PF2E.NPC.Abilities.Glossary.Grab] more",
        "<strong>text</strong>  more"
    )]
    fn remove_inline_tag(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, clean_description(input));
    }

    #[rstest]
    #[case(
        "@Localize[PF2E.SpecificRule.Foo]\n@Localize[PF2E.SpecificRule.Bar]",
        "\n"
    )]
    fn remove_multiple_tags(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, clean_description(input));
    }
}
