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

/**
 * Status:             ON
---------------------
Curing:             ON
Defences:           ON
Reporting:          ON
Heal Health:        ON (85%) (Force: 50%)
Heal Mana:          ON (85%) (Force: 40%)
Vitals Priority:    Health
Using Anabiotic:    ON (H:65%) (M:65%)
Stop Mana Below:    ON (40%)
Stupidity Double:   ON
Pre-caching:        ON (Amt: 2)
Auto-stand:         ON
Auto-wake:          ON
Tree Curing:        ON
Focus Curing:       ON
Clotting:           ON (At: 50) (M:60%)
Insomnia:           ON
Pre-restoration:    ON
Adder:              OFF
Stormtouched:       OFF
Halt For Channel:   ON
 */
lazy_static! {
    static ref HEAL_HEALTH: Regex =
        Regex::new(r"Heal Health:\s+ON \((\d+)%\) \(Force: (\d+)%\)").unwrap();
    static ref HEAL_MANA: Regex =
        Regex::new(r"Heal Mana:\s+ON \((\d+)%\) \(Force: (\d+)%\)").unwrap();
    static ref VITALS_PRIORITY: Regex = Regex::new(r"Vitals Priority:\s+(\w+)").unwrap();
    static ref USE_ANABIOTIC: Regex =
        Regex::new(r"Using Anabiotic:\s+ON \(H:(\d+)%\) \(M:(\d+)%\)").unwrap();
    static ref STOP_MANA_BELOW: Regex = Regex::new(r"Stop Mana Below:\s+ON \((\d+)%\)").unwrap();
    static ref USE_TREE: Regex = Regex::new(r"Tree Curing:\s+(ON|OFF)").unwrap();
    static ref USE_FOCUS: Regex = Regex::new(r"Focus Curing:\s+(ON|OFF)").unwrap();
    static ref CLOTTING: Regex =
        Regex::new(r"Clotting:\s+(ON|SAFE) \(At: (\d+)\) \(M:(\d+)%\)").unwrap();
    static ref CLOTTING_OFF: Regex = Regex::new(r"Clotting:\s+OFF").unwrap();
    static ref INSOMNIA: Regex = Regex::new(r"Insomnia:\s+(ON|OFF)").unwrap();
    static ref ADDER: Regex = Regex::new(r"Adder:\s+(OFF|(\d+) seconds)").unwrap();
    static ref STORMTOUCHED: Regex = Regex::new(r"Stormtouched:\s+(ON|OFF)").unwrap();
}

