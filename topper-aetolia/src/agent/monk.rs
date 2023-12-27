use super::*;
use serde::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum MonkStance {
    #[default]
    None,
    Horse,
    Eagle,
    Cat,
    Bear,
    Rat,
    Scorpion,
    Cobra,
    Phoenix,
    Tiger,
    Wolf,
    Dragon,
}

impl MonkStance {
    pub fn from_name(name: &str) -> Self {
        match name {
            "horse" => MonkStance::Horse,
            "eagle" => MonkStance::Eagle,
            "cat" => MonkStance::Cat,
            "bear" => MonkStance::Bear,
            "rat" => MonkStance::Rat,
            "scorpion" => MonkStance::Scorpion,
            "cobra" => MonkStance::Cobra,
            "phoenix" => MonkStance::Phoenix,
            "tiger" => MonkStance::Tiger,
            "wolf" => MonkStance::Wolf,
            "dragon" => MonkStance::Dragon,
            _ => MonkStance::None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MonkClassState {
    pub stance: MonkStance,
    pub kai: CType,
}
