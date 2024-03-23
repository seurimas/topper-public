use super::super::statics::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use topper_core::observations::strip_ansi;
use topper_core::timeline::BaseAgentState;

impl ActiveTransition for SimpleCureAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![AetObservation::SimpleCureAction(self.clone())])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        match &self.cure_type {
            SimpleCure::Pill(pill) => Ok(format!("eat {}", pill)),
            SimpleCure::Salve(salve, location) => Ok(format!("apply {} to {}", salve, location)),
            SimpleCure::Smoke(herb) => Ok(format!("smoke {}", herb)),
        }
    }
}

pub struct FocusAction {
    caster: String,
}

impl FocusAction {
    pub fn new(caster: &str) -> Self {
        FocusAction {
            caster: caster.to_string(),
        }
    }
}

impl ActiveTransition for FocusAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Survival",
            &"Focus",
            &"",
            &"",
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok("focus".to_string())
    }
}

pub struct TreeAction {
    caster: String,
}

impl TreeAction {
    pub fn new(caster: &str) -> Self {
        TreeAction {
            caster: caster.to_string(),
        }
    }
}

impl ActiveTransition for TreeAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        ProbableEvent::certain(vec![CombatAction::observation(
            &self.caster,
            &"Tattoos",
            &"Tree",
            &"",
            &"",
        )])
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok("touch tree".to_string())
    }
}

pub enum FirstAidAction {
    Simple(SimpleCureAction),
    Focus(FocusAction),
    Tree(TreeAction),
    Wait,
}

impl FirstAidAction {
    pub fn is_tree(&self) -> bool {
        match self {
            FirstAidAction::Tree(_) => true,
            _ => false,
        }
    }
    pub fn is_focus(&self) -> bool {
        match self {
            FirstAidAction::Focus(_) => true,
            _ => false,
        }
    }
}

impl ActiveTransition for FirstAidAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        match self {
            FirstAidAction::Simple(action) => action.simulate(&timeline),
            FirstAidAction::Focus(action) => action.simulate(&timeline),
            FirstAidAction::Tree(action) => action.simulate(&timeline),
            FirstAidAction::Wait => vec![],
        }
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        match self {
            FirstAidAction::Simple(action) => action.act(&timeline),
            FirstAidAction::Focus(action) => action.act(&timeline),
            FirstAidAction::Tree(action) => action.act(&timeline),
            FirstAidAction::Wait => Ok("".to_string()),
        }
    }
}
