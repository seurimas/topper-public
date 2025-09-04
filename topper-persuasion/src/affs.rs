use lazy_static::lazy_static;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Clone,
    Copy,
    TryFromPrimitive,
    EnumString,
    Serialize,
    Deserialize,
    Display,
)]
#[repr(u16)]
pub enum PersuasionAff {
    Conflicted,
    Confounded,
    Engrossed,
    Entrenched,
    Fatigued,
    Pressured,
    LimitedAppeals,
    Slandered,
    Revelation,
    Gravitas,
    Influence,
    Conviction,
    Tradition,

    SIZE,
}

lazy_static! {
    pub static ref PERSUASION_AFFS: Vec<PersuasionAff> = {
        vec![
            PersuasionAff::Conflicted,
            PersuasionAff::Confounded,
            PersuasionAff::Engrossed,
            PersuasionAff::Entrenched,
            PersuasionAff::Fatigued,
            PersuasionAff::Pressured,
            PersuasionAff::LimitedAppeals,
            PersuasionAff::Slandered,
        ]
    };
}
