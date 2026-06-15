use serde_json::Value;
use {once_cell::sync::Lazy, regex::Regex};

pub fn clean_description(description: &str, json_data: &Value) -> String {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"@Localize\[([^\]]*)\]").unwrap());
    RE.replace_all(description, |caps: &regex::Captures| {
        let path = &caps[1];
        let mut current = json_data;
        for key in path.split('.') {
            match current.get(key) {
                Some(value) => current = value,
                None => return String::new(),
            }
        }
        current.as_str().unwrap_or("").to_string()
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
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
        assert_eq!(expected, clean_description(input, &mock_json));
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
        assert_eq!(expected, clean_description(input, &mock_json));
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
        assert_eq!(expected, clean_description(input, &mock_json));
    }

    #[rstest]
    #[case(
        "@Localize[PF2E.SpecificRule.Foo]\n@Localize[PF2E.SpecificRule.Bar]",
        "Foo text\nBar text"
    )]
    fn remove_multiple_tags(mock_json: Value, #[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, clean_description(input, &mock_json));
    }
}
