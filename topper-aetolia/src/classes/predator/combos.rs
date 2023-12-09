use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::*;
use crate::types::*;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComboAttack {
    Tidalslash,
    Freefall,
    Pheromones,
    Pindown,
    Mindnumb,
    Jab,
    Pinprick,
    Lateral,
    Vertical,
    Crescentcut,
    Spinslash,
    Lowhook,
    Butterfly,
    Flashkick,
    Trip,
    Veinrip,
    Feint,
    Raze,
    Gouge,
    Bleed,
    Swiftkick,
}

impl ComboAttack {
    pub fn is_combo_attack(&self) -> bool {
        match self {
            ComboAttack::Tidalslash => false,
            ComboAttack::Freefall => false,
            ComboAttack::Pheromones => false,
            ComboAttack::Pindown => false,
            ComboAttack::Mindnumb => false,
            _ => true,
        }
    }

    pub fn rebounds(&self) -> bool {
        match self {
            ComboAttack::Freefall => false,
            ComboAttack::Pheromones => false,
            ComboAttack::Pindown => false,
            ComboAttack::Mindnumb => false,
            ComboAttack::Raze => false,
            _ => true,
        }
    }

    pub fn strips_rebounding(&self) -> bool {
        match self {
            ComboAttack::Raze => true,
            _ => false,
        }
    }

    pub fn get_aff_count(&self) -> usize {
        if self.can_use_venom() {
            1
        } else {
            match self {
                ComboAttack::Pinprick => 1,
                ComboAttack::Flashkick => 1,
                ComboAttack::Veinrip => 2,
                ComboAttack::Gouge => 1,
                ComboAttack::Pheromones => 1,
                ComboAttack::Mindnumb => 1,
                _ => 0,
            }
        }
    }

    // Combo attacks where using it twice is impossible or an anti-pattern.
    pub fn idempotent(&self) -> bool {
        match self {
            ComboAttack::Tidalslash => true,
            ComboAttack::Freefall => true,
            ComboAttack::Pheromones => true,
            ComboAttack::Pindown => true,
            ComboAttack::Mindnumb => true,
            ComboAttack::Pinprick => true,
            ComboAttack::Veinrip => true,
            ComboAttack::Feint => true,
            ComboAttack::Gouge => true,
            ComboAttack::Vertical => true,
            _ => false,
        }
    }

    pub fn parryable(&self) -> bool {
        match self {
            ComboAttack::Jab
            | ComboAttack::Lateral
            | ComboAttack::Lowhook
            | ComboAttack::Flashkick
            | ComboAttack::Veinrip
            | ComboAttack::Gouge => true,
            _ => false,
        }
    }

    pub fn get_single_limb_target(&self) -> Option<LType> {
        match self {
            ComboAttack::Lateral => Some(LType::TorsoDamage),
            ComboAttack::Flashkick => Some(LType::HeadDamage),
            ComboAttack::Veinrip => Some(LType::HeadDamage),
            ComboAttack::Gouge => Some(LType::HeadDamage),
            _ => None,
        }
    }

    pub fn can_hit(&self, limb: LType) -> bool {
        if Some(limb) == self.get_single_limb_target() {
            true
        } else {
            match (self, limb) {
                (ComboAttack::Jab, LType::LeftArmDamage) => true,
                (ComboAttack::Jab, LType::RightArmDamage) => true,
                (ComboAttack::Lowhook, LType::LeftArmDamage) => true,
                (ComboAttack::Lowhook, LType::RightArmDamage) => true,
                (ComboAttack::Spinslash, _) => true,
                _ => false,
            }
        }
    }

    pub fn get_limb_damage(&self) -> CType {
        match self {
            ComboAttack::Lateral => 600,
            ComboAttack::Flashkick => 500,
            ComboAttack::Veinrip => 200,
            ComboAttack::Gouge => 650,
            ComboAttack::Lowhook => 550,
            ComboAttack::Jab => 550,
            ComboAttack::Spinslash => 400,
            _ => 0,
        }
    }

