use super::super::statics::*;
use super::FirstAidPriorities;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use topper_core::observations::strip_ansi;
use topper_core::timeline::BaseAgentState;

#[derive(Debug, Default)]
pub enum VitalsPriority {
    #[default]
    Hp,
    Mp,
    Alt,
}

#[derive(Debug, Default)]
pub struct FirstAidConfig {
    simple_priorities: FirstAidPriorities,
    predicted: Vec<FType>,
    elevated: Vec<FType>,
    health_percent: i32,
    mana_percent: i32,
    force_health_percent: i32,
    force_mana_percent: i32,
    anabiotic_health_percent: i32,
    anabiotic_mana_percent: i32,
    vitals_priority: VitalsPriority,
    stop_mana_below_percent: i32,
    use_anabiotic: bool,
    // Not managed: Precache
    // Not managed: auto stand/wake
    use_tree: bool,
    use_focus: bool,
    use_insomnia: bool,
    use_clotting: bool,
    clot_above_percent_mana: i32,
    clot_above_bleed: i32,
    // Not managed: Stupidity double
    // Not managed: Halt for channel
    adder: Option<isize>,
    stormtouched: bool,
}

#[derive(Debug)]
pub enum FirstaidSetting {
    SimplePriority(FType, u32),
    Predict(FType),
    UnPredict(FType),
    Elevate(FType),
    UnElevate(FType),
    HealthPercent(i32),
    ManaPercent(i32),
    ForceHealthPercent(i32),
    ForceManaPercent(i32),
    AnabioticHealthPercent(i32),
    AnabioticManaPercent(i32),
    VitalsPriority(VitalsPriority),
    StopManaBelowPercent(i32),
    UseAnabiotic(bool),
    UseTree(bool),
    UseFocus(bool),
    UseInsomnia(bool),
    UseClotting(bool),
    ClotAbovePercentMana(i32),
    ClotAboveBleed(i32),
    Adder(isize),
    Stormtouched(bool),
}

impl FirstAidConfig {}
