use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum GameSystem {
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