    pub fn can_drop_parry(&self) -> bool {
        if self == &ComboAttack::Feint || self == &ComboAttack::Pindown {
            true
        } else {
            false
        }
    }

    pub fn requires_prone(&self) -> bool {
        match self {
            ComboAttack::Pindown => true,
            ComboAttack::Crescentcut => true, // Technically not true, but it's a good idea.
            _ => false,
        }
    }

    pub fn can_prone(&self) -> bool {
        match self {
            ComboAttack::Trip => true,
            _ => false,
        }
    }

    pub fn can_use_venom(&self) -> bool {
        match self {
            ComboAttack::Freefall
            | ComboAttack::Vertical
            | ComboAttack::Crescentcut
            | ComboAttack::Butterfly => true,
            _ => false,
        }
    }

    pub fn is_good_combo_attack(&self, stance: Stance) -> bool {
        if *self == ComboAttack::Raze || stance == Stance::Bladesurge {
            return true;
        }
        !self.is_combo_attack() || self.get_next_stance(stance) != stance
    }

    pub fn must_begin_combo(&self) -> bool {
        match self {
            ComboAttack::Freefall => true,
            ComboAttack::Pindown => true,
            ComboAttack::Butterfly => true,
            _ => false,
        }
    }

    pub fn should_end_combo(&self) -> bool {
        match self {
            ComboAttack::Feint => false,
            _ => true,
        }
    }

    pub fn get_balance_time(&self, stance: Stance) -> CType {
        let base = match self {
            ComboAttack::Jab => 205,
            ComboAttack::Pinprick => 205,
            ComboAttack::Lateral => 205,
            ComboAttack::Lowhook => 205,
            ComboAttack::Feint => 205,
            ComboAttack::Raze => 205,
            ComboAttack::Pheromones => 205,
            ComboAttack::Mindnumb => 251,
            ComboAttack::Vertical => 251,
            ComboAttack::Spinslash => 251,
            ComboAttack::Trip => 298,
            ComboAttack::Gouge => 298,
            ComboAttack::Tidalslash => 300, // Guess
            ComboAttack::Butterfly => 300,  // Guess
            ComboAttack::Bleed => 344,
            ComboAttack::Swiftkick => 344,
            ComboAttack::Crescentcut => 367,
            ComboAttack::Pindown => 372,
            ComboAttack::Freefall => 391,
            ComboAttack::Flashkick => 391,
            ComboAttack::Veinrip => 391,
        };
        if !self.is_good_combo_attack(stance) {
            base + 40
        } else if stance == Stance::Laesan || stance == Stance::Bladesurge {
            base - 40
        } else {
            base
        }
    }