lazy_static! {
    static ref SET_SIMPLE_PRIORITY: Regex = Regex::new(r"You have set the '(\w+)' (?:affliction|defence) to the (\d+) priority.").unwrap();
    static ref SET_RESET_PRIORITIES: Regex = Regex::new(r"All your curing affliction priorities have been reset to defaults.").unwrap();
    static ref SET_HEAL_HEALTH: Regex =
        Regex::new(r"You will now heal your health when it drops below (\d+)%.").unwrap();
    static ref SET_HEAL_MANA: Regex =
        Regex::new(r"You will now heal your mana when it drops below (\d+)%.").unwrap();
    static ref SET_FORCE_HEALTH: Regex =
        Regex::new(r"You will now prioritize healing your health when it drops below (\d+)%.").unwrap();
    static ref SET_FORCE_MANA: Regex =
        Regex::new(r"You will now prioritize healing your mana when it drops below (\d+)%.").unwrap();
    static ref SET_VITALS_PRIORITY: Regex = Regex::new(r"First Aid will now (?:priortize healing|alternate between) (health over mana|mana over health|health and mana) if both are low.").unwrap();
    static ref SET_USE_ANABIOTIC: Regex =
        Regex::new(r"You will now eat anabiotic when your (health|mana) drops below (\d+)%.").unwrap();
    static ref SET_STOP_MANA_BELOW: Regex =
        Regex::new(r"First Aid will no longer use mana-consuming commands when your mana falls below (\d+)%.").unwrap();
    static ref SET_USE_TREE: Regex =
        Regex::new(r"First Aid will (now utilize|no longer utilize) the tree of life tattoo in its curing.").unwrap();
    static ref SET_USE_FOCUS: Regex =
        Regex::new(r"First Aid will (now utilize|no longer utilize) the focusing ability in its curing.").unwrap();
    static ref SET_USE_INSOMNIA: Regex =
        Regex::new(r"First Aid will (now attempt to use|no longer attempt to use) the Insomnia skill (instead of a kawhe pill|and will eat a kawhe pill).").unwrap();
    static ref SET_USE_CLOTTING: Regex =
        Regex::new(r"First Aid will( no longer attempt to| now attempt to|) clot your bleeding damage( only if you do not have haemophilia and alike afflictions)?.?").unwrap();
    static ref SET_CLOTTING_MANA: Regex =
        Regex::new(r"You will now use clotting when your mana is above (\d+)%.").unwrap();
    static ref SET_CLOTTING_BLEED: Regex =
        Regex::new(r"First Aid will now only start clotting when bleed amount is above (\d+).").unwrap();
    static ref SET_ADDER: Regex =
        Regex::new(r"FirstAid will (no longer attempt to|now send) RIP CARD FROM BODY(?: when (\d+) seconds have passed after receiving the affliction|).").unwrap();
    static ref SET_STORMTOUCHED: Regex =
        Regex::new(r"First Aid will (no longer|now) utilize stormtouched mode curing.").unwrap();
    /**
     * You have added the confusion affliction to your predicted afflictions list.
        You have elevated confusion to be the maximum priority.
        You have unelevated confusion back to its original priority.
        You are no longer predicting that you have been afflicted with the confusion affliction.
        You're not afflicted with confusion.
     */
    static ref SET_PREDICTED: Regex =
        Regex::new(r"You have added the (\w+) affliction to your predicted afflictions list.").unwrap();
    static ref SET_ELEVATED: Regex =
        Regex::new(r"You have elevated (\w+) to be the maximum priority.").unwrap();
    static ref SET_UNELEVATED: Regex =
        Regex::new(r"You have unelevated (\w+) back to its original priority.").unwrap();
    static ref SET_UNPREDICTED: Regex =
        Regex::new(r"You are no longer predicting that you have been afflicted with the (\w+) affliction.").unwrap();
    static ref SET_NOT_AFFLICTED: Regex =
        Regex::new(r"You're not afflicted with (\w+).").unwrap();
}

#[derive(Debug, Display, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub enum VitalsPriority {
    #[default]
    Hp,
    Mp,
    Alt,
}

#[derive(Debug, Display, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub enum ClottingType {
    #[default]
    On,
    Safe,
    Off,
}

#[derive(Debug, Default)]
pub struct FirstAidConfig {
    simple_priorities: FirstAidPriorities,
    pub predicted: Vec<FType>,
    pub elevated: Vec<FType>,
    health_percent: i32,
    force_health_percent: i32,

    mana_percent: i32,
    force_mana_percent: i32,

    vitals_priority: VitalsPriority,

    use_anabiotic: bool,
    anabiotic_health_percent: i32,
    anabiotic_mana_percent: i32,

    stop_mana_below_percent: i32,
    // Not managed: Precache
    // Not managed: auto stand/wake
    use_tree: bool,
    use_focus: bool,
    use_insomnia: bool,

