use crate::game_system_handler::current_game_system;
use crate::schema::localize_loader;
use crate::utils::game_system_enum::GameSystem;
use crate::utils::tag::tag_parser::clean_description_from_all_tags;
#[cfg(feature = "dry-run")]
use crate::utils::tag::tag_parser::find_remaining_tags;

#[derive(Clone, Debug, PartialEq)]
pub struct Description {
    raw_description: String,
}

fn resolve_localization(source: &str, item_lvl: Option<i64>) -> String {
    let gs = current_game_system();
    let lookup = move |path: &str| -> Option<String> {
        if matches!(gs, GameSystem::Starfinder)
            && let Some(sf2e_value) =
                localize_loader::lookup_path(localize_loader::sf2e_data(), path)
        {
            return Some(sf2e_value);
        }
        // `/act` action titles live in their own action-en.json, not en.json.
        localize_loader::lookup_path(localize_loader::lang_data(), path)
            .or_else(|| localize_loader::lookup_path(localize_loader::action_lang_data(), path))
    };
    clean_description_from_all_tags(source, item_lvl, lookup, localize_loader::parsed_actions())
}

impl Description {
    pub fn is_valid(&self) -> bool {
        !self.raw_description.contains("@Embed")
    }

    #[cfg(feature = "dry-run")]
    pub fn parsing_errors(&self, item_lvl: Option<i64>) -> Vec<String> {
        let cleaned = resolve_localization(&self.raw_description.as_str(), item_lvl);
        find_remaining_tags(&cleaned)
    }
}

impl From<&str> for Description {
    fn from(description: &str) -> Self {
        Description {
            raw_description: description.to_string(),
        }
    }
}

impl From<String> for Description {
    fn from(raw_description: String) -> Self {
        Description { raw_description }
    }
}

impl Description {
    pub fn resolve(&self, item_lvl: Option<i64>) -> String {
        resolve_localization(self.raw_description.as_str(), item_lvl)
    }
}

impl Default for Description {
    fn default() -> Self {
        Description {
            raw_description: "".to_string(),
        }
    }
}
