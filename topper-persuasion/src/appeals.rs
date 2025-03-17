use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::agent::PersuasionState;

use super::Personality;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AppealType {
    Ethos,
    Logos,
    Pathos,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Appeals {
    // Ethos
    Authority,  // Defense buff
    Morality,   // Hand buff
    Reputation, // Influence
    Tradition,  // Cure negatives

    // Logos
    Evidence,  // Next appeal Weakness
    Reason,    // Appeal buff
    Analogy,   // Buff ethos/pathos
    Causality, // Crits

    // Pathos
    Intimidation, // Prevent afflictions
    Reassurance,  // Acumen regen
    Inspiration,  // Cheap attacks
    Provocation,  // Big, but risky
}

impl FromStr for Appeals {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(appeal) = Appeals::from_name(s) {
            Ok(appeal)
        } else {
            Err(format!("Unknown appeal: {}", s))
        }
    }
}

impl Appeals {
    pub fn from_name(name: &str) -> Option<Appeals> {
        match name.to_ascii_lowercase().as_str() {
            "authority" => Some(Appeals::Authority),
            "morality" => Some(Appeals::Morality),
            "reputation" => Some(Appeals::Reputation),
            "tradition" => Some(Appeals::Tradition),
            "evidence" => Some(Appeals::Evidence),
            "reason" => Some(Appeals::Reason),
            "analogy" => Some(Appeals::Analogy),
            "causality" => Some(Appeals::Causality),
            "intimidation" => Some(Appeals::Intimidation),
            "reassurance" => Some(Appeals::Reassurance),
            "inspiration" => Some(Appeals::Inspiration),
            "provocation" => Some(Appeals::Provocation),
            _ => None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            Appeals::Authority => "authority",
            Appeals::Morality => "morality",
            Appeals::Reputation => "reputation",
            Appeals::Tradition => "tradition",
            Appeals::Evidence => "evidence",
            Appeals::Reason => "reason",
            Appeals::Analogy => "analogy",
            Appeals::Causality => "causality",
            Appeals::Intimidation => "intimidation",
            Appeals::Reassurance => "reassurance",
            Appeals::Inspiration => "inspiration",
            Appeals::Provocation => "provocation",
        }
    }

    pub fn appeal_type(&self) -> AppealType {
        if self.is_ethos() {
            AppealType::Ethos
        } else if self.is_logos() {
            AppealType::Logos
        } else {
            AppealType::Pathos
        }
    }

    pub fn is_ethos(&self) -> bool {
        match self {
            Appeals::Authority | Appeals::Morality | Appeals::Reputation | Appeals::Tradition => {
                true
            }
            _ => false,
        }
    }

    pub fn is_logos(&self) -> bool {
        match self {
            Appeals::Evidence | Appeals::Reason | Appeals::Analogy | Appeals::Causality => true,
            _ => false,
        }
    }

    pub fn is_pathos(&self) -> bool {
        match self {
            Appeals::Intimidation
            | Appeals::Reassurance
            | Appeals::Inspiration
            | Appeals::Provocation => true,
            _ => false,
        }
    }
}

impl Appeals {
    pub fn base_speed(&self) -> f32 {
        match self {
            Appeals::Authority => 2.58,
            Appeals::Morality => 2.80,
            Appeals::Reputation => 2.58,
            Appeals::Tradition => 3.87,
            Appeals::Evidence => 2.80,
            Appeals::Reason => 1.72,
            Appeals::Analogy => 2.80,
            Appeals::Causality => 3.00,
            Appeals::Intimidation => 2.58,
            Appeals::Reassurance => 2.37,
            Appeals::Inspiration => 2.58,
            Appeals::Provocation => 3.00,
        }
    }

    pub fn base_cost(&self) -> i32 {
        match self {
            Appeals::Authority => 196,
            Appeals::Morality => 83,
            Appeals::Reputation => 167,
            Appeals::Tradition => 392,
            Appeals::Evidence => 196,
            Appeals::Reason => 10,
            Appeals::Analogy => 10,
            Appeals::Causality => 10,
            Appeals::Intimidation => 10,
            Appeals::Reassurance => 10,
            Appeals::Inspiration => 10,
            Appeals::Provocation => 10,
        }
    }

    pub fn guess_resolve_damage(
        &self,
        personality: Personality,
        player_stats: &PersuasionState,
    ) -> i32 {
        let base_damage = if self.is_ethos() {
            player_stats.wis.map(|str| str * 10 + 250).unwrap_or(400)
        } else if self.is_logos() {
            player_stats.int.map(|str| str * 10 + 250).unwrap_or(400)
        } else {
            player_stats.str.map(|str| str * 10 + 250).unwrap_or(400)
        };
        let personality = if personality.is_weak_to(*self) {
            base_damage + 100
        } else if personality.is_strong_to(*self) {
            base_damage - 100
        } else {
            base_damage
        };
        let analogy = if player_stats.last_appeal == Some((Appeals::Analogy, false)) {
            if self.is_logos() {
                personality / 2
            } else {
                personality / 2 * 3
            }
        } else if player_stats.last_appeal == Some((Appeals::Analogy, true)) {
            if self.is_logos() {
                personality
            } else {
                personality * 2
            }
        } else {
            personality
        };
        match self {
            Appeals::Tradition | Appeals::Causality | Appeals::Provocation => analogy * 2,
            Appeals::Reassurance => analogy / 2,
            _ => analogy,
        }
    }
}