    use_clotting: ClottingType,
    clot_above_percent_mana: i32,
    clot_above_bleed: i32,
    // Not managed: Stupidity double
    // Not managed: Halt for channel
    adder: Option<isize>,
    stormtouched: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
pub enum FirstAidSetting {
    SimplePriority(FType, u32),
    ResetPriorities,
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
    UseClotting(ClottingType),
    ClotAbovePercentMana(i32),
    ClotAboveBleed(i32),
    Adder(Option<isize>),
    Stormtouched(bool),
}

impl FirstAidConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, setting: FirstAidSetting) -> FirstAidSetting {
        match setting {
            FirstAidSetting::SimplePriority(ft, prio) => {
                if let Some(previous) = self.simple_priorities.insert(ft, prio) {
                    FirstAidSetting::SimplePriority(ft, previous)
                } else {
                    FirstAidSetting::SimplePriority(ft, 26)
                }
            }
            FirstAidSetting::ResetPriorities => {
                if let Some((aff, priority)) = self.simple_priorities.reset() {
                    FirstAidSetting::SimplePriority(aff, priority)
                } else {
                    FirstAidSetting::ResetPriorities
                }
            }
            FirstAidSetting::Predict(ft) => {
                if self.predicted.contains(&ft) {
                    FirstAidSetting::Predict(ft)
                } else {
                    self.predicted.push(ft);
                    FirstAidSetting::UnPredict(ft)
                }
            }
            FirstAidSetting::UnPredict(ft) => {
                if self.predicted.contains(&ft) {
                    self.predicted.retain(|&x| x != ft);
                    FirstAidSetting::Predict(ft)
                } else {
                    FirstAidSetting::UnPredict(ft)
                }
            }
            FirstAidSetting::Elevate(ft) => {
                if self.elevated.contains(&ft) {
                    FirstAidSetting::Elevate(ft)
                } else {
                    self.elevated.push(ft);
                    FirstAidSetting::UnElevate(ft)
                }
            }
            FirstAidSetting::UnElevate(ft) => {
                if self.elevated.contains(&ft) {
                    self.elevated.retain(|&x| x != ft);
                    FirstAidSetting::Elevate(ft)
                } else {
                    FirstAidSetting::UnElevate(ft)
                }
            }
            FirstAidSetting::HealthPercent(p) => {
                let previous = self.health_percent;
                self.health_percent = p;
                FirstAidSetting::HealthPercent(previous)
            }
            FirstAidSetting::ManaPercent(p) => {
                let previous = self.mana_percent;
                self.mana_percent = p;
                FirstAidSetting::ManaPercent(previous)
            }
            FirstAidSetting::ForceHealthPercent(p) => {
                let previous = self.force_health_percent;
                self.force_health_percent = p;
                FirstAidSetting::ForceHealthPercent(previous)
            }
            FirstAidSetting::ForceManaPercent(p) => {
                let previous = self.force_mana_percent;
                self.force_mana_percent = p;
                FirstAidSetting::ForceManaPercent(previous)
            }
            FirstAidSetting::AnabioticHealthPercent(p) => {
                let previous = self.anabiotic_health_percent;
                self.anabiotic_health_percent = p;
                FirstAidSetting::AnabioticHealthPercent(previous)
            }
            FirstAidSetting::AnabioticManaPercent(p) => {
                let previous = self.anabiotic_mana_percent;
                self.anabiotic_mana_percent = p;
                FirstAidSetting::AnabioticManaPercent(previous)
            }
            FirstAidSetting::VitalsPriority(p) => {
                let previous = self.vitals_priority;
                self.vitals_priority = p;
                FirstAidSetting::VitalsPriority(previous)
            }
            FirstAidSetting::StopManaBelowPercent(p) => {
                let previous = self.stop_mana_below_percent;
                self.stop_mana_below_percent = p;
                FirstAidSetting::StopManaBelowPercent(previous)
            }
            FirstAidSetting::UseAnabiotic(b) => {
                let previous = self.use_anabiotic;
                self.use_anabiotic = b;
                FirstAidSetting::UseAnabiotic(previous)
            }
            FirstAidSetting::UseTree(b) => {
                let previous = self.use_tree;
                self.use_tree = b;
                FirstAidSetting::UseTree(previous)
            }
            FirstAidSetting::UseFocus(b) => {
                let previous = self.use_focus;
                self.use_focus = b;
                FirstAidSetting::UseFocus(previous)
            }
            FirstAidSetting::UseInsomnia(b) => {
                let previous = self.use_insomnia;
                self.use_insomnia = b;
                FirstAidSetting::UseInsomnia(previous)
            }
            FirstAidSetting::UseClotting(b) => {
                let previous = self.use_clotting;
                self.use_clotting = b;
                FirstAidSetting::UseClotting(previous)
            }
            FirstAidSetting::ClotAbovePercentMana(p) => {
                let previous = self.clot_above_percent_mana;
                self.clot_above_percent_mana = p;
                FirstAidSetting::ClotAbovePercentMana(previous)
            }
            FirstAidSetting::ClotAboveBleed(p) => {
                let previous = self.clot_above_bleed;
                self.clot_above_bleed = p;
                FirstAidSetting::ClotAboveBleed(previous)
            }
            FirstAidSetting::Adder(p) => {
                let previous = self.adder;
                self.adder = p;
                FirstAidSetting::Adder(previous)
            }
            FirstAidSetting::Stormtouched(b) => {
                let previous = self.stormtouched;
                self.stormtouched = b;
                FirstAidSetting::Stormtouched(previous)
            }
        }
    }
}

