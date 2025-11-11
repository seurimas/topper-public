use crate::{affliction_plan_stacker, classes::AFFLICT_VENOMS, classes::*, types::*};
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
    DauntCoyote,
    DauntRaloth,
    DauntCrocodile,
    DauntCockatrice,
    Icebreath,
}

impl FirstStrike {
    pub fn combo_str(&self, mirrored: bool) -> String {
        if mirrored {
            match self {
                FirstStrike::Slash(_venom) => "contrive".to_string(),
                FirstStrike::Ambush(_venom) => "waylay".to_string(),
                FirstStrike::Blind => "ploy".to_string(),
                FirstStrike::Twirl => "ruse".to_string(),
                FirstStrike::Strike => "gambit".to_string(),
                FirstStrike::Crosscut => "phlebotomise".to_string(),
                FirstStrike::WeakenArms => "impair".to_string(),
                FirstStrike::WeakenLegs => "impair".to_string(),
                FirstStrike::Reave => "shave".to_string(),
                FirstStrike::Trip => "gambol".to_string(),
                FirstStrike::Slam => "perplex".to_string(),
                _ => "".to_string(),
            }
        } else {
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
    }

    pub fn full_str(&self, target: &String, mirrored: bool) -> String {
        match (self, mirrored) {
            (FirstStrike::DauntCoyote, false) => format!("order coyote daunt {}", target),
            (FirstStrike::DauntCoyote, true) => format!("order darkhound accost {}", target),
            (FirstStrike::DauntRaloth, false) => format!("order raloth daunt {}", target),
            (FirstStrike::DauntRaloth, true) => format!("order brutaliser accost {}", target),
            (FirstStrike::DauntCrocodile, false) => format!("order crocodile daunt {}", target),
            (FirstStrike::DauntCrocodile, true) => format!("order eviscerator accost {}", target),
            (FirstStrike::DauntCockatrice, false) => format!("order cockatrice daunt {}", target),
            (FirstStrike::DauntCockatrice, true) => format!("order terrifier accost {}", target),
            (FirstStrike::Icebreath, false) => format!("order icewyrm icebreath {}", target),
            (FirstStrike::Icebreath, true) => format!("order rimestalker verglas {}", target),
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
            FirstStrike::DauntCoyote
            | FirstStrike::DauntCrocodile
            | FirstStrike::DauntCockatrice
            | FirstStrike::DauntRaloth
            | FirstStrike::Icebreath => true,
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
    pub fn combo_str(&self, mirrored: bool) -> String {
        if mirrored {
            match self {
                SecondStrike::Stab(_venom) => "beguile".to_string(),
                SecondStrike::Slice(_venom) => "wile".to_string(),
                SecondStrike::Thrust(_venom) => "inveigle".to_string(),
                SecondStrike::Disarm => "conciliate".to_string(),
                SecondStrike::Gouge => "muddle".to_string(),
                SecondStrike::Heartbreaker => "desolate".to_string(),
                SecondStrike::Slit => "razor".to_string(),
                _ => "".to_string(),
            }
        } else {
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
    }

    pub fn full_str(&self, target: &String, mirrored: bool) -> String {
        match (self, mirrored) {
            (SecondStrike::Flourish(venom), false) => {
                format!("dhuriv flourish {} {}", target, venom)
            }
            (SecondStrike::Flourish(venom), true) => {
                format!("ringblade brandish {} {}", target, venom)
            }
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
        val.insert(FirstStrike::Slash("epseth"), vec![FType::FeebleLegs]);
        val.insert(FirstStrike::Slash("epteth"), vec![FType::FeebleArms]);
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

affliction_plan_stacker!(
    add_first_strike_from_plan,
    get_first_strike_from_plan,
    FIRST_STRIKES,
    FirstStrike
);

affliction_plan_stacker!(
    add_second_strike_from_plan,
    get_second_strike_from_plan,
    SECOND_STRIKES,
    SecondStrike
);

pub const RESIN_BURN_TIME: CType = 600;
