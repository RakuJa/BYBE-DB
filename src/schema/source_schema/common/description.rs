use crate::game_system_handler::current_game_system;
use crate::schema::localize_loader;
use crate::utils::game_system_enum::GameSystem;
use crate::utils::tag::tag_parser::clean_description_from_all_tags;
#[cfg(feature = "dry-run")]
use crate::utils::tag::tag_parser::find_remaining_tags;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Description {
    raw_description: String,
}

impl Description {
    pub fn is_valid(&self) -> bool {
        !self.raw_description.contains("@Embed")
    }

    #[cfg(feature = "dry-run")]
    pub fn parsing_errors(&self, item_lvl: Option<i64>) -> Vec<String> {
        let json_data = match current_game_system() {
            GameSystem::Pathfinder => localize_loader::lang_data(),
            GameSystem::Starfinder => localize_loader::sf2e_data(),
        };
        let cleaned = clean_description_from_all_tags(&self.raw_description, item_lvl, json_data);
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

impl fmt::Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let json_data = match current_game_system() {
            GameSystem::Pathfinder => localize_loader::lang_data(),
            GameSystem::Starfinder => localize_loader::sf2e_data(),
        };

        write!(
            f,
            "{}",
            clean_description_from_all_tags(self.raw_description.as_str(), None, json_data)
        )
    }
}

impl Default for Description {
    fn default() -> Self {
        Description {
            raw_description: "".to_string(),
        }
    }
}
