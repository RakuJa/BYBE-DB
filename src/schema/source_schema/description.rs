use crate::utils::tag::tag_parser::clean_description_from_all_tags;
use std::fmt;
use std::fmt::Formatter;

pub struct Description {
    raw_description: String,
}

impl Description {
    pub fn is_valid(&self) -> bool {
        !self.raw_description.contains("@Embed")
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
        write!(
            f,
            "{}",
            clean_description_from_all_tags(self.raw_description.as_str(), None)
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