impl FirstAidSetting {
    pub fn get_command(&self) -> String {
        match self {
            FirstAidSetting::SimplePriority(ft, prio) => {
                if ft.is_affliction() {
                    format!("firstaid priority {} {}", ft, prio)
                } else {
                    format!("firstaid priority defense {} {}", ft, prio)
                }
            }
            FirstAidSetting::ResetPriorities => "firstaid priority reset".to_string(),
            FirstAidSetting::Predict(ft) => {
                format!("firstaid predict {}", ft)
            }
            FirstAidSetting::UnPredict(ft) => {
                format!("firstaid unpredict {}", ft)
            }
            FirstAidSetting::Elevate(ft) => {
                format!("firstaid elevate {}", ft)
            }
            FirstAidSetting::UnElevate(ft) => {
                format!("firstaid unelevate {}", ft)
            }
            FirstAidSetting::HealthPercent(p) => {
                format!("firstaid health {}", p)
            }
            FirstAidSetting::ManaPercent(p) => {
                format!("firstaid mana {}", p)
            }
            FirstAidSetting::ForceHealthPercent(p) => {
                format!("firstaid forcehealth {}", p)
            }
            FirstAidSetting::ForceManaPercent(p) => {
                format!("firstaid forcemana {}", p)
            }
            FirstAidSetting::AnabioticHealthPercent(p) => {
                format!("firstaid anabiotic health {}", p)
            }
            FirstAidSetting::AnabioticManaPercent(p) => {
                format!("firstaid anabiotic mana {}", p)
            }
            FirstAidSetting::VitalsPriority(p) => {
                format!("firstaid vitals priority {}", p)
            }
            FirstAidSetting::StopManaBelowPercent(p) => {
                format!("firstaid stop mana {}", p)
            }
            FirstAidSetting::UseAnabiotic(b) => {
                format!("firstaid use anabiotic {}", if *b { "on" } else { "off" })
            }
            FirstAidSetting::UseTree(b) => {
                format!("firstaid use tree {}", if *b { "on" } else { "off" })
            }
            FirstAidSetting::UseFocus(b) => {
                format!("firstaid use focus {}", if *b { "on" } else { "off" })
            }
            FirstAidSetting::UseInsomnia(b) => {
                format!("firstaid use insomnia {}", if *b { "on" } else { "off" })
            }
            FirstAidSetting::UseClotting(b) => {
                format!("firstaid use clot {}", b)
            }
            FirstAidSetting::ClotAbovePercentMana(p) => {
                format!("firstaid clot above {}", p)
            }
            FirstAidSetting::ClotAboveBleed(p) => {
                format!("firstaid clot at {}", p)
            }
            FirstAidSetting::Adder(p) => {
                if let Some(p) = p {
                    format!("firstaid adder {}", p)
                } else {
                    "firstaid adder 0".to_string()
                }
            }
            FirstAidSetting::Stormtouched(b) => {
                format!("firstaid stormtouched {}", if *b { "on" } else { "off" })
            }
        }
    }

