use super::*;
use crate::alpha_beta::ActionPlanner;
use crate::classes::*;
use crate::curatives::get_cure_depth;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParamComboAttack {
    Tidalslash,
    Freefall,
    Pheromones,
    Pindown,
    Mindnumb,
    Jab(LType),
    Pinprick,
    Lateral,
    Vertical,
    Crescentcut,
    Spinslash,
    Lowhook(LType),
    Butterfly,
    Flashkick,
    Trip,
    Veinrip,
    Feint(LType),
    Raze,
    Gouge,
    Bleed,
    Swiftkick,
}

impl ParamComboAttack {
    pub fn get_param_string(&self) -> String {
        match self {
            ParamComboAttack::Tidalslash => "tidalslash".to_string(),
            ParamComboAttack::Freefall => "freefall".to_string(),
            ParamComboAttack::Pheromones => "pheromones".to_string(),
            ParamComboAttack::Pindown => "pindown".to_string(),
            ParamComboAttack::Mindnumb => "mindnumb".to_string(),
            ParamComboAttack::Jab(limb) => {
                if *limb == LType::LeftArmDamage {
                    "jab left".to_string()
                } else if *limb == LType::RightArmDamage {
                    "jab right".to_string()
                } else {
                    "jab".to_string()
                }
            }
            ParamComboAttack::Lowhook(limb) => {
                if *limb == LType::LeftLegDamage {
                    "lowhook left".to_string()
                } else if *limb == LType::RightLegDamage {
                    "lowhook right".to_string()
                } else {
                    "lowhook".to_string()
                }
            }
            ParamComboAttack::Pinprick => "pinprick".to_string(),
            ParamComboAttack::Lateral => "lateral".to_string(),
            ParamComboAttack::Vertical => "vertical".to_string(),
            ParamComboAttack::Crescentcut => "crescentcut".to_string(),
            ParamComboAttack::Spinslash => "spinslash".to_string(),
            ParamComboAttack::Butterfly => "butterfly".to_string(),
            ParamComboAttack::Flashkick => "flashkick".to_string(),
            ParamComboAttack::Trip => "trip".to_string(),
            ParamComboAttack::Veinrip => "veinrip".to_string(),
            ParamComboAttack::Feint(limb) => format!("feint {}", limb.to_string()),
            ParamComboAttack::Raze => "raze".to_string(),
            ParamComboAttack::Gouge => "gouge".to_string(),
            ParamComboAttack::Bleed => "bleed".to_string(),
            ParamComboAttack::Swiftkick => "swiftkick".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeriesAttack {
    pub attacks: Vec<ParamComboAttack>,
    pub target: String,
    pub venom: VenomType,
}

impl SeriesAttack {
    pub fn new(attacks: Vec<ParamComboAttack>, target: String, venom: &'static str) -> Self {
        Self {
            attacks,
            target,
            venom,
        }
    }

    pub fn new_random_params(
        base_attacks: Vec<ComboAttack>,
        target: String,
        venom: &'static str,
    ) -> Self {
        let mut valid_feints = vec![
            LType::TorsoDamage,
            LType::HeadDamage,
            LType::LeftArmDamage,
            LType::RightArmDamage,
            LType::LeftLegDamage,
            LType::RightLegDamage,
        ];
        valid_feints.retain(|l| {
            base_attacks.iter().any(|attack| match attack {
                ComboAttack::Jab => *l != LType::LeftArmDamage && *l != LType::RightArmDamage,
                ComboAttack::Lowhook => *l != LType::LeftLegDamage && *l != LType::RightLegDamage,
                _ => Some(*l) != attack.get_single_limb_target(),
            })
        });
        let attacks = base_attacks.iter().map(|base| match base {
            ComboAttack::Jab => ParamComboAttack::Jab(LType::HeadDamage),
            ComboAttack::Lowhook => ParamComboAttack::Lowhook(LType::HeadDamage),
            ComboAttack::Feint => ParamComboAttack::Feint(valid_feints[0]),
            ComboAttack::Bleed => ParamComboAttack::Bleed,
            ComboAttack::Gouge => ParamComboAttack::Gouge,
            ComboAttack::Raze => ParamComboAttack::Raze,
            ComboAttack::Swiftkick => ParamComboAttack::Swiftkick,
            ComboAttack::Tidalslash => ParamComboAttack::Tidalslash,
            ComboAttack::Freefall => ParamComboAttack::Freefall,
            ComboAttack::Pheromones => ParamComboAttack::Pheromones,
            ComboAttack::Pindown => ParamComboAttack::Pindown,
            ComboAttack::Mindnumb => ParamComboAttack::Mindnumb,
            ComboAttack::Pinprick => ParamComboAttack::Pinprick,
            ComboAttack::Lateral => ParamComboAttack::Lateral,
            ComboAttack::Vertical => ParamComboAttack::Vertical,
            ComboAttack::Crescentcut => ParamComboAttack::Crescentcut,
            ComboAttack::Spinslash => ParamComboAttack::Spinslash,
            ComboAttack::Butterfly => ParamComboAttack::Butterfly,
            ComboAttack::Flashkick => ParamComboAttack::Flashkick,
            ComboAttack::Trip => ParamComboAttack::Trip,
            ComboAttack::Veinrip => ParamComboAttack::Veinrip,
        });
        Self {
            attacks: attacks.collect(),
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for SeriesAttack {
    fn simulate(&self, timline: &AetTimeline) -> Vec<ProbableEvent> {
        vec![]
    }
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "series {} {} {}",
            self.attacks
                .iter()
                .map(|attack| attack.get_param_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.target,
            self.venom
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BloodscourgeAction {
    pub target: String,
    pub venom: VenomType,
}

impl BloodscourgeAction {
    pub fn new(target: String, venom: &'static str) -> Self {
        Self {
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for BloodscourgeAction {
    fn simulate(&self, timline: &AetTimeline) -> Vec<ProbableEvent> {
        todo!()
    }
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "spider acid {};;bloodscourge {} {}",
            self.target, self.target, self.venom
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DartshotAction {
    pub target: String,
    pub venom: VenomType,
}

impl DartshotAction {
    pub fn new(target: String, venom: &'static str) -> Self {
        Self {
            target,
            venom: venom.into(),
        }
    }
}

impl ActiveTransition for DartshotAction {
    fn simulate(&self, timline: &AetTimeline) -> Vec<ProbableEvent> {
        todo!()
    }
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!("dartshot {} {}", self.target, self.venom))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TwinshotAction {
    pub target: String,
    pub venom_0: VenomType,
    pub venom_1: VenomType,
}

impl TwinshotAction {
    pub fn new(target: String, venom_0: VenomType, venom_1: VenomType) -> Self {
        Self {
            target,
            venom_0,
            venom_1,
        }
    }
}

impl ActiveTransition for TwinshotAction {
    fn simulate(&self, timline: &AetTimeline) -> Vec<ProbableEvent> {
        todo!()
    }
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "twinshot {} {} {}",
            self.target, self.venom_0, self.venom_1
        ))
    }
}
