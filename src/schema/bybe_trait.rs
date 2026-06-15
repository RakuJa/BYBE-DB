use crate::schema::localize_loader;
use crate::utils::game_system_enum::GameSystem;
use anyhow::anyhow;
use bon::bon;
use capitalize::Capitalize;

pub struct Trait {
    pub name: String,
    pub description: Option<String>,
}

#[bon]
impl Trait {
    #[builder]
    pub fn new(name: &str, game_system: &GameSystem) -> Self {
        Self {
            description: fetch_trait_description(name, game_system).ok(),
            name: name.to_string(),
        }
    }
}

pub fn fetch_trait_description(name: &str, gs: &GameSystem) -> anyhow::Result<String> {
    let c_name = sanitize_trait_name(name);
    let find_key = format!("TraitDescription{c_name}");

    let pf2e_result = localize_loader::lang_data()
        .get("PF2E")
        .ok_or_else(|| anyhow!("Could not find PF2E"))?
        .as_object()
        .ok_or_else(|| anyhow!("PF2E object is not an object"))?
        .get(&find_key)
        .ok_or_else(|| anyhow!("Cannot find the key {find_key}"))?
        .as_str()
        .ok_or_else(|| anyhow!("Cannot convert description of key {find_key} to string"))?
        .to_string();

    Ok(if matches!(gs, GameSystem::Starfinder) {
        if let Some(sf2e_value) = localize_loader::sf2e_data()
            .get("PF2E")
            .and_then(|v| v.as_object())
            .and_then(|obj| obj.get(&find_key))
            .and_then(|v| v.as_str())
        {
            sf2e_value.to_string()
        } else {
            pf2e_result
        }
    } else {
        pf2e_result
    })
}

fn sanitize_trait_name(name: &str) -> String {
    name.replace('-', " ")
        .split_whitespace()
        .map(|word| word.capitalize())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bob", "Bob")]
    #[case("alice", "Alice")]
    #[case("x", "X")]
    fn single_word(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize_trait_name(input), expected);
    }

    #[rstest]
    #[case("bob has", "BobHas")]
    #[case("alice cooper", "AliceCooper")]
    #[case("john doe smith", "JohnDoeSmith")]
    fn multiple_words(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize_trait_name(input), expected);
    }

    #[rstest]
    #[case("bob-has", "BobHas")]
    #[case("alice-cooper", "AliceCooper")]
    #[case("john-doe-smith", "JohnDoeSmith")]
    fn dashed_input(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize_trait_name(input), expected);
    }

    #[rstest]
    #[case("bob-has fun", "BobHasFun")]
    #[case("  bob   has  ", "BobHas")]
    #[case("bob--has", "BobHas")]
    fn mixed_and_edge_cases(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize_trait_name(input), expected);
    }

    #[rstest]
    #[case("", "")]
    #[case("-", "")]
    #[case("   ", "")]
    fn empty_and_trivial(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(sanitize_trait_name(input), expected);
    }
}
