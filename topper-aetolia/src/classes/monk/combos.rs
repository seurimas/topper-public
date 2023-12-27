use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::*;
use crate::types::*;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonkComboAttack {
    // Kicks, valid first attacks.
    Sidekick,
    SnapkickLeft,
    SnapkickRight,
    Roundhouse,
    Sweep,
    Moonkick,
    Cometkick,
    Thrustkick,
    Scythekick,
    Axe,
    Whirlwind,
    Jumpkick,
    // Throws, valid first attacks.
    Slam,
    WrenchLeftLeg,
    WrenchRightLeg,
    WrenchLeftArm,
    WrenchRightArm,
    FeintLeftLeg,
    FeintRightLeg,
    FeintLeftArm,
    FeintRightArm,
    FeintHead,
    FeintTorso,
    // Punches, non-first attacks.
    Jab,
    Hook,
    Uppercut,
    Palmstrike,
    Hammerfist,
    SpearLeft,
    SpearRight,
    ThroatStrike,
    Bladehand,
}

impl MonkComboAttack {
    pub fn is_kick(self) -> bool {
        match self {
            MonkComboAttack::Sidekick
            | MonkComboAttack::SnapkickLeft
            | MonkComboAttack::SnapkickRight
            | MonkComboAttack::Roundhouse
            | MonkComboAttack::Sweep
            | MonkComboAttack::Moonkick
            | MonkComboAttack::Cometkick
            | MonkComboAttack::Thrustkick
            | MonkComboAttack::Scythekick
            | MonkComboAttack::Axe
            | MonkComboAttack::Whirlwind
            | MonkComboAttack::Jumpkick => true,
            _ => false,
        }
    }

    pub fn is_throw(self) -> bool {
        match self {
            MonkComboAttack::Slam
            | MonkComboAttack::WrenchLeftLeg
            | MonkComboAttack::WrenchRightLeg
            | MonkComboAttack::WrenchLeftArm
            | MonkComboAttack::WrenchRightArm => true,
            _ => false,
        }
    }

    pub fn is_feint(self) -> bool {
        match self {
            MonkComboAttack::FeintLeftLeg
            | MonkComboAttack::FeintRightLeg
            | MonkComboAttack::FeintLeftArm
            | MonkComboAttack::FeintRightArm
            | MonkComboAttack::FeintHead
            | MonkComboAttack::FeintTorso => true,
            _ => false,
        }
    }

    pub fn is_punch(self) -> bool {
        match self {
            MonkComboAttack::Jab
            | MonkComboAttack::Hook
            | MonkComboAttack::Uppercut
            | MonkComboAttack::Palmstrike
            | MonkComboAttack::Hammerfist
            | MonkComboAttack::SpearLeft
            | MonkComboAttack::SpearRight
            | MonkComboAttack::ThroatStrike
            | MonkComboAttack::Bladehand => true,
            _ => false,
        }
    }

    pub fn is_first_attack(self) -> bool {
        self.is_kick() || self.is_throw() || self.is_feint()
    }

    pub fn is_non_first_attack(self) -> bool {
        self.is_punch()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MonkCombo(MonkStance, [MonkComboAttack; 3]);

impl MonkCombo {
    pub fn new(stance: MonkStance, attacks: [MonkComboAttack; 3]) -> Self {
        MonkCombo(stance, attacks)
    }
}

#[derive(Debug, Default)]
pub struct MonkComboGenerator {
    valid_attacks: Vec<MonkComboAttack>,
    stance: MonkStance,
}

impl MonkComboGenerator {
    pub fn new(stance: MonkStance) -> Self {
        MonkComboGenerator {
            ..Default::default()
        }
    }

    pub fn set_stance(&mut self, stance: MonkStance) {
        self.stance = stance;
    }

    pub fn get_stance(&self) -> MonkStance {
        self.stance
    }

    pub fn get_valid_attacks(&self) -> &[MonkComboAttack] {
        &self.valid_attacks
    }

    pub fn set_valid_attacks(&mut self, valid_attacks: Vec<MonkComboAttack>) {
        self.valid_attacks = valid_attacks;
    }

    pub fn generate(&self) -> Vec<MonkCombo> {
        let mut combos = Vec::new();
        for first_attack in self.valid_attacks.iter().filter(|a| a.is_first_attack()) {
            for second_attack in self
                .valid_attacks
                .iter()
                .filter(|a| a.is_non_first_attack())
            {
                for third_attack in self
                    .valid_attacks
                    .iter()
                    .filter(|a| a.is_non_first_attack())
                {
                    combos.push(MonkCombo::new(
                        self.stance,
                        [*first_attack, *second_attack, *third_attack],
                    ));
                }
            }
        }
        combos
    }
}