    pub fn has_setting_in_line(line: &str) -> bool {
        HEAL_HEALTH.is_match(line)
            || HEAL_MANA.is_match(line)
            || VITALS_PRIORITY.is_match(line)
            || USE_ANABIOTIC.is_match(line)
            || STOP_MANA_BELOW.is_match(line)
            || USE_TREE.is_match(line)
            || USE_FOCUS.is_match(line)
            || CLOTTING.is_match(line)
            || CLOTTING_OFF.is_match(line)
            || INSOMNIA.is_match(line)
            || ADDER.is_match(line)
            || STORMTOUCHED.is_match(line)
    }

    pub fn get_setting_from_line(line: &str) -> Vec<Self> {
        if let Some(caps) = HEAL_HEALTH.captures(line) {
            return vec![
                FirstAidSetting::HealthPercent(caps[1].parse().unwrap()),
                FirstAidSetting::ForceHealthPercent(caps[2].parse().unwrap()),
            ];
        }
        if let Some(caps) = HEAL_MANA.captures(line) {
            return vec![
                FirstAidSetting::ManaPercent(caps[1].parse().unwrap()),
                FirstAidSetting::ForceManaPercent(caps[2].parse().unwrap()),
            ];
        }
        if let Some(caps) = VITALS_PRIORITY.captures(line) {
            return vec![FirstAidSetting::VitalsPriority(match &caps[1] {
                "Health" => VitalsPriority::Hp,
                "Mana" => VitalsPriority::Mp,
                _ => VitalsPriority::Alt,
            })];
        }
        if let Some(caps) = USE_ANABIOTIC.captures(line) {
            return vec![
                FirstAidSetting::AnabioticHealthPercent(caps[1].parse().unwrap()),
                FirstAidSetting::AnabioticManaPercent(caps[2].parse().unwrap()),
            ];
        }
        if let Some(caps) = STOP_MANA_BELOW.captures(line) {
            return vec![FirstAidSetting::StopManaBelowPercent(
                caps[1].parse().unwrap(),
            )];
        }
        if let Some(caps) = USE_TREE.captures(line) {
            return vec![FirstAidSetting::UseTree(&caps[1] == "ON")];
        }
        if let Some(caps) = USE_FOCUS.captures(line) {
            return vec![FirstAidSetting::UseFocus(&caps[1] == "ON")];
        }
        if let Some(caps) = CLOTTING.captures(line) {
            return vec![
                FirstAidSetting::UseClotting(match &caps[1] {
                    "ON" => ClottingType::On,
                    "SAFE" => ClottingType::Safe,
                    _ => ClottingType::Off,
                }),
                FirstAidSetting::ClotAboveBleed(caps[2].parse().unwrap()),
                FirstAidSetting::ClotAbovePercentMana(caps[3].parse().unwrap()),
            ];
        }
        if CLOTTING_OFF.is_match(line) {
            return vec![FirstAidSetting::UseClotting(ClottingType::Off)];
        }
        if let Some(caps) = INSOMNIA.captures(line) {
            return vec![FirstAidSetting::UseInsomnia(&caps[1] == "ON")];
        }
        if let Some(caps) = ADDER.captures(line) {
            return vec![FirstAidSetting::Adder(if &caps[1] == "OFF" {
                None
            } else {
                Some(caps[2].parse().unwrap())
            })];
        }
        if let Some(caps) = STORMTOUCHED.captures(line) {
            return vec![FirstAidSetting::Stormtouched(&caps[1] == "ON")];
        }
        vec![]
    }

