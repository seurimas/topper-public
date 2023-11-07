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
