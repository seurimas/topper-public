use crate::non_agent::persuasion::*;

use super::{AgentState, FType, SType};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PersuasionState {
    pub max_acumen: i32,
    pub acumen: i32,
    pub int: Option<i32>,
    pub wis: Option<i32>,
    pub str: Option<i32>,
    pub last_appeal: Option<(Appeals, bool)>,
    pub target: Option<i64>,
    pub appeals_in_hand: Vec<Appeals>,
    pub discarded_appeals: Vec<Appeals>,
    pub appeals_in_deck: Vec<Appeals>,
    pub cyclic: Option<Appeals>,
    flags: [bool; 12],
    influence_stacks: i32,
}

impl PersuasionState {
    pub fn is(&self, aff: FType) -> bool {
        self.flags[aff as usize - FType::Conflicted as usize]
    }

    pub fn set(&mut self, aff: FType, val: bool) {
        self.flags[aff as usize - FType::Conflicted as usize] = val;
    }
}
