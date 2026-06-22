use crate::schema::localize_loader;
use crate::schema::localize_loader::{lookup_path, parsed_traits};
use crate::utils::game_system_enum::GameSystem;
use anyhow::anyhow;
use bon::bon;

pub struct Trait {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[bon]
impl Trait {
    #[builder]
    pub fn new(name: &str, game_system: &GameSystem) -> Self {
        Self {
            display_name: fetch_clean_trait_string(name, "trait", game_system).ok(),
            description: fetch_clean_trait_string(name, "description", game_system).ok(),
            name: name.to_string(),
        }
    }
}

pub fn fetch_clean_trait_string(name: &str, kind: &str, gs: &GameSystem) -> anyhow::Result<String> {
    let traits = parsed_traits();
    let dot_path = traits
        .get(name)
        .and_then(|m| m.get(kind))
        .ok_or_else(|| anyhow!("no trait found for {}/{}", name, kind))?
        .clone();

    let sf2e_result = matches!(gs, GameSystem::Starfinder)
        .then(|| lookup_path(localize_loader::sf2e_data(), &dot_path))
        .flatten();

    sf2e_result
        .or_else(|| lookup_path(localize_loader::lang_data(), &dot_path))
        .ok_or_else(|| anyhow!("no data found for {} in both sf2e and pf2e", name))
}
