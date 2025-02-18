use serde::Deserialize;
use topper_core::timeline::CType;

use crate::agent::PersuasionState;

use super::Personality;

#[derive(Debug, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
pub enum AppealType {
    Ethos,
    Logos,
    Pathos,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
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
    pub fn base_speed(&self) -> CType {
        match self {
            Appeals::Authority => 258,
            Appeals::Morality => 280,
            Appeals::Reputation => 258,
            Appeals::Tradition => 387,
            Appeals::Evidence => 280,
            Appeals::Reason => 172,
            Appeals::Analogy => 280,
            Appeals::Causality => 300,
            Appeals::Intimidation => 258,
            Appeals::Reassurance => 237,
            Appeals::Inspiration => 258,
            Appeals::Provocation => 300,
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
        let personality = if personality.is_weak_to(self) {
            base_damage + 100
        } else if personality.is_strong_to(self) {
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