    pub fn setting_changed_in_line(line: &str) -> bool {
        SET_SIMPLE_PRIORITY.is_match(line)
            || SET_RESET_PRIORITIES.is_match(line)
            || SET_HEAL_HEALTH.is_match(line)
            || SET_HEAL_MANA.is_match(line)
            || SET_FORCE_HEALTH.is_match(line)
            || SET_FORCE_MANA.is_match(line)
            || SET_VITALS_PRIORITY.is_match(line)
            || SET_USE_ANABIOTIC.is_match(line)
            || SET_STOP_MANA_BELOW.is_match(line)
            || SET_USE_TREE.is_match(line)
            || SET_USE_FOCUS.is_match(line)
            || SET_USE_INSOMNIA.is_match(line)
            || SET_USE_CLOTTING.is_match(line)
            || SET_CLOTTING_MANA.is_match(line)
            || SET_CLOTTING_BLEED.is_match(line)
            || SET_ADDER.is_match(line)
            || SET_STORMTOUCHED.is_match(line)
            || SET_PREDICTED.is_match(line)
            || SET_ELEVATED.is_match(line)
            || SET_UNELEVATED.is_match(line)
            || SET_UNPREDICTED.is_match(line)
            || SET_NOT_AFFLICTED.is_match(line)
    }

    fn get_setting_changed_from_line(line: &str) -> Vec<Self> {
        if let Some(caps) = SET_SIMPLE_PRIORITY.captures(line) {
            return vec![FirstAidSetting::SimplePriority(
                FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
                caps[2].parse().unwrap(),
            )];
        }
        if SET_RESET_PRIORITIES.is_match(line) {
            return vec![FirstAidSetting::ResetPriorities];
        }
        if let Some(caps) = SET_HEAL_HEALTH.captures(line) {
            return vec![FirstAidSetting::HealthPercent(caps[1].parse().unwrap())];
        }
        if let Some(caps) = SET_HEAL_MANA.captures(line) {
            return vec![FirstAidSetting::ManaPercent(caps[1].parse().unwrap())];
        }
        if let Some(caps) = SET_FORCE_HEALTH.captures(line) {
            return vec![FirstAidSetting::ForceHealthPercent(
                caps[1].parse().unwrap(),
            )];
        }
        if let Some(caps) = SET_FORCE_MANA.captures(line) {
            return vec![FirstAidSetting::ForceManaPercent(caps[1].parse().unwrap())];
        }
        if let Some(caps) = SET_VITALS_PRIORITY.captures(line) {
            return vec![FirstAidSetting::VitalsPriority(match &caps[0] {
                "First Aid will now priortize healing health over mana if both are low." => {
                    VitalsPriority::Hp
                }
                "First Aid will now priortize healing mana over health if both are low." => {
                    VitalsPriority::Mp
                }
                _ => VitalsPriority::Alt,
            })];
        }
        if let Some(caps) = SET_USE_ANABIOTIC.captures(line) {
            match &caps[1] {
                "health" => {
                    return vec![FirstAidSetting::AnabioticHealthPercent(
                        caps[2].parse().unwrap(),
                    )];
                }
                "mana" => {
                    return vec![FirstAidSetting::AnabioticManaPercent(
                        caps[2].parse().unwrap(),
                    )];
                }
                _ => {}
            }
        }
        if let Some(caps) = SET_STOP_MANA_BELOW.captures(line) {
            return vec![FirstAidSetting::StopManaBelowPercent(
                caps[1].parse().unwrap(),
            )];
        }
        if SET_USE_TREE.is_match(line) {
            return vec![FirstAidSetting::UseTree(line.contains("now utilize"))];
        }
        if SET_USE_FOCUS.is_match(line) {
            return vec![FirstAidSetting::UseFocus(line.contains("now utilize"))];
        }
        if SET_USE_INSOMNIA.is_match(line) {
            return vec![FirstAidSetting::UseInsomnia(
                line.contains("now attempt to"),
            )];
        }
        if let Some(caps) = SET_USE_CLOTTING.captures(line) {
            return vec![FirstAidSetting::UseClotting(match &caps[0] {
                "First Aid will no longer attempt to clot your bleeding damage." => ClottingType::Off,
                "First Aid will clot your bleeding damage only if you do not have haemophilia and alike afflictions." => ClottingType::Safe,
                _ => ClottingType::On,
            })];
        }
        if let Some(caps) = SET_CLOTTING_MANA.captures(line) {
            return vec![FirstAidSetting::ClotAbovePercentMana(
                caps[1].parse().unwrap(),
            )];
        }
        if let Some(caps) = SET_CLOTTING_BLEED.captures(line) {
            return vec![FirstAidSetting::ClotAboveBleed(caps[1].parse().unwrap())];
        }
        if let Some(caps) = SET_ADDER.captures(line) {
            return vec![FirstAidSetting::Adder(
                if &caps[1] == "no longer attempt to" {
                    None
                } else {
                    Some(caps[2].parse().unwrap())
                },
            )];
        }
        if SET_STORMTOUCHED.is_match(line) {
            return vec![FirstAidSetting::Stormtouched(line.contains("now"))];
        }
        if let Some(caps) = SET_PREDICTED.captures(line) {
            return vec![FirstAidSetting::Predict(
                FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
            )];
        }
        if let Some(caps) = SET_ELEVATED.captures(line) {
            return vec![FirstAidSetting::Elevate(
                FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
            )];
        }
        if let Some(caps) = SET_UNELEVATED.captures(line) {
            return vec![FirstAidSetting::UnElevate(
                FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
            )];
        }
        if let Some(caps) = SET_UNPREDICTED.captures(line) {
            return vec![FirstAidSetting::UnPredict(
                FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
            )];
        }
        if let Some(caps) = SET_NOT_AFFLICTED.captures(line) {
            return vec![
                FirstAidSetting::UnElevate(
                    FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
                ),
                FirstAidSetting::UnPredict(
                    FType::from_name(&caps[1].to_string()).unwrap_or(FType::Dead),
                ),
            ];
        }
        vec![]
    }

