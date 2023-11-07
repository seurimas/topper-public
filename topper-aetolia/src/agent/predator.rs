use super::*;
use serde::*;

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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PredatorClassState {
    pub apex: u32,
    pub stance: Stance,
    pub tidal_charge: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PredatorBoard {
    pub fleshbane: Option<CType>,
    pub bloodscourge: Option<CType>,
    pub veinrip: Option<CType>,
    pub acid: Option<CType>,
}
