use super::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash};

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
}

impl Hash for SentinelClassState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.alacrity.hash(state);
        self.spike.hash(state);
        // HashSet doesn't impl Hash; use sorted vec for determinism
        let mut beasts: Vec<_> = self.beasts.iter().copied().collect();
        beasts.sort_by_key(|b| *b as u8);
        beasts.hash(state);
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub burning: bool,
    pub burning_time: CType,
    pub ticks_left: u8,
}

impl ResinState {
    pub fn wait(&mut self, duration: CType) {
        if self.burning {
            self.burning_time += duration;
        }
        if self.burning_time > 1500 {
            // Clear state.
            self.hot_burn();
        }
    }
    pub fn clear(&mut self) {
        self.burning = false;
        self.burning_time = 0;
        self.ticks_left = 0;
        self.hot = None;
        self.cold = None;
    }
    pub fn apply(&mut self, layer: Resin) {
        self.hot = self.cold.clone();
        self.cold = Some(layer);
        self.burning = false;
        self.burning_time = 0;
        self.ticks_left = 0;
    }
    pub fn ignite(&mut self) {
        self.burning = true;
        self.burning_time = 0;
        self.ticks_left = match self.cold {
            Some(Resin::Pyrolum) => 12,
            Some(Resin::Corsin) => 8,
            Some(Resin::Trientia) => 9,
            Some(Resin::Harimel) => 14,
            Some(Resin::Glauxe) => 10,
            Some(Resin::Badulem) => 8,
            Some(Resin::Lysirine) => 6,
            None => 0,
        }
    }
    pub fn cold_burn(&mut self) {
        self.burning_time = 0;
        if (self.ticks_left > 0) {
            self.ticks_left -= 1;
        }
        if (self.ticks_left == 0) {
            self.cold = None;
        }
    }
    pub fn hot_burn(&mut self) {
        self.clear();
    }
}
