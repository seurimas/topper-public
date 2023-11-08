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
            ComboAttack::Flashkick => false,
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

    pub fn can_drop_parry(&self) -> bool {
        if self.can_use_venom() {
            true
        } else if self == &ComboAttack::Feint {
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
        if *self == ComboAttack::Raze {
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
        } else if stance == Stance::Laesan {
            base - 40
        } else {
            base
        }
    }

    pub fn get_next_stance(&self, stance: Stance) -> Stance {
        match (self, stance) {
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

    pub fn get_attacks_to_stance(
        &self,
        current_stance: Stance,
        target_stance: Stance,
    ) -> Vec<Self> {
        let mut attacks = vec![];
        for attack in ComboAttack::iter() {
            if attack.get_next_stance(current_stance) == target_stance {
                attacks.push(attack);
            }
        }
        attacks
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
}

fn add_combos<'a>(
    valid_attacks: &Vec<ComboAttack>,
    combos: &mut Vec<PredatorCombo>,
    starting_stance: Stance,
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
        combos.push(PredatorCombo::new(starting_stance, new_attacks.clone()));
    }
    if new_attacks.len() == 3 && next_stance != Stance::Laesan {
        return;
    } else if new_attacks.len() == 4 {
        return;
    }
    parrying |= attack.can_drop_parry();
    prone |= attack.can_prone();
    if attack.strips_rebounding() && rebounds > 0 {
        rebounds -= 1;
    }
    for next_attack in valid_attacks.iter() {
        if next_attack.is_good_combo_attack(next_stance)
            && !next_attack.must_begin_combo()
            && (!next_attack.idempotent() || !new_attacks.contains(&next_attack))
            && (!parrying || !next_attack.parryable())
            && (prone || !next_attack.requires_prone())
            && (rebounds == 0 || !next_attack.rebounds())
        {
            add_combos(
                valid_attacks,
                combos,
                starting_stance,
                next_stance,
                *next_attack,
                new_attacks.clone(),
                parrying,
                prone,
                rebounds,
            );
        }
    }
}

pub fn find_combos<'a>(
    stance: Stance,
    valid_attacks: &Vec<ComboAttack>,
    parrying: bool,
    prone: bool,
    rebounds: u32,
) -> Vec<PredatorCombo> {
    let mut combos = vec![];
    for attack in valid_attacks.iter() {
        if attack.is_good_combo_attack(stance)
            && (!parrying || !attack.parryable())
            && (prone || !attack.requires_prone())
            && (rebounds == 0 || !attack.rebounds())
        {
            add_combos(
                valid_attacks,
                &mut combos,
                stance,
                stance,
                *attack,
                vec![],
                parrying,
                prone,
                rebounds,
            );
        }
    }
    combos
}

mod predator_tests {
    use super::*;

    #[test]
    pub fn test_find_combos() {
        let combos = find_combos(
            Stance::Rizet,
            &vec![
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
            ],
            false,
            false,
            0,
        );
        assert_eq!(combos.len(), 610);
        for combo in combos.iter() {
            println!("{:?}", combo);
        }
        assert!(combos.contains(
            (&PredatorCombo::new(
                Stance::Rizet,
                vec![
                    ComboAttack::Pinprick,
                    ComboAttack::Pheromones,
                    ComboAttack::Vertical,
                ]
            ))
        ));
        assert!(combos.contains(
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
}
