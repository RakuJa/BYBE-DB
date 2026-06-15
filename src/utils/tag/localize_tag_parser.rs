use {once_cell::sync::Lazy, regex::Regex};

pub fn clean_description<F>(description: &str, lookup: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Localize\[([^\]]*)\]").unwrap());
    RE.replace_all(description, |caps: &regex::Captures| {
        lookup(&caps[1]).unwrap_or_default()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tag::tag_parser::lookup_path;
    use rstest::{fixture, rstest};
    use serde_json::Value;
    use serde_json::json;

    #[fixture]
    fn mock_json() -> Value {
        json!({
            "PF2E": {
                "NPC": {
                    "Abilities": {
                        "Glossary": {
                            "Grab": "Grab description text",
                            "SwallowWhole": "Swallow Whole description text"
                        }
                    }
                },
                "PersistentDamage": {
                    "Bleed1": {
                        "success": "1 persistent bleed damage"
                    }
                },
                "SpecificRule": {
                    "Foo": "Foo text",
                    "Bar": "Bar text"
                }
            }
        })
    }

    #[rstest]
    #[case("@Localize[PF2E.NPC.Abilities.Glossary.Grab]", "Grab description text")]
    #[case("@Localize[PF2E.AttackEffectGrab]", "")]
    fn remove_bare_tag(mock_json: Value, #[case] input: &str, #[case] expected: &str) {
        let result = clean_description(input, |path| lookup_path(&mock_json, path));
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(
        "<p>@Localize[PF2E.NPC.Abilities.Glossary.SwallowWhole]</p>",
        "<p>Swallow Whole description text</p>"
    )]
    fn remove_tag_sole_paragraph_content(
        mock_json: Value,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        let result = clean_description(input, |path| lookup_path(&mock_json, path));
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(
        "You take @Localize[PF2E.PersistentDamage.Bleed1.success].",
        "You take 1 persistent bleed damage."
    )]
    #[case(
        "<strong>text</strong> @Localize[PF2E.NPC.Abilities.Glossary.Grab] more",
        "<strong>text</strong> Grab description text more"
    )]
    fn remove_inline_tag(mock_json: Value, #[case] input: &str, #[case] expected: &str) {
        let result = clean_description(input, |path| lookup_path(&mock_json, path));
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(
        "@Localize[PF2E.SpecificRule.Foo]\n@Localize[PF2E.SpecificRule.Bar]",
        "Foo text\nBar text"
    )]
    fn remove_multiple_tags(mock_json: Value, #[case] input: &str, #[case] expected: &str) {
        let result = clean_description(input, |path| lookup_path(&mock_json, path));
        assert_eq!(expected, result);
    }

    #[rstest]
    fn merged_lookup_overrides_pf2e(mock_json: Value) {
        let sf2e_json = json!({
            "PF2E": {
                "NPC": {
                    "Abilities": {
                        "Glossary": {
                            "Grab": "SF2E Grab override"
                        }
                    }
                }
            }
        });

        let lookup =
            |path: &str| lookup_path(&sf2e_json, path).or_else(|| lookup_path(&mock_json, path));

        assert_eq!(
            "SF2E Grab override",
            clean_description("@Localize[PF2E.NPC.Abilities.Glossary.Grab]", lookup)
        );
        assert_eq!(
            "1 persistent bleed damage",
            clean_description("@Localize[PF2E.PersistentDamage.Bleed1.success]", lookup)
        );
    }
}
