use std::str::FromStr;

use serde::Deserialize;

use super::{AppealType, Appeals};

#[derive(Debug, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum Personality {
    #[default]
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

impl FromStr for Personality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "analytical" => Ok(Personality::Analytical),
            "emotional" => Ok(Personality::Emotional),
            "rational" => Ok(Personality::Rational),
            "dutiful" => Ok(Personality::Dutiful),
            "empathic" => Ok(Personality::Empathic),
            "principled" => Ok(Personality::Principled),
            _ => Err(format!("Unknown personality: {}", s)),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub enum PersuasionStatus {
    Convinced, // No additional info needed. Can't persuade.
    Persuading {
        resolve: i32,
        max_resolve: i32,
        personality: Personality,
        weakened: Vec<AppealType>,
    },
    Scrutinised {
        resolve: i32,
        max_resolve: i32,
        personality: Personality,
        weakened: Vec<AppealType>,
        unique: bool,
    },
    #[default]
    Unscrutinised,
    NonSentient,
}

impl PersuasionStatus {
    pub fn start_persuasion(&mut self) {
        match self {
            PersuasionStatus::Scrutinised {
                resolve,
                max_resolve,
                personality,
                weakened,
                ..
            } => {
                *self = PersuasionStatus::Persuading {
                    resolve: *resolve,
                    max_resolve: *max_resolve,
                    personality: *personality,
                    weakened: weakened.clone(),
                };
            }
            _ => {
                *self = PersuasionStatus::Persuading {
                    resolve: 10000,
                    max_resolve: 10000,
                    personality: Personality::Unknown,
                    weakened: vec![],
                };
            }
        }
    }

    pub fn value_influence_by_resolve(&self) -> i32 {
        match self {
            PersuasionStatus::Convinced => 0,
            PersuasionStatus::Persuading { resolve, .. } => *resolve * 5 / 105,
            PersuasionStatus::Scrutinised { resolve, .. } => *resolve * 5 / 105,
            PersuasionStatus::Unscrutinised => 0,
            PersuasionStatus::NonSentient => 0,
        }
    }

    pub fn resolve(&self) -> i32 {
        match self {
            PersuasionStatus::Convinced => 0,
            PersuasionStatus::Persuading { resolve, .. } => *resolve,
            PersuasionStatus::Scrutinised { resolve, .. } => *resolve,
            PersuasionStatus::Unscrutinised => 10000,
            PersuasionStatus::NonSentient => 0,
        }
    }

    pub fn max_resolve(&self) -> i32 {
        match self {
            PersuasionStatus::Convinced => 0,
            PersuasionStatus::Persuading { max_resolve, .. } => *max_resolve,
            PersuasionStatus::Scrutinised { max_resolve, .. } => *max_resolve,
            PersuasionStatus::Unscrutinised => 10000,
            PersuasionStatus::NonSentient => 0,
        }
    }

    pub fn resolve_affect(&mut self, amount: i32) {
        match self {
            PersuasionStatus::Convinced => {}
            PersuasionStatus::Persuading { resolve, .. } => {
                *resolve -= amount;
                if *resolve < 0 {
                    *resolve = 0;
                }
            }
            PersuasionStatus::Scrutinised { resolve, .. } => {
                *resolve -= amount;
                if *resolve < 0 {
                    *resolve = 0;
                }
            }
            PersuasionStatus::Unscrutinised => {}
            PersuasionStatus::NonSentient => {}
        }
    }

    pub fn personality(&self) -> Personality {
        match self {
            PersuasionStatus::Convinced => Personality::Unknown,
            PersuasionStatus::Persuading { personality, .. } => *personality,
            PersuasionStatus::Scrutinised { personality, .. } => *personality,
            PersuasionStatus::Unscrutinised => Personality::Unknown,
            PersuasionStatus::NonSentient => Personality::Unknown,
        }
    }
}
