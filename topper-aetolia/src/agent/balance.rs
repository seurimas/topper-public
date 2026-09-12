use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use structdiff::{Difference, StructDiff};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive, Serialize)]
#[repr(usize)]
pub enum BType {
    // Actions
    Balance,
    Equil,
    Secondary,
    LeftHandBalance,
    RightHandBalance,

    // Curatives
    Elixir,
    Pill,
    Salve,
    Smoke,
    Focus,
    Tree,
    Regenerate,

    // Misc
    Fitness,
    ClassCure1,
    ClassCure2,

    // Cooldowns
    ClassAttack1,
    ClassAttack2,
    ClassAttack3,
    ClassAttack4,

    // Timers
    // Disabled,
    Hypnosis,
    Marked,
    Fangbarrier,
    Rebounding,
    Void,
    ParesisParalysis,
    SelfLoathing,
    // Manabarbs,
    Pacifism,
    Boar,
    Moon,

    // Writhe
    WritheDartpinned,
    WritheWeb,

    UNKNOWN,
    SIZE,

    Induce,
}

impl BType {
    pub fn from_name(bal_name: &String) -> Self {
        match bal_name.as_str() {
            "Balance" => BType::Balance,
            "Equilibrium" => BType::Equil,
            "Cosmic" | "Shadow" => BType::Secondary,
            "Left Hand Balance" => BType::LeftHandBalance,
            "Right Hand Balance" => BType::RightHandBalance,
            _ => BType::UNKNOWN,
        }
    }

    pub fn wrath() -> Self {
        BType::ClassAttack1
    }

    pub fn firefist() -> Self {
        BType::ClassAttack2
    }

    pub fn pendulum() -> Self {
        BType::ClassAttack3
    }

    pub fn disable() -> Self {
        BType::ClassAttack4
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Difference)]
pub struct BalanceSet {
    #[difference(collection_strategy = "unordered_map_like")]
    timers: HashMap<BType, Timer>,
}

impl Default for BalanceSet {
    fn default() -> Self {
        let timers = (0..BType::SIZE as usize)
            .filter_map(|i| BType::try_from(i).ok())
            .map(|balance| (balance, Timer::default()))
            .collect();
        BalanceSet { timers }
    }
}

// HashMap doesn't implement Hash, so hash a deterministically ordered view of it instead.
impl Hash for BalanceSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut entries = self.timers.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(balance, _)| **balance as usize);
        for (balance, timer) in entries {
            balance.hash(state);
            timer.hash(state);
        }
    }
}

const DEFAULT_TIMER: Timer = Timer::CountDown(0);

impl BalanceSet {
    pub fn wait(&mut self, duration: CType, cooldown_effect: bool) {
        for (balance, timer) in self.timers.iter_mut() {
            match (balance, cooldown_effect) {
                (
                    BType::Fitness | BType::ClassCure1 | BType::ClassCure2 | BType::Regenerate,
                    true,
                ) => {
                    // Aeon pauses cooldowns.
                }
                _ => timer.wait(duration),
            }
        }
    }
}

impl Index<BType> for BalanceSet {
    type Output = Timer;

    fn index(&self, balance: BType) -> &Timer {
        self.timers.get(&balance).unwrap_or(&DEFAULT_TIMER)
    }
}

impl IndexMut<BType> for BalanceSet {
    fn index_mut(&mut self, balance: BType) -> &mut Timer {
        self.timers.entry(balance).or_insert_with(Timer::default)
    }
}
