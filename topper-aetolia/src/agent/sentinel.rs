use super::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

use crate::agent::general::Timer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SentinelBeast {
    Wisp,
    Weasel,
    Nightingale,
    Rook,
    Coyote,
    Raccoon,
    Elk,
    Gyrfalcon,
    Raloth,
    Crocodile,
    Icewyrm,
    Cockatrice,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SentinelClassState {
    pub alacrity: u32,
    pub spike: Option<String>,
    pub beasts: HashSet<SentinelBeast>,
    pub first_strike_timer: Timer,
    pub whirl_timer: Timer,
    pub flourish: bool,
    pub calling: Timer,
}

impl Hash for SentinelClassState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.alacrity.hash(state);
        self.spike.hash(state);
        // HashSet doesn't impl Hash; use sorted vec for determinism
        let mut beasts: Vec<_> = self.beasts.iter().copied().collect();
        beasts.sort_by_key(|b| *b as u8);
        beasts.hash(state);
        self.first_strike_timer.hash(state);
        self.whirl_timer.hash(state);
        self.calling.hash(state);
    }
}

impl SentinelClassState {
    pub fn has_beast(&self, beast: SentinelBeast) -> bool {
        self.beasts.contains(&beast)
    }

    pub fn summon_beast(&mut self, beast: SentinelBeast) {
        self.beasts.insert(beast);
    }

    pub fn dismiss_beast(&mut self, beast: SentinelBeast) {
        self.beasts.remove(&beast);
    }

    pub fn beast_count(&self) -> usize {
        self.beasts.len()
    }

    pub fn start_whirl(&mut self) {
        self.whirl_timer = Timer::count_down_seconds(1.6);
    }

    pub fn confirm_whirl(&mut self) {
        self.whirl_timer.reset();
    }

    pub fn whirl_coming(&self) -> bool {
        self.whirl_timer.is_active() && self.whirl_timer.get_time_left_seconds() < 0.25
    }

    pub fn start_first_strike(&mut self, flourish: bool) {
        self.first_strike_timer = Timer::count_down_seconds(1.2);
        self.flourish = flourish;
    }

    pub fn has_first_strike(&self, flourish: bool) -> bool {
        self.first_strike_timer.is_active() && self.flourish == flourish
    }

    pub fn second_strike(&mut self) {
        self.first_strike_timer.reset();
        self.flourish = false;
    }

    pub fn start_calling(&mut self) {
        self.calling = Timer::count_down_seconds(25.0);
    }

    pub fn is_calling(&self) -> bool {
        self.calling.is_active()
    }

    pub fn clear_calling(&mut self) {
        self.calling.reset();
    }

    pub fn wait(&mut self, duration: CType) {
        self.first_strike_timer.wait(duration);
        self.whirl_timer.wait(duration);
        self.calling.wait(duration);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resin {
    Pyrolum,
    Corsin,
    Trientia,
    Harimel,
    Glauxe,
    Badulem,
    Lysirine,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ResinState {
    pub hot: Option<Resin>,
    pub cold: Option<Resin>,
    pub burning: Timer,
    pub ticks_left: u8,
}

impl ResinState {
    pub fn wait(&mut self, duration: CType) {
        self.burning.wait(duration);
        if self.burning.get_time_left_seconds() > self.ticks_left as f32 * 6.0 {
            // Clear state.
            self.hot_burn();
        }
    }
    pub fn clear(&mut self) {
        self.burning.reset();
        self.ticks_left = 0;
        self.hot = None;
        self.cold = None;
    }
    pub fn apply(&mut self, layer: Resin) {
        self.hot = self.cold.clone();
        self.cold = Some(layer);
        self.burning.reset();
        self.ticks_left = 0;
    }
    pub fn ignite(&mut self) {
        self.ticks_left = match self.cold {
            Some(Resin::Pyrolum) => 12,
            Some(Resin::Corsin) => 8,
            Some(Resin::Trientia) => 9,
            Some(Resin::Harimel) => 14,
            Some(Resin::Glauxe) => 10,
            Some(Resin::Badulem) => 8,
            Some(Resin::Lysirine) => 6,
            None => 0,
        };
        self.reset_burning_timer_for_ticks_left();
    }

    fn reset_burning_timer_for_ticks_left(&mut self) {
        self.burning = Timer::count_up_observe_seconds(
            self.ticks_left as f32 * 6.0,
            self.ticks_left as f32 * 6.0 + 3.,
        );
    }

    pub fn cold_burn(&mut self) {
        println!(
            "Cold burn: hot={:?} cold={:?} burning_time={:?} ticks_left={}",
            self.hot, self.cold, self.burning, self.ticks_left
        );
        if (self.ticks_left > 0) {
            self.ticks_left -= 1;
        }
        if (self.ticks_left == 0) {
            self.cold = None;
        }
        self.reset_burning_timer_for_ticks_left();
    }
    pub fn hot_burn(&mut self) {
        println!(
            "Hot burn: hot={:?} cold={:?} burning_time={:?} ticks_left={}",
            self.hot, self.cold, self.burning, self.ticks_left
        );
        self.clear();
    }
}
