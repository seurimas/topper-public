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
        swooping: Option<CType>,
    },
    Orgyuk {
        roaring: Option<CType>,
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
                    *swooping -= time;
                }
            }
            PredatorCompanionState::Orgyuk { roaring } => {
                if let Some(roaring) = roaring {
                    *roaring -= time;
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PredatorBoard {
    pub fleshbane: Option<CType>,
    pub bloodscourge: Option<CType>,
    pub veinrip: Option<CType>,
    pub acid: Option<CType>,
    pub cirisosis: Option<CType>,
}

impl PredatorBoard {
    pub fn wait(&mut self, time: CType) {
        if let Some(fleshbane) = &mut self.fleshbane {
            *fleshbane -= time;
        }
        if let Some(bloodscourge) = &mut self.bloodscourge {
            *bloodscourge -= time;
            if *bloodscourge < -600 {
                self.bloodscourge = None;
            }
        }
        if let Some(veinrip) = &mut self.veinrip {
            *veinrip -= time;
        }
        if let Some(acid) = &mut self.acid {
            *acid -= time;
        }
        if let Some(cirisosis) = &mut self.cirisosis {
            *cirisosis -= time;
            if *cirisosis < -600 {
                self.cirisosis = None;
            }
        }
    }

    pub fn fleshbaned(&mut self) {
        self.fleshbane = Some(0);
    }

    pub fn bloodscourged(&mut self) {
        self.bloodscourge = Some(0);
    }

    pub fn bloodscourge_hit(&mut self) {
        self.bloodscourge = Some(0);
    }

    pub fn bloodscourge_end(&mut self) {
        self.bloodscourge = None;
    }

    pub fn cirisosis_start(&mut self) {
        self.cirisosis = Some(0);
    }

    pub fn cirisosis_shock(&mut self) {
        self.cirisosis = Some(0);
    }

    pub fn cirisosis_lost(&mut self) {
        self.cirisosis = None;
    }
}
