use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::Display;

#[derive(Serialize, Deserialize, Display, Default, Clone)]
pub enum RarityEnum {
    #[default]
    #[serde(alias = "common", alias = "COMMON")]
    Common,
    #[serde(alias = "uncommon", alias = "UNCOMMON")]
    Uncommon,
    #[serde(alias = "rare", alias = "RARE")]
    Rare,
    #[serde(alias = "unique", alias = "UNIQUE")]
    Unique,
}

impl FromStr for RarityEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "COMMON" => Ok(RarityEnum::Common),
            "UNCOMMON" => Ok(RarityEnum::Uncommon),
            "RARE" => Ok(RarityEnum::Rare),
            "UNIQUE" => Ok(RarityEnum::Unique),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Display, Default, Clone)]
pub enum SizeEnum {
    #[serde(alias = "tiny", alias = "TINY")]
    Tiny,
    #[serde(alias = "small", alias = "SMALL")]
    Small,
    #[serde(alias = "medium", alias = "MEDIUM")]
    #[default]
    Medium,
    #[serde(alias = "large", alias = "LARGE")]
    Large,
    #[serde(alias = "huge", alias = "HUGE")]
    Huge,
    #[serde(alias = "gargantuan", alias = "GARGANTUAN")]
    Gargantuan,
}

impl FromStr for SizeEnum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TINY" => Ok(SizeEnum::Tiny),
            "SMALL" | "SM" => Ok(SizeEnum::Small),
            "MEDIUM" | "MED" => Ok(SizeEnum::Medium),
            "LARGE" | "LG" => Ok(SizeEnum::Large),
            "HUGE" => Ok(SizeEnum::Huge),
            "GARGANTUAN" | "GRG" => Ok(SizeEnum::Gargantuan),
            _ => Err(()),
        }
    }
}
