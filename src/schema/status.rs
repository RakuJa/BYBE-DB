use crate::schema::source_schema::item::source_item::SourceItem;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Status {
    #[default]
    Valid,
    Archived,
    Deprecated,
}

impl From<&SourceItem> for Status {
    fn from(item: &SourceItem) -> Self {
        match item.description.is_valid() {
            true => Status::Valid,
            false => Status::Deprecated,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Valid => "valid",
                Self::Archived => "archived",
                Self::Deprecated => "deprecated",
            }
        )
    }
}