    pub fn get_from_line(line: &str) -> Vec<Self> {
        if Self::has_setting_in_line(line) {
            return Self::get_setting_from_line(line);
        } else if Self::setting_changed_in_line(line) {
            return Self::get_setting_changed_from_line(line);
        }
        vec![]
    }
}

#[cfg(test)]
mod firstaid_tests {
    use super::*;

    #[test]
    fn test_health() {
        let line = "Heal Health:        ON (85%) (Force: 50%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![
                FirstAidSetting::HealthPercent(85),
                FirstAidSetting::ForceHealthPercent(50)
            ]
        );
    }

    #[test]
    fn test_mana() {
        let line = "Heal Mana:          ON (85%) (Force: 40%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![
                FirstAidSetting::ManaPercent(85),
                FirstAidSetting::ForceManaPercent(40)
            ]
        );
    }

    #[test]
    fn test_vitals() {
        let line = "Vitals Priority:    Health";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::VitalsPriority(VitalsPriority::Hp)]
        );
    }

    #[test]
    fn test_sip_setting_changed() {
        let line = "You will now heal your health when it drops below 85%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::HealthPercent(85)]);
        let line = "You will now heal your mana when it drops below 85%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::ManaPercent(85)]);
        let line = "You will now prioritize healing your health when it drops below 50%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::ForceHealthPercent(50)]);
        let line = "You will now prioritize healing your mana when it drops below 40%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::ForceManaPercent(40)]);
        let line = "First Aid will now priortize healing health over mana if both are low.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::VitalsPriority(VitalsPriority::Hp)]
        );
        let line = "First Aid will now priortize healing mana over health if both are low.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::VitalsPriority(VitalsPriority::Mp)]
        );
        let line = "First Aid will now alternate between health and mana if both are low.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::VitalsPriority(VitalsPriority::Alt)]
        );
    }

    #[test]
    fn test_anabiotic() {
        let line = "Using Anabiotic:    ON (H:65%) (M:65%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![
                FirstAidSetting::AnabioticHealthPercent(65),
                FirstAidSetting::AnabioticManaPercent(65)
            ]
        );
    }

    #[test]
    fn test_change_anabiotic() {
        let line = "You will now eat anabiotic when your health drops below 65%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::AnabioticHealthPercent(65)]);
        let line = "You will now eat anabiotic when your mana drops below 65%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::AnabioticManaPercent(65)]);
    }

    #[test]
    fn test_stop_mana() {
        let line = "Stop Mana Below:    ON (40%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::StopManaBelowPercent(40)]);
    }

    #[test]
    fn test_change_stop_mana() {
        let line =
            "First Aid will no longer use mana-consuming commands when your mana falls below 40%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::StopManaBelowPercent(40)]);
    }

    #[test]
    fn test_tree() {
        let line = "Tree Curing:        ON";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseTree(true)]);
    }

    #[test]
    fn test_focus() {
        let line = "Focus Curing:       ON";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseFocus(true)]);
    }

    #[test]
    fn test_toggle_tree_focus() {
        let line = "First Aid will now utilize the tree of life tattoo in its curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseTree(true)]);
        let line = "First Aid will no longer utilize the tree of life tattoo in its curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseTree(false)]);
        let line = "First Aid will now utilize the focusing ability in its curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseFocus(true)]);
        let line = "First Aid will no longer utilize the focusing ability in its curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseFocus(false)]);
    }

    #[test]
    fn test_clotting() {
        let line = "Clotting:           ON (At: 50) (M:60%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![
                FirstAidSetting::UseClotting(ClottingType::On),
                FirstAidSetting::ClotAboveBleed(50),
                FirstAidSetting::ClotAbovePercentMana(60)
            ]
        );
        let line = "Clotting:           SAFE (At: 50) (M:60%)";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![
                FirstAidSetting::UseClotting(ClottingType::Safe),
                FirstAidSetting::ClotAboveBleed(50),
                FirstAidSetting::ClotAbovePercentMana(60)
            ]
        );
        let line = "Clotting:           OFF";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::UseClotting(ClottingType::Off)]
        );
    }

    #[test]
    fn test_change_clotting() {
        let line = "First Aid will clot your bleeding damage only if you do not have haemophilia and alike afflictions.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::UseClotting(ClottingType::Safe)]
        );
        let line = "First Aid will no longer attempt to clot your bleeding damage.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::UseClotting(ClottingType::Off)]
        );
        let line = "First Aid will now attempt to clot your bleeding damage.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(
            settings,
            vec![FirstAidSetting::UseClotting(ClottingType::On)]
        );
        let line = "First Aid will now only start clotting when bleed amount is above 50.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::ClotAboveBleed(50)]);
        let line = "You will now use clotting when your mana is above 60%.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::ClotAbovePercentMana(60)]);
    }

    #[test]
    fn test_insomnia() {
        let line = "Insomnia:           ON";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseInsomnia(true)]);
    }

    #[test]
    fn test_change_insomnia() {
        let line = "First Aid will now attempt to use the Insomnia skill instead of a kawhe pill.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseInsomnia(true)]);
        let line =
            "First Aid will no longer attempt to use the Insomnia skill and will eat a kawhe pill.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::UseInsomnia(false)]);
    }

    #[test]
    fn test_adder() {
        let line = "Adder:              OFF";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Adder(None)]);
        let line = "Adder:              30 seconds";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Adder(Some(30))]);
    }

    #[test]
    fn test_change_adder() {
        let line = "FirstAid will no longer attempt to RIP CARD FROM BODY.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Adder(None)]);
        let line = "FirstAid will now send RIP CARD FROM BODY when 3 seconds have passed after receiving the affliction.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Adder(Some(3))]);
    }

    #[test]
    fn test_stormtouched() {
        let line = "Stormtouched:       OFF";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Stormtouched(false)]);
        let line = "Stormtouched:       ON";
        let settings = FirstAidSetting::get_setting_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Stormtouched(true)]);
    }

    #[test]
    fn test_change_stormtouched() {
        let line = "First Aid will no longer utilize stormtouched mode curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Stormtouched(false)]);
        let line = "First Aid will now utilize stormtouched mode curing.";
        let settings = FirstAidSetting::get_from_line(line);
        assert_eq!(settings, vec![FirstAidSetting::Stormtouched(true)]);
    }
}
