use crate::non_agent::persuasion::*;

use super::{AgentState, FType, SType};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RhetoricState {
    #[default]
    NoRhetoric,
    Started, // No appeal type yet.
    TwoLeft(AppealType),
    OneLeft(AppealType),
    BonusActive(AppealType),
}

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
    rhetoric_state: RhetoricState,
}

impl PersuasionState {
    pub fn is(&self, aff: FType) -> bool {
        self.flags[aff as usize - FType::Conflicted as usize]
    }

    pub fn set(&mut self, aff: FType, val: bool) {
        self.flags[aff as usize - FType::Conflicted as usize] = val;
    }

    pub fn rhetoric_start(&mut self) {
        self.rhetoric_state = RhetoricState::Started;
    }

    pub fn appeal(&mut self, appeal: Appeals) {
        let was_reasoned = self
            .last_appeal
            .map_or(false, |(app, _)| app == Appeals::Reason);
        let same_type = self
            .last_appeal
            .map_or(false, |(app, _)| app.appeal_type() == appeal.appeal_type());
        // Advance rhetoric, if it exists.
        match self.rhetoric_state {
            RhetoricState::Started => {
                self.rhetoric_state = RhetoricState::TwoLeft(appeal.appeal_type());
            }
            RhetoricState::TwoLeft(_) => {
                if same_type {
                    self.rhetoric_state = RhetoricState::OneLeft(appeal.appeal_type());
                } else {
                    self.rhetoric_state = RhetoricState::NoRhetoric;
                }
            }
            RhetoricState::OneLeft(_) => {
                if same_type {
                    self.rhetoric_state = RhetoricState::BonusActive(appeal.appeal_type());
                } else {
                    self.rhetoric_state = RhetoricState::NoRhetoric;
                }
            }
            RhetoricState::BonusActive(_) => {
                self.rhetoric_state = RhetoricState::NoRhetoric;
            }
            _ => {}
        }
        // Put the appeal in discard, unless it's cyclical.
        if self.cyclic == Some(appeal) {
            self.appeals_in_deck.push(appeal);
        } else {
            self.discarded_appeals.push(appeal);
        }
        // Remember the appeal.
        self.last_appeal = Some((appeal, was_reasoned));
    }

    pub fn drawn(&mut self, appeal: Appeals) {
        self.appeals_in_hand.push(appeal);
        if self.appeals_in_deck.contains(&appeal) {
            self.appeals_in_deck.retain(|&x| x != appeal);
        } else {
            if !self.appeals_in_deck.is_empty() {
                println!("Mystery appeal drawn: {:?}", appeal);
            }
            self.appeals_in_deck = self.discarded_appeals.clone();
            self.discarded_appeals.clear();
            self.appeals_in_deck.retain(|&x| x != appeal);
        }
    }

    pub fn discard(&mut self, appeal: Appeals) {
        self.discarded_appeals.push(appeal);
        self.appeals_in_hand.retain(|&x| x != appeal);
    }
}
