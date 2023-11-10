use crate::classes::VenomType;

use super::*;
use serde::*;

pub const FEINT_COOLDOWN: CType = 10 * BALANCE_SCALE as CType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum Stance {
    None,
    Gyanis,
    VaeSant,
    Rizet,
    EinFasit,
    Laesan,
}

impl Default for Stance {
    fn default() -> Self {
        Stance::None
    }
}

impl Stance {
    pub fn from_name(name: &str) -> Stance {
        match name {
            "Gyanis" => Stance::Gyanis,
            "Vae-Sant" => Stance::VaeSant,
            "Rizet" => Stance::Rizet,
            "Ein-Fasit" => Stance::EinFasit,
            "Laesan" => Stance::Laesan,
            _ => Stance::None,
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            Stance::Gyanis => "Gyanis",
            Stance::VaeSant => "Vae-Sant",
            Stance::Rizet => "Rizet",
            Stance::EinFasit => "Ein-Fasit",
            Stance::Laesan => "Laesan",
            _ => "None",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredatorCompanionState {
    Orel {
        venoms: Vec<String>,
        swooping: Option<Timer>,
    },
    Orgyuk {
        roaring: Option<Timer>,
    },
    Spider {
        strands_target: Option<String>,
    },
}

impl PredatorCompanionState {
    pub fn wait(&mut self, time: CType) {
        match self {
            PredatorCompanionState::Orel { swooping, .. } => {
                if let Some(swooping) = swooping {
                    swooping.wait(time);
                }
            }
            PredatorCompanionState::Orgyuk { roaring } => {
                if let Some(roaring) = roaring {
                    roaring.wait(time);
                }
            }
            PredatorCompanionState::Spider { .. } => {}
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PredatorClassState {
    pub apex: u32,
    pub stance: Stance,
    pub tidal_charge: u32,
    pub feint_time: CType,
    pub companion: Option<PredatorCompanionState>,
}

impl PredatorClassState {
    pub fn wait(&mut self, time: CType) {
        self.feint_time -= time;
    }

    pub fn feint(&mut self) {
        self.feint_time = FEINT_COOLDOWN;
    }

    pub fn get_spider(&mut self) {
        if !self.has_spider() {
            self.companion = Some(PredatorCompanionState::Spider {
                strands_target: None,
            });
        }
    }

    pub fn has_spider(&self) -> bool {
        if let Some(PredatorCompanionState::Spider { .. }) = self.companion {
            true
        } else {
            false
        }
    }

    pub fn get_orgyuk(&mut self) {
        if !self.has_orgyuk() {
            self.companion = Some(PredatorCompanionState::Orgyuk { roaring: None });
        }
    }

    pub fn has_orgyuk(&self) -> bool {
        if let Some(PredatorCompanionState::Orgyuk { .. }) = self.companion {
            true
        } else {
            false
        }
    }

    pub fn get_orel(&mut self) {
        if !self.has_orel() {
            self.companion = Some(PredatorCompanionState::Orel {
                venoms: Vec::new(),
                swooping: None,
            });
        }
    }

    pub fn has_orel(&self) -> bool {
        if let Some(PredatorCompanionState::Orel { .. }) = self.companion {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredatorBoard {
    pub fleshbane: Timer,
    pub bloodscourge: Timer,
    pub veinrip: Timer,
    pub acid: Timer,
    pub cirisosis: Timer,
}

impl Default for PredatorBoard {
    fn default() -> Self {
        PredatorBoard {
            fleshbane: Timer::count_up_seconds(10.),
            bloodscourge: Timer::count_up_seconds(5.),
            veinrip: Timer::count_up_seconds(6.),
            acid: Timer::count_up_seconds(10.),
            cirisosis: Timer::count_up_seconds(6.),
        }
    }
}

impl PredatorBoard {
    pub fn wait(&mut self, time: CType) {
        self.fleshbane.wait(time);
        self.bloodscourge.wait(time);
        self.veinrip.wait(time);
        self.acid.wait(time);
        self.cirisosis.wait(time);
    }

    pub fn fleshbaned(&mut self) {
        self.fleshbane.reset();
    }

    pub fn bloodscourged(&mut self) {
        self.bloodscourge.reset();
    }

    pub fn bloodscourge_hit(&mut self) {
        self.bloodscourge.reset();
    }

    pub fn bloodscourge_end(&mut self) {
        self.bloodscourge.expire();
    }

    pub fn cirisosis_start(&mut self) {
        self.cirisosis.reset();
    }

    pub fn cirisosis_shock(&mut self) {
        self.cirisosis.reset();
    }

    pub fn cirisosis_lost(&mut self) {
        self.cirisosis.expire();
    }
}
