use serde::Deserialize;

use super::{AppealType, Appeals};

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum Personality {
    Unknown,
    Analytical, // +Ethos -logos
    Emotional,  // +pathos -logos
    Rational,   // +Logos -pathos
    Dutiful,    // +pathos -ethos
    Empathic,   // +ethos -pathos
    Principled, // +logos -ethos
}

impl Personality {
    pub fn is_weak_to(&self, appeal: Appeals) -> bool {
        match self {
            Personality::Analytical => appeal.is_ethos(),
            Personality::Emotional => appeal.is_pathos(),
            Personality::Rational => appeal.is_logos(),
            Personality::Dutiful => appeal.is_pathos(),
            Personality::Empathic => appeal.is_ethos(),
            Personality::Principled => appeal.is_logos(),
            _ => false,
        }
    }

    pub fn is_strong_to(&self, appeal: Appeals) -> bool {
        match self {
            Personality::Analytical => appeal.is_logos(),
            Personality::Emotional => appeal.is_logos(),
            Personality::Rational => appeal.is_pathos(),
            Personality::Dutiful => appeal.is_ethos(),
            Personality::Empathic => appeal.is_pathos(),
            Personality::Principled => appeal.is_ethos(),
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub enum PersuasionStatus {
    Convinced, // No additional info needed. Can't persuade.
    Persuading {
        resolve: i64,
        personality: Personality,
        weakened: Vec<AppealType>,
    },
    Scrutinised {
        resolve: i64,
        personality: Personality,
        weakened: Vec<AppealType>,
    },
    #[default]
    Unscrutinised,
    NonSentient,
}

impl PersuasionStatus {
    pub fn value_influence_by_resolve(&self) -> i64 {
        match self {
            PersuasionStatus::Convinced => 0,
            PersuasionStatus::Persuading { resolve, .. } => *resolve * 5 / 105,
            PersuasionStatus::Scrutinised { resolve, .. } => *resolve * 5 / 105,
            PersuasionStatus::Unscrutinised => 0,
            PersuasionStatus::NonSentient => 0,
        }
    }
}
