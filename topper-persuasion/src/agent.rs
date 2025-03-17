use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RhetoricState {
    #[default]
    NoRhetoric,
    Started, // No appeal type yet.
    TwoLeft(AppealType),
    OneLeft(AppealType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersuasionState {
    pub max_acumen: i32,
    pub acumen: i32,
    pub int: Option<i32>,
    pub wis: Option<i32>,
    pub str: Option<i32>,
    pub last_appeal: Option<(Appeals, bool)>,
    target: Option<i64>,
    pub appeals_in_hand: Vec<Appeals>,
    pub discarded_appeals: Vec<Appeals>,
    pub appeals_in_deck: Vec<Appeals>,
    pub cyclic: Option<Appeals>,
    flags: [bool; 13],
    pub influence_stacks: i32,
    rhetoric_state: RhetoricState,
    rhetoric_bonus: Option<AppealType>,
    conflicted_type: Option<AppealType>,
    entrenched_type: Option<AppealType>,
    prudence_cooldown: i32,
}

impl Default for PersuasionState {
    fn default() -> Self {
        PersuasionState {
            max_acumen: 5000,
            acumen: 5000,
            int: None,
            wis: None,
            str: None,
            last_appeal: None,
            target: None,
            appeals_in_hand: vec![],
            discarded_appeals: vec![],
            appeals_in_deck: vec![],
            cyclic: None,
            flags: [false; 13],
            influence_stacks: 0,
            rhetoric_state: RhetoricState::NoRhetoric,
            rhetoric_bonus: None,
            conflicted_type: None,
            entrenched_type: None,
            prudence_cooldown: 0,
        }
    }
}

impl PersuasionState {
    pub fn start_persuasion(&mut self, target: i64) {
        self.target = Some(target);
        self.appeals_in_hand = vec![];
        self.discarded_appeals = vec![];
        self.appeals_in_deck = vec![
            Appeals::Authority,
            Appeals::Morality,
            Appeals::Reputation,
            Appeals::Tradition,
            Appeals::Evidence,
            Appeals::Reason,
            Appeals::Analogy,
            Appeals::Causality,
            Appeals::Intimidation,
            Appeals::Reassurance,
            Appeals::Inspiration,
            Appeals::Provocation,
        ];
        self.rhetoric_bonus = None;
        self.rhetoric_state = RhetoricState::NoRhetoric;
    }

    pub fn finish_persuasion(&mut self) {
        self.target = None;
        self.appeals_in_hand = vec![];
        self.discarded_appeals = vec![];
        self.appeals_in_deck = vec![];
        self.flags = [false; 13];
        self.influence_stacks = 0;
        self.rhetoric_state = RhetoricState::NoRhetoric;
    }

    pub fn get_target(&self) -> Option<i64> {
        self.target
    }

    pub fn is(&self, aff: PersuasionAff) -> bool {
        self.flags[aff as usize]
    }

    pub fn set(&mut self, aff: PersuasionAff, val: bool) {
        self.flags[aff as usize] = val;
    }

    pub fn reasoned(&self) -> bool {
        self.last_appeal
            .map_or(false, |(appeal, _)| appeal == Appeals::Reason)
    }

    pub fn analogizing(&self) -> bool {
        self.last_appeal
            .map_or(false, |(appeal, _)| appeal == Appeals::Analogy)
    }

    pub fn get_rhetoric_state(&self) -> RhetoricState {
        self.rhetoric_state.clone()
    }

    pub fn rhetoric_start(&mut self) {
        self.rhetoric_state = RhetoricState::Started;
    }

    pub fn rhetoric_end(&mut self) {
        self.rhetoric_state = RhetoricState::NoRhetoric;
    }

    pub fn would_conflict(&self, appeal: Appeals) -> bool {
        self.is(PersuasionAff::Conflicted) && self.conflicted_type == Some(appeal.appeal_type())
    }

    pub fn would_entrench(&self, appeal: Appeals) -> bool {
        self.is(PersuasionAff::Entrenched) && self.entrenched_type != Some(appeal.appeal_type())
    }

    pub fn would_unrhetoric(&self, appeal: Appeals) -> bool {
        match self.rhetoric_state {
            RhetoricState::TwoLeft(t) => t != appeal.appeal_type(),
            RhetoricState::OneLeft(t) => t != appeal.appeal_type(),
            _ => false,
        }
    }

    pub fn rhetoric_just_started(&self) -> bool {
        match self.rhetoric_state {
            RhetoricState::Started => true,
            _ => false,
        }
    }

    pub fn would_follow_rhetoric(&self, appeal: Appeals) -> bool {
        match self.rhetoric_state {
            RhetoricState::TwoLeft(t) => t == appeal.appeal_type(),
            RhetoricState::OneLeft(t) => t == appeal.appeal_type(),
            _ => false,
        }
    }

    pub fn would_finish_rhetoric(&self, appeal: Appeals) -> bool {
        match self.rhetoric_state {
            RhetoricState::OneLeft(t) => t == appeal.appeal_type(),
            _ => false,
        }
    }

    pub fn in_rhetoric(&self) -> bool {
        match self.rhetoric_state {
            RhetoricState::Started => true,
            RhetoricState::TwoLeft(_) => true,
            RhetoricState::OneLeft(_) => true,
            _ => false,
        }
    }

    pub fn has_ethos_bonus(&self) -> bool {
        self.rhetoric_bonus == Some(AppealType::Ethos)
    }

    pub fn has_pathos_bonus(&self) -> bool {
        self.rhetoric_bonus == Some(AppealType::Pathos)
    }

    pub fn has_logos_bonus(&self) -> bool {
        self.rhetoric_bonus == Some(AppealType::Logos)
    }

    pub fn use_bonus(&mut self) {
        self.rhetoric_bonus = None;
    }

    pub fn could_follow_rhetoric(&self, appeal_type: AppealType) -> bool {
        if self
            .appeals_in_hand
            .iter()
            .filter(|&x| x.appeal_type() == appeal_type)
            .count()
            > 2
        {
            return true;
        } else if self
            .appeals_in_hand
            .iter()
            .filter(|&x| x.appeal_type() == appeal_type)
            .count()
            > 1
            && self
                .appeals_in_deck
                .iter()
                .filter(|&x| x.appeal_type() == appeal_type)
                .count()
                > 0
            && self.appeals_in_deck.len() < 3
        {
            return true;
        }
        false
    }

    pub fn could_follow_any_rhetoric(&self) -> bool {
        vec![AppealType::Logos, AppealType::Ethos, AppealType::Pathos]
            .iter()
            .any(|&x| self.could_follow_rhetoric(x))
    }

    pub fn cyclic_in_discard(&self) -> bool {
        if let Some(cyclic) = self.cyclic {
            self.discarded_appeals.contains(&cyclic)
        } else {
            false
        }
    }

    pub fn first_not_in_discard(&self, options: &[Appeals]) -> Option<Appeals> {
        options
            .iter()
            .find(|&x| !self.discarded_appeals.contains(x))
            .cloned()
    }

    fn advance_rhetoric(&mut self, appeal: Appeals) {
        // Advance rhetoric, if it exists.
        match self.rhetoric_state {
            RhetoricState::Started => {
                self.rhetoric_state = RhetoricState::TwoLeft(appeal.appeal_type());
            }
            RhetoricState::TwoLeft(rhetoric_type) => {
                if rhetoric_type == appeal.appeal_type() {
                    self.rhetoric_state = RhetoricState::OneLeft(appeal.appeal_type());
                } else {
                    self.rhetoric_state = RhetoricState::NoRhetoric;
                }
            }
            RhetoricState::OneLeft(rhetoric_type) => {
                if rhetoric_type == appeal.appeal_type() {
                    self.rhetoric_bonus = Some(appeal.appeal_type());
                    self.rhetoric_state = RhetoricState::NoRhetoric;
                } else {
                    self.rhetoric_state = RhetoricState::NoRhetoric;
                }
            }
            _ => {}
        }
    }

    pub fn appeal(&mut self, appeal: Appeals) {
        let was_reasoned = self
            .last_appeal
            .map_or(false, |(app, _)| app == Appeals::Reason);
        if self.is(PersuasionAff::Conflicted) {
            self.conflicted_type = Some(appeal.appeal_type());
        }
        if self.is(PersuasionAff::Entrenched) {
            self.entrenched_type = Some(appeal.appeal_type());
        }
        // Put the appeal in discard, unless it's cyclical.
        if self.has_logos_bonus() {
            self.use_bonus();
            self.last_appeal = Some((appeal, was_reasoned));
            self.advance_rhetoric(appeal);
            return; // Don't discard.
        } else if self.cyclic == Some(appeal) {
            self.appeals_in_deck.push(appeal);
        } else {
            self.discarded_appeals.push(appeal);
        }
        self.use_bonus();
        self.appeals_in_hand.retain(|&x| x != appeal);
        // Remember the appeal.
        self.last_appeal = Some((appeal, was_reasoned));
        self.advance_rhetoric(appeal);
    }

    pub fn drawn(&mut self, appeal: Appeals) {
        self.appeals_in_hand.push(appeal);
        if self.appeals_in_deck.contains(&appeal) {
            self.appeals_in_deck.retain(|&x| x != appeal);
        } else {
            if !self.appeals_in_deck.is_empty() {
                println!(
                    "Mystery appeal drawn: {:?} - {:?}",
                    appeal, self.appeals_in_deck
                );
            }
            println!("Shuffling appeals.");
            self.appeals_in_deck = self.discarded_appeals.clone();
            self.discarded_appeals.clear();
            self.appeals_in_deck.retain(|&x| x != appeal);
        }
    }

    pub fn discard(&mut self, appeal: Appeals) {
        self.discarded_appeals.push(appeal);
        self.appeals_in_hand.retain(|&x| x != appeal);
    }

    pub fn sip_prudence(&mut self) {
        self.prudence_cooldown = 650;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persuasion_state() {
        let mut state = PersuasionState::default();
        state.start_persuasion(1);
        assert_eq!(state.get_target(), Some(1));
        assert_eq!(state.appeals_in_hand.len(), 0);
        assert_eq!(state.discarded_appeals.len(), 0);
        assert_eq!(state.appeals_in_deck.len(), 12);
        state.drawn(Appeals::Authority);
        state.drawn(Appeals::Analogy);
        assert_eq!(
            state.appeals_in_hand,
            vec![Appeals::Authority, Appeals::Analogy]
        );
        state.finish_persuasion();
        assert_eq!(state.get_target(), None);
        assert_eq!(state.appeals_in_hand.len(), 0);
        assert_eq!(state.discarded_appeals.len(), 0);
        assert_eq!(state.appeals_in_deck.len(), 0);
        assert_eq!(state.flags, [false; 13]);
        assert_eq!(state.influence_stacks, 0);
        assert_eq!(state.rhetoric_state, RhetoricState::NoRhetoric);
        assert!(!state.is(PersuasionAff::Conflicted));
        state.set(PersuasionAff::Conflicted, true);
        assert!(state.is(PersuasionAff::Conflicted));
        assert!(!state.reasoned());
        state.appeal(Appeals::Reason);
        assert!(state.reasoned());
        state.rhetoric_start();
        assert_eq!(state.rhetoric_state, RhetoricState::Started);
        state.appeal(Appeals::Reason);
        assert_eq!(
            state.rhetoric_state,
            RhetoricState::TwoLeft(AppealType::Logos)
        );
        state.appeal(Appeals::Reason);
        assert_eq!(
            state.rhetoric_state,
            RhetoricState::OneLeft(AppealType::Logos)
        );
        state.appeal(Appeals::Reason);
        assert_eq!(state.rhetoric_state, RhetoricState::NoRhetoric);
        assert!(state.has_logos_bonus());
        state.appeal(Appeals::Reason);
        assert_eq!(state.rhetoric_state, RhetoricState::NoRhetoric);
        state.drawn(Appeals::Reason);
        assert_eq!(state.appeals_in_hand.len(), 1);
        state.discard(Appeals::Reason);
        assert_eq!(state.appeals_in_hand.len(), 0);
        assert_eq!(state.discarded_appeals.len(), 1);
    }

    #[test]
    fn test_drawing_bug() {
        let mut state = PersuasionState::default();
        state.drawn(Appeals::Intimidation);
        state.drawn(Appeals::Authority);
        state.drawn(Appeals::Tradition);
        // Retort
        state.discard(Appeals::Intimidation);
        state.drawn(Appeals::Reason);
        // Appeal
        state.appeal(Appeals::Authority);
        state.drawn(Appeals::Provocation);
        // Appeal
        state.appeal(Appeals::Provocation);
        state.drawn(Appeals::Reputation);
        // Appeal
        state.appeal(Appeals::Reputation);
        state.drawn(Appeals::Evidence);
        // Appeal
        state.appeal(Appeals::Reason);
        state.drawn(Appeals::Morality);
        // Appeal
        state.appeal(Appeals::Morality);
        state.drawn(Appeals::Reassurance);
        state.drawn(Appeals::Inspiration);
        // Appeals
        state.appeal(Appeals::Tradition);
        // Appeals
        state.appeal(Appeals::Inspiration);
        state.drawn(Appeals::Causality);
        state.rhetoric_start();
        state.rhetoric_end();
        // Appeals
        state.appeal(Appeals::Causality);
        state.drawn(Appeals::Analogy);
        assert_eq!(state.appeals_in_hand.len(), 3);
        // Appeals
        state.appeal(Appeals::Evidence);
        state.drawn(Appeals::Tradition);
        // Appeals
        state.appeal(Appeals::Analogy);
        state.drawn(Appeals::Causality);
        // Appeals
        state.appeal(Appeals::Causality);
        state.drawn(Appeals::Intimidation);
        assert_eq!(state.appeals_in_hand.len(), 3);
    }
}