    pub fn get_next_stance(&self, stance: Stance) -> Stance {
        match (self, stance) {
            // Bladesurge stays in bladesurge.
            (_, Stance::Bladesurge) => Stance::Bladesurge,
            // Non knifeplay attacks.
            (ComboAttack::Tidalslash, _) => stance,
            (ComboAttack::Freefall, _) => stance,
            (ComboAttack::Pheromones, _) => stance,
            (ComboAttack::Pindown, _) => stance,
            (ComboAttack::Mindnumb, _) => stance,
            // Jab
            (ComboAttack::Jab, Stance::None) => Stance::Gyanis,
            (ComboAttack::Jab, Stance::Gyanis) => Stance::Rizet,
            (ComboAttack::Jab, Stance::VaeSant) => Stance::Gyanis,
            (ComboAttack::Jab, Stance::Rizet) => stance,
            (ComboAttack::Jab, Stance::EinFasit) => Stance::VaeSant,
            (ComboAttack::Jab, Stance::Laesan) => Stance::Rizet,
            // Pinprick
            (ComboAttack::Pinprick, Stance::None) => Stance::Gyanis,
            (ComboAttack::Pinprick, Stance::Gyanis) => Stance::Rizet,
            (ComboAttack::Pinprick, Stance::VaeSant) => Stance::Rizet,
            (ComboAttack::Pinprick, Stance::Rizet) => Stance::VaeSant,
            (ComboAttack::Pinprick, Stance::EinFasit) => stance,
            (ComboAttack::Pinprick, Stance::Laesan) => Stance::Gyanis,
            // Lateral
            (ComboAttack::Lateral, Stance::None) => Stance::Gyanis,
            (ComboAttack::Lateral, Stance::Gyanis) => Stance::VaeSant,
            (ComboAttack::Lateral, Stance::VaeSant) => Stance::EinFasit,
            (ComboAttack::Lateral, Stance::Rizet) => Stance::EinFasit,
            (ComboAttack::Lateral, Stance::EinFasit) => Stance::Laesan,
            (ComboAttack::Lateral, Stance::Laesan) => stance,
            // Vertical
            (ComboAttack::Vertical, Stance::None) => Stance::Laesan,
            (ComboAttack::Vertical, Stance::Gyanis) => Stance::Laesan,
            (ComboAttack::Vertical, Stance::VaeSant) => Stance::Rizet,
            (ComboAttack::Vertical, Stance::Rizet) => Stance::EinFasit,
            (ComboAttack::Vertical, Stance::EinFasit) => Stance::VaeSant,
            (ComboAttack::Vertical, Stance::Laesan) => stance,
            // Crescentcut
            (ComboAttack::Crescentcut, Stance::None) => Stance::VaeSant,
            (ComboAttack::Crescentcut, Stance::Gyanis) => Stance::EinFasit,
            (ComboAttack::Crescentcut, Stance::VaeSant) => stance,
            (ComboAttack::Crescentcut, Stance::Rizet) => Stance::Laesan,
            (ComboAttack::Crescentcut, Stance::EinFasit) => Stance::Gyanis,
            (ComboAttack::Crescentcut, Stance::Laesan) => Stance::VaeSant,
            // Spinslash
            (ComboAttack::Spinslash, Stance::None) => Stance::VaeSant,
            (ComboAttack::Spinslash, Stance::Gyanis) => Stance::VaeSant,
            (ComboAttack::Spinslash, Stance::VaeSant) => Stance::EinFasit,
            (ComboAttack::Spinslash, Stance::Rizet) => Stance::Laesan,
            (ComboAttack::Spinslash, Stance::EinFasit) => stance,
            (ComboAttack::Spinslash, Stance::Laesan) => Stance::EinFasit,
            // Lowhook
            (ComboAttack::Lowhook, Stance::None) => Stance::VaeSant,
            (ComboAttack::Lowhook, Stance::Gyanis) => Stance::VaeSant,
            (ComboAttack::Lowhook, Stance::VaeSant) => Stance::Gyanis,
            (ComboAttack::Lowhook, Stance::Rizet) => stance,
            (ComboAttack::Lowhook, Stance::EinFasit) => Stance::Gyanis,
            (ComboAttack::Lowhook, Stance::Laesan) => Stance::Gyanis,
            // Butterfly
            (ComboAttack::Butterfly, Stance::None) => Stance::Rizet,
            (ComboAttack::Butterfly, Stance::Gyanis) => stance,
            (ComboAttack::Butterfly, Stance::VaeSant) => Stance::Gyanis,
            (ComboAttack::Butterfly, Stance::Rizet) => Stance::Gyanis,
            (ComboAttack::Butterfly, Stance::EinFasit) => Stance::Laesan,
            (ComboAttack::Butterfly, Stance::Laesan) => Stance::Rizet,
            // Flashkick
            (ComboAttack::Flashkick, Stance::None) => Stance::Rizet,
            (ComboAttack::Flashkick, Stance::Gyanis) => Stance::Rizet,
            (ComboAttack::Flashkick, Stance::VaeSant) => Stance::Laesan,
            (ComboAttack::Flashkick, Stance::Rizet) => stance,
            (ComboAttack::Flashkick, Stance::EinFasit) => Stance::Laesan,
            (ComboAttack::Flashkick, Stance::Laesan) => Stance::VaeSant,
            // Trip
            (ComboAttack::Trip, Stance::None) => Stance::EinFasit,
            (ComboAttack::Trip, Stance::Gyanis) => Stance::VaeSant,
            (ComboAttack::Trip, Stance::VaeSant) => stance,
            (ComboAttack::Trip, Stance::Rizet) => Stance::Gyanis,
            (ComboAttack::Trip, Stance::EinFasit) => Stance::Gyanis,
            (ComboAttack::Trip, Stance::Laesan) => Stance::Rizet,
            // Veinrip
            (ComboAttack::Veinrip, Stance::None) => stance,
            (ComboAttack::Veinrip, Stance::Gyanis) => Stance::EinFasit,
            (ComboAttack::Veinrip, Stance::VaeSant) => Stance::EinFasit,
            (ComboAttack::Veinrip, Stance::Rizet) => Stance::Gyanis,
            (ComboAttack::Veinrip, Stance::EinFasit) => Stance::Laesan,
            (ComboAttack::Veinrip, Stance::Laesan) => Stance::VaeSant,
            // Feint
            (ComboAttack::Feint, Stance::None) => Stance::EinFasit,
            (ComboAttack::Feint, Stance::Gyanis) => Stance::EinFasit,
            (ComboAttack::Feint, Stance::VaeSant) => Stance::Laesan,
            (ComboAttack::Feint, Stance::Rizet) => stance,
            (ComboAttack::Feint, Stance::EinFasit) => Stance::Gyanis,
            (ComboAttack::Feint, Stance::Laesan) => Stance::EinFasit,
            // Raze
            (ComboAttack::Raze, Stance::None) => Stance::Laesan,
            (ComboAttack::Raze, Stance::Gyanis) => Stance::Laesan,
            (ComboAttack::Raze, Stance::VaeSant) => stance,
            (ComboAttack::Raze, Stance::Rizet) => Stance::VaeSant,
            (ComboAttack::Raze, Stance::EinFasit) => Stance::Rizet,
            (ComboAttack::Raze, Stance::Laesan) => Stance::EinFasit,
            // Gouge
            (ComboAttack::Gouge, Stance::None) => Stance::Laesan,
            (ComboAttack::Gouge, Stance::Gyanis) => Stance::EinFasit,
            (ComboAttack::Gouge, Stance::VaeSant) => Stance::Gyanis,
            (ComboAttack::Gouge, Stance::Rizet) => Stance::VaeSant,
            (ComboAttack::Gouge, Stance::EinFasit) => Stance::Rizet,
            (ComboAttack::Gouge, Stance::Laesan) => stance,
            // Bleed
            (ComboAttack::Bleed, Stance::None) => stance,
            (ComboAttack::Bleed, Stance::Gyanis) => Stance::Laesan,
            (ComboAttack::Bleed, Stance::VaeSant) => Stance::Rizet,
            (ComboAttack::Bleed, Stance::Rizet) => Stance::EinFasit,
            (ComboAttack::Bleed, Stance::EinFasit) => stance,
            (ComboAttack::Bleed, Stance::Laesan) => Stance::VaeSant,
            // Swiftkick
            (ComboAttack::Swiftkick, Stance::None) => Stance::Gyanis,
            (ComboAttack::Swiftkick, Stance::Gyanis) => Stance::Laesan,
            (ComboAttack::Swiftkick, Stance::VaeSant) => Stance::EinFasit,
            (ComboAttack::Swiftkick, Stance::Rizet) => stance,
            (ComboAttack::Swiftkick, Stance::EinFasit) => Stance::VaeSant,
            (ComboAttack::Swiftkick, Stance::Laesan) => Stance::Rizet,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PredatorCombo(Stance, Vec<ComboAttack>);

impl PredatorCombo {
    pub fn new(stance: Stance, attacks: Vec<ComboAttack>) -> Self {
        Self(stance, attacks)
    }

    pub fn get_attacks(&self) -> &Vec<ComboAttack> {
        &self.1
    }

    pub fn get_starting_stance(&self) -> Stance {
        self.0
    }

    pub fn get_final_stance(&self) -> Stance {
        self.1
            .iter()
            .fold(self.0, |stance, attack| attack.get_next_stance(stance))
    }

    pub fn get_balance_time(&self) -> CType {
        self.1
            .iter()
            .fold((self.0, 0), |(stance, balance), attack| {
                (
                    attack.get_next_stance(stance),
                    CType::max(balance, attack.get_balance_time(stance)),
                )
            })
            .1
    }

    pub fn estimate_aff_rate(&self) -> f32 {
        let balance = self.get_balance_time();
        let affs = self
            .1
            .iter()
            .fold(0, |affs, attack| affs + attack.get_aff_count());
        affs as f32 / balance as f32
    }

    pub fn score_combo(&self, graders: &Vec<ComboGrader>) -> i32 {
        graders
            .iter()
            .fold(0, |score, grader| score + grader.grade(self))
    }
}

#[derive(Debug)]
pub struct ComboSolver {
    attacks: Vec<ComboAttack>,
    starting_stance: Stance,
    start_parry: bool,
    start_prone: bool,
    start_rebounds: u32,
    blade_surge: bool,
    allow_bad_stances: bool,
    allow_parries: bool,
}

impl Default for ComboSolver {
    fn default() -> Self {
        Self::new(Stance::None)
    }
}

impl ComboSolver {
    pub fn new(stance: Stance) -> Self {
        Self {
            attacks: vec![],
            starting_stance: stance,
            start_parry: false,
            start_prone: false,
            start_rebounds: 0,
            blade_surge: false,
            allow_bad_stances: false,
            allow_parries: false,
        }
    }

    pub fn set_stance(&mut self, stance: Stance) -> &mut Self {
        self.starting_stance = stance;
        self
    }

    pub fn set_attacks(&mut self, attacks: Vec<ComboAttack>) -> &mut Self {
        self.attacks = attacks;
        self
    }

    pub fn add_attacks<'a>(&mut self, attacks: impl Iterator<Item = &'a ComboAttack>) {
        self.attacks.extend(attacks);
    }

    pub fn set_parry(&mut self, parry: bool) -> &mut Self {
        self.start_parry = parry;
        self
    }

    pub fn set_prone(&mut self, prone: bool) -> &mut Self {
        self.start_prone = prone;
        self
    }

    pub fn set_rebounds(&mut self, rebounds: u32) -> &mut Self {
        self.start_rebounds = rebounds;
        self
    }

    pub fn set_blade_surge(&mut self, blade_surge: bool) -> &mut Self {
        self.blade_surge = blade_surge;
        self
    }

    pub fn set_allow_bad_stances(&mut self, allow_bad_stances: bool) -> &mut Self {
        self.allow_bad_stances = allow_bad_stances;
        self
    }

    pub fn set_allow_parries(&mut self, allow_parries: bool) -> &mut Self {
        self.allow_parries = allow_parries;
        self
    }

    fn add_combos(
        &self,
        combos: &mut Vec<PredatorCombo>,
        current_stance: Stance,
        attack: ComboAttack,
        previous_attacks: Vec<ComboAttack>,
        mut parrying: bool,
        mut prone: bool,
        mut rebounds: u32,
    ) {
        if combos.len() > 1000 {
            return;
        }
        let next_stance = attack.get_next_stance(current_stance);
        let mut new_attacks = previous_attacks.clone();
        new_attacks.push(attack);
        if attack.should_end_combo() {
            combos.push(PredatorCombo::new(
                self.starting_stance,
                new_attacks.clone(),
            ));
        }
        if new_attacks.len() == 3
            && next_stance != Stance::Laesan
            && next_stance != Stance::Bladesurge
        {
            return;
        } else if new_attacks.len() == 4 {
            return;
        }
        parrying &= !attack.can_drop_parry();
        prone |= attack.can_prone();
        if attack.strips_rebounding() && rebounds > 0 {
            rebounds -= 1;
        }
        for next_attack in self.attacks.iter() {
            self.add_next_attack(
                combos,
                next_attack,
                next_stance,
                &new_attacks,
                parrying,
                prone,
                rebounds,
            );
        }
    }

    fn add_next_attack(
        &self,
        combos: &mut Vec<PredatorCombo>,
        next_attack: &ComboAttack,
        next_stance: Stance,
        new_attacks: &Vec<ComboAttack>,
        parrying: bool,
        prone: bool,
        rebounds: u32,
    ) {
        if (self.allow_bad_stances || next_attack.is_good_combo_attack(next_stance))
            && (new_attacks.len() == 0 || !next_attack.must_begin_combo())
            && (!next_attack.idempotent() || !new_attacks.contains(&next_attack))
            && (!parrying || !next_attack.parryable() || self.allow_parries)
            && (prone || !next_attack.requires_prone())
            && (rebounds == 0 || !next_attack.rebounds())
        {
            self.add_combos(
                combos,
                next_stance,
                *next_attack,
                new_attacks.clone(),
                parrying,
                prone,
                rebounds,
            );
        }
    }

    pub fn find_combos(&self) -> ComboSet {
        let mut combos = vec![];
        for attack in self.attacks.iter() {
            self.add_next_attack(
                &mut combos,
                attack,
                self.starting_stance,
                &vec![],
                self.start_parry,
                self.start_prone,
                self.start_rebounds,
            );
        }
        ComboSet(combos)
    }
}

#[derive(Debug, Default)]
pub struct ComboSet(Vec<PredatorCombo>);

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ComboPredicate {
    WithAttack(ComboAttack),
    EndsInStance(Stance),
    MinimumAttacks(usize),
    MaxBalanceTime(CType),
    ScoreOver(i32),
}

impl ComboPredicate {
    pub fn matches(&self, combo: &PredatorCombo, score: Option<i32>) -> bool {
        match self {
            ComboPredicate::WithAttack(attack) => combo.get_attacks().contains(attack),
            ComboPredicate::EndsInStance(stance) => {
                combo.0 == Stance::Bladesurge || combo.get_final_stance() == *stance
            }
            ComboPredicate::MinimumAttacks(minimum) => combo.get_attacks().len() >= *minimum,
            ComboPredicate::MaxBalanceTime(max_balance) => combo.get_balance_time() <= *max_balance,
            ComboPredicate::ScoreOver(min_score) => {
                if let Some(score) = score {
                    score >= *min_score
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ComboGrader {
    Reuse(i32),
    Hits(LType, i32),
    ValueMove(ComboAttack, i32),
    ValueMoveInStance(ComboAttack, Stance, i32),
    HasVenom(i32),
    EndsInStance(Stance, i32),
}

impl ComboGrader {
    pub fn grade(&self, combo: &PredatorCombo) -> i32 {
        match self {
            ComboGrader::Reuse(value) => {
                let mut seen_hits = vec![];
                for attack in combo.get_attacks().iter() {
                    if seen_hits.contains(&attack) {
                        return *value;
                    } else {
                        seen_hits.push(attack);
                    }
                }
                0
            }
            ComboGrader::Hits(limb, value) => {
                let mut total_value = 0;
                for attack in combo.get_attacks().iter() {
                    if attack.can_hit(*limb) {
                        total_value += *value;
                    }
                }
                total_value
            }
            ComboGrader::ValueMove(attack, value) => {
                if combo.get_attacks().contains(attack) {
                    *value
                } else {
                    0
                }
            }
            ComboGrader::ValueMoveInStance(attack, stance, value) => {
                combo
                    .get_attacks()
                    .iter()
                    .fold((combo.0, 0), |(current_stance, total), combo_attack| {
                        if combo_attack == attack {
                            if combo.0 == *stance {
                                (combo_attack.get_next_stance(current_stance), total + *value)
                            } else {
                                (combo_attack.get_next_stance(current_stance), total)
                            }
                        } else {
                            (combo_attack.get_next_stance(current_stance), total)
                        }
                    })
                    .1
            }
            ComboGrader::HasVenom(value) => {
                for attack in combo.get_attacks().iter() {
                    if attack.can_use_venom() {
                        return *value;
                    }
                }
                return 0;
            }
            ComboGrader::EndsInStance(stance, value) => {
                if combo.0 == Stance::Bladesurge || combo.get_final_stance() == *stance {
                    *value
                } else {
                    0
                }
            }
        }
    }
}

impl ComboSet {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get_fastest_combo(&self, predicates: &Vec<ComboPredicate>) -> Option<PredatorCombo> {
        let mut fastest_combo = None;
        let mut fastest_time = CType::max_value();
        for combo in self.0.iter() {
            let mut valid = true;
            for predicate in predicates.iter() {
                if !predicate.matches(combo, None) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let balance_time = combo.get_balance_time();
                if balance_time < fastest_time {
                    fastest_time = balance_time;
                    fastest_combo = Some(combo);
                }
            }
        }
        fastest_combo.cloned()
    }

    pub fn get_highest_aff_rate_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
    ) -> Option<PredatorCombo> {
        let mut highest_combo = None;
        let mut highest_aff_rate = 0.0;
        for combo in self.0.iter() {
            let mut valid = true;
            for predicate in predicates.iter() {
                if !predicate.matches(combo, None) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let aff_rate = combo.estimate_aff_rate();
                if aff_rate > highest_aff_rate {
                    highest_aff_rate = aff_rate;
                    highest_combo = Some(combo);
                }
            }
        }
        highest_combo.cloned()
    }

    pub fn get_highest_scored_combo(
        &self,
        predicates: &Vec<ComboPredicate>,
        graders: &Vec<ComboGrader>,
    ) -> Option<PredatorCombo> {
        let mut highest_combo = None;
        let mut highest_score = 0.0;
        for combo in self.0.iter() {
            let mut valid = true;
            let score = combo.score_combo(graders);
            for predicate in predicates.iter() {
                if !predicate.matches(combo, Some(score)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let balance_score = score as f32 / (combo.get_balance_time() as f32);
                if balance_score > highest_score {
                    highest_score = balance_score;
                    highest_combo = Some(combo);
                }
            }
        }
        highest_combo.cloned()
    }
}

mod predator_tests {
    use super::*;

    #[test]
    pub fn test_find_combos() {
        let mut solver = ComboSolver::new(Stance::Rizet);
        solver
            .set_attacks(vec![
                ComboAttack::Jab,
                ComboAttack::Pinprick,
                ComboAttack::Mindnumb,
                ComboAttack::Vertical,
                ComboAttack::Veinrip,
                ComboAttack::Lowhook,
                ComboAttack::Pheromones,
                ComboAttack::Gouge,
                ComboAttack::Trip,
                ComboAttack::Raze,
            ])
            .set_prone(false)
            .set_parry(false)
            .set_rebounds(0);
        let combos = solver.find_combos();
        assert_eq!(combos.0.len(), 921);
        for combo in combos.0.iter() {
            println!("{:?}", combo);
        }
        assert!(combos.0.contains(
            (&PredatorCombo::new(
                Stance::Rizet,
                vec![
                    ComboAttack::Pinprick,
                    ComboAttack::Pheromones,
                    ComboAttack::Vertical,
                ]
            ))
        ));
        assert!(combos.0.contains(
            (&PredatorCombo::new(
                Stance::Rizet,
                vec![
                    ComboAttack::Raze,
                    ComboAttack::Gouge,
                    ComboAttack::Vertical,
                    ComboAttack::Trip,
                ]
            ))
        ));
    }

    #[test]
    fn find_veinrip_combo() {
        let attacks = vec![
            ComboAttack::Veinrip,
            ComboAttack::Vertical,
            ComboAttack::Crescentcut,
            ComboAttack::Jab,
            ComboAttack::Lowhook,
            ComboAttack::Mindnumb,
            ComboAttack::Pheromones,
            ComboAttack::Flashkick,
            ComboAttack::Pinprick,
            ComboAttack::Feint,
            ComboAttack::Raze,
        ];
        let mut solver = ComboSolver::new(Stance::EinFasit);
        solver.set_attacks(attacks).set_parry(true).set_rebounds(1);
        let combos = solver.find_combos();
        assert_eq!(combos.0.len(), 57);
        for combo in combos.0.iter() {
            println!("{:?}", combo);
        }
    }
}
