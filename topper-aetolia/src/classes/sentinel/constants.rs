use crate::{classes::AFFLICT_VENOMS, types::*};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum FirstStrike {
    Slash(&'static str),
    Ambush(&'static str),
    Blind,
    Twirl,
    Strike,
    Crosscut,
    WeakenArms,
    WeakenLegs,
    Reave,
    Trip,
    Slam,
    Daunt(&'static str),
    Icebreath,
}

impl FirstStrike {
    pub fn combo_str(&self) -> String {
        match self {
            FirstStrike::Slash(venom) => "slash".to_string(),
            FirstStrike::Ambush(venom) => "ambush".to_string(),
            FirstStrike::Blind => "blind".to_string(),
            FirstStrike::Twirl => "twirl".to_string(),
            FirstStrike::Strike => "strike".to_string(),
            FirstStrike::Crosscut => "crosscut".to_string(),
            FirstStrike::WeakenArms => "weaken arms".to_string(),
            FirstStrike::WeakenLegs => "weaken legs".to_string(),
            FirstStrike::Reave => "reave".to_string(),
            FirstStrike::Trip => "trip".to_string(),
            FirstStrike::Slam => "slam".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            FirstStrike::Daunt(animal) => format!("order {} daunt {}", animal, target),
            FirstStrike::Icebreath => format!("order icewyrm icebreath {}", target),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            FirstStrike::Slash(venom) | FirstStrike::Ambush(venom) => venom,
            _ => "",
        }
    }

    pub fn flourish(&self) -> bool {
        match self {
            FirstStrike::Daunt(_) | FirstStrike::Icebreath => true,
            _ => false,
        }
    }

    pub fn ignores_rebounding(&self) -> bool {
        match self {
            FirstStrike::Twirl => false, // TODO: We need to handle for second strike rebounding if we try this.
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SecondStrike {
    Stab(&'static str),
    Slice(&'static str),
    Thrust(&'static str),
    Flourish(&'static str),
    Disarm,
    Gouge,
    Heartbreaker,
    Slit,
}

impl SecondStrike {
    pub fn combo_str(&self) -> String {
        match self {
            SecondStrike::Stab(venom) => "stab".to_string(),
            SecondStrike::Slice(venom) => "slice".to_string(),
            SecondStrike::Thrust(venom) => "thrust".to_string(),
            SecondStrike::Disarm => "disarm".to_string(),
            SecondStrike::Gouge => "gouge".to_string(),
            SecondStrike::Heartbreaker => "heartbreaker".to_string(),
            SecondStrike::Slit => "slit".to_string(),
            _ => "".to_string(),
        }
    }

    pub fn full_str(&self, target: &String) -> String {
        match self {
            SecondStrike::Flourish(venom) => format!("dhuriv flourish {} {}", target, venom),
            _ => "".to_string(), // TODO
        }
    }

    pub fn venom(&self) -> &'static str {
        match self {
            SecondStrike::Stab(venom)
            | SecondStrike::Slice(venom)
            | SecondStrike::Thrust(venom) => venom,
            _ => "",
        }
    }
}

lazy_static! {
    pub static ref FIRST_STRIKES: HashMap<FType, FirstStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, FirstStrike::Slash(venom));
        }
        val.insert(FType::Frozen, FirstStrike::Icebreath);
        val.insert(FType::Shivering, FirstStrike::Icebreath);
        val.insert(FType::Confusion, FirstStrike::Twirl);
        val.insert(FType::Impairment, FirstStrike::Crosscut);
        val.insert(FType::Addiction, FirstStrike::Crosscut);
        val.insert(FType::Lethargy, FirstStrike::WeakenLegs);
        val.insert(FType::Epilepsy, FirstStrike::Slam);
        val.insert(FType::Laxity, FirstStrike::Slam);
        val
    };
}

lazy_static! {
    pub static ref FIRST_STRIKE_AFFS: HashMap<FirstStrike, Vec<FType>> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(FirstStrike::Slash(venom), vec![*aff]);
        }
        val.insert(FirstStrike::Slash("epseth"), vec![FType::LeftLegCrippled, FType::RightLegCrippled]);
        val.insert(FirstStrike::Slash("epteth"), vec![FType::LeftArmCrippled, FType::RightArmCrippled]);
        val.insert(FirstStrike::Twirl, vec![FType::Confusion]);
        // Wrong, only one actually applies
        val.insert(FirstStrike::Crosscut, vec![FType::Impairment, FType::Addiction]);
        val.insert(FirstStrike::WeakenLegs, vec![FType::Lethargy]);
        val.insert(FirstStrike::Slam, vec![FType::Epilepsy, FType::Laxity]);
        val
    };
}

lazy_static! {
    pub static ref SECOND_STRIKES: HashMap<FType, SecondStrike> = {
        let mut val = HashMap::new();
        for (aff, venom) in AFFLICT_VENOMS.iter() {
            val.insert(*aff, SecondStrike::Stab(venom));
        }
        val.insert(FType::Impatience, SecondStrike::Gouge);
        val.insert(FType::Arrhythmia, SecondStrike::Heartbreaker);
        val.insert(FType::CrippledThroat, SecondStrike::Slit);
        val
    };
}

lazy_static! {
    pub static ref DUALRAZE_ORDER: Vec<FType> =
        vec![FType::Shielded, FType::Rebounding, FType::Speed,];
}

lazy_static! {
    pub static ref REAVE_ORDER: Vec<FType> = vec![FType::Shielded, FType::Rebounding];
}
