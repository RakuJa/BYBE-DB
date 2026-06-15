use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug, Default)]
pub enum GameSystem {
    #[default]
    Pathfinder,
    Starfinder,
}

impl Display for GameSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pathfinder => "pf",
                Self::Starfinder => "sf",
            }
        )
    }
}
