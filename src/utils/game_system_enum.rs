use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum GameSystem {
    Pathfinder2e,
    Starfinder2e,
}

impl Display for GameSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pathfinder2e => "pf2e",
                Self::Starfinder2e => "sf2e",
            }
        )
    }
}
