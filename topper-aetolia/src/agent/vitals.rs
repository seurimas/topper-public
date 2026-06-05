use super::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Tracks the state of a single vital stat.
///
/// When exact values are known (e.g., from GMCP) the `KnownAt` variant is used.
/// When only an observed percentage is available (e.g., `Assess` on an enemy) the
/// `Estimated` variant is used. All existing [`AgentState`] accessor methods remain
/// compatible: for `Estimated`, [`get_current`] maps `current_percent` to a 0–100
/// integer and [`get_max`] returns 100.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VitalState {
    /// Exact values confirmed at a specific timeline timestamp.
    KnownAt {
        current: CType,
        max: CType,
        /// The [`TimelineState::time`] value when these values were last observed.
        last_check: CType,
    },
    Estimated {
        current_percent: CType,
        max: CType,
        last_check: CType,
    },
}

impl Hash for VitalState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            VitalState::KnownAt {
                current,
                max,
                last_check,
            } => {
                0u8.hash(state);
                current.hash(state);
                max.hash(state);
                last_check.hash(state);
            }
            VitalState::Estimated {
                current_percent,
                max,
                last_check,
            } => {
                1u8.hash(state);
                current_percent.hash(state);
                max.hash(state);
                last_check.hash(state);
            }
        }
    }
}

/// Defaults to fully-healthy (100 %) estimated – used for unknown agents.
impl Default for VitalState {
    fn default() -> Self {
        VitalState::Estimated {
            current_percent: 100,
            max: 5000,
            last_check: 0,
        }
    }
}

impl VitalState {
    // ── Getters ──────────────────────────────────────────────────────────────

    /// Returns the current value as an integer.
    ///
    /// * `KnownAt` → exact `current`.
    /// * `Estimated` → `current_percent * 100` cast to `CType`, giving a 0–100
    ///   pseudo-integer that keeps all existing integer callers working.
    pub fn get_current(&self) -> CType {
        match self {
            VitalState::KnownAt { current, .. } => *current,
            VitalState::Estimated {
                current_percent,
                max,
                ..
            } => (*current_percent * *max / 100) as CType,
        }
    }

    /// Returns the max value.
    ///
    /// * `KnownAt` → exact `max`.
    /// * `Estimated` → `max` (matching the pseudo-integer scale of [`get_current`]).
    pub fn get_max(&self) -> CType {
        match self {
            VitalState::KnownAt { max, .. } => *max,
            VitalState::Estimated { max, .. } => *max,
        }
    }

    /// Returns the percentage in `[0, 100]`.
    pub fn get_percent(&self) -> CType {
        match self {
            VitalState::KnownAt { current, max, .. } => {
                if *max == 0 {
                    0
                } else {
                    *current * 100 / *max
                }
            }
            VitalState::Estimated {
                current_percent, ..
            } => *current_percent,
        }
    }

    // ── Setters ──────────────────────────────────────────────────────────────

    /// Updates only the current value (no clamping; mirrors existing `set_stat` semantics).
    ///
    /// If the state is `Estimated`, transitions to `KnownAt { current: value, max: 100,
    /// last_check: 0 }`.
    pub fn set_current(&mut self, value: CType) {
        match self {
            VitalState::KnownAt { current, .. } => *current = value,
            VitalState::Estimated { .. } => {
                *self = VitalState::KnownAt {
                    current: value,
                    max: 100,
                    last_check: 0,
                };
            }
        }
    }

    /// Updates only the current value by scaling `percent` against the existing max.
    /// KnownAt calculates `current = percent * max / 100`.
    pub fn set_current_percent(&mut self, percent: CType) {
        if let VitalState::Estimated {
            current_percent, ..
        } = self
        {
            *current_percent = percent;
        } else if let VitalState::KnownAt { max, .. } = self {
            let current = (percent * *max / 100) as CType;
            *self = VitalState::KnownAt {
                current,
                max: *max,
                last_check: 0,
            };
        }
    }

    /// Updates only the max value.
    ///
    /// If the state is `Estimated`, transitions to `KnownAt` with a current value
    /// derived by scaling the stored percentage against the new max.
    pub fn set_max(&mut self, value: CType) {
        match self {
            VitalState::KnownAt { max, .. } | VitalState::Estimated { max, .. } => *max = value,
        }
    }

    /// Fully overwrites with a known reading including timeline timestamp.
    pub fn set_known(&mut self, current: CType, max: CType, last_check: CType) {
        *self = VitalState::KnownAt {
            current,
            max,
            last_check,
        };
    }

    // ── Arithmetic helpers ───────────────────────────────────────────────────

    /// Increases current by `amount`, clamping at max.  No-op for `Estimated`.
    pub fn restore(&mut self, amount: CType) {
        if let VitalState::KnownAt { current, max, .. } = self {
            *current += amount;
            if *current > *max {
                *current = *max;
            }
        } else if let VitalState::Estimated {
            current_percent,
            max,
            ..
        } = self
        {
            let current = (*current_percent * *max / 100) as CType + amount;
            let new_percent = (current * 100 / *max) as CType;
            *current_percent = new_percent.min(100);
        }
    }

    pub fn restore_percent(&mut self, percent: CType) {
        if let VitalState::KnownAt { current, max, .. } = self {
            let amount = (percent * *max / 100) as CType;
            *current += amount;
            if *current > *max {
                *current = *max;
            }
        } else if let VitalState::Estimated {
            current_percent,
            max,
            ..
        } = self
        {
            *current_percent += percent;
            if *current_percent > 100 {
                *current_percent = 100;
            }
        }
    }

    /// Decreases current by `amount`, clamping at 0.  No-op for `Estimated`.
    pub fn damage(&mut self, amount: CType) {
        if let VitalState::KnownAt { current, .. } = self {
            *current -= amount;
            if *current < 0 {
                *current = 0;
            }
        } else if let VitalState::Estimated {
            current_percent,
            max,
            ..
        } = self
        {
            let current = (*current_percent * *max / 100) as CType - amount;
            let new_percent = (current * 100 / *max) as CType;
            *current_percent = new_percent.max(0);
        }
    }

    pub fn damage_percent(&mut self, percent: CType) {
        if let VitalState::KnownAt { current, max, .. } = self {
            let amount = (percent * *max / 100) as CType;
            *current -= amount;
            if *current < 0 {
                *current = 0;
            }
        } else if let VitalState::Estimated {
            current_percent, ..
        } = self
        {
            *current_percent -= percent;
            if *current_percent < 0 {
                *current_percent = 0;
            }
        }
    }
}

// ── VitalsState ───────────────────────────────────────────────────────────────

/// All vital stats for a single agent, stored as one [`VitalState`] per [`SType`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VitalsState {
    pub vitals: [VitalState; SType::SIZE as usize],
}

impl VitalsState {
    pub fn is_estimated(&self, stat: SType) -> bool {
        matches!(self.vitals[stat as usize], VitalState::Estimated { .. })
    }

    pub fn is_estimate_good(&self, stat: SType, current_time: CType) -> bool {
        if let VitalState::Estimated { last_check, .. } = self.vitals[stat as usize] {
            // If the last check was within the last 10 seconds, consider the estimate good.
            current_time - last_check <= (10 * BALANCE_SCALE as CType)
        } else {
            false
        }
    }

    // ── Forwarding helpers ───────────────────────────────────────────────────

    pub fn get_current(&self, stat: SType) -> CType {
        self.vitals[stat as usize].get_current()
    }

    pub fn get_max(&self, stat: SType) -> CType {
        self.vitals[stat as usize].get_max()
    }

    pub fn get_percent(&self, stat: SType) -> CType {
        self.vitals[stat as usize].get_percent()
    }

    pub fn set_current(&mut self, stat: SType, value: CType) {
        self.vitals[stat as usize].set_current(value);
    }

    pub fn set_current_percent(&mut self, stat: SType, percent: CType) {
        self.vitals[stat as usize].set_current_percent(percent);
    }

    pub fn set_max(&mut self, stat: SType, value: CType) {
        self.vitals[stat as usize].set_max(value);
    }

    pub fn set_seen(&mut self, stat: SType, current: CType, last_check: CType) {
        if let VitalState::Estimated { max, .. } = self.vitals[stat as usize] {
            self.vitals[stat as usize] = VitalState::Estimated {
                current_percent: if max > 0 {
                    (current * 100 / max) as CType
                } else {
                    0
                },
                max: max.max(current), // Update max if the observed current exceeds it
                last_check,
            };
        } else {
            self.vitals[stat as usize] = VitalState::KnownAt {
                current,
                // For KnownAt, we trust the observed current and max as they are.
                max: self.vitals[stat as usize].get_max(),
                last_check,
            };
        }
    }

    /// Sets a fully-known reading with the current timeline timestamp.
    pub fn set_known(&mut self, stat: SType, current: CType, max: CType, last_check: CType) {
        self.vitals[stat as usize].set_known(current, max, last_check);
    }

    pub fn restore(&mut self, stat: SType, amount: CType) {
        self.vitals[stat as usize].restore(amount);
    }

    pub fn restore_percent(&mut self, stat: SType, percent: CType) {
        self.vitals[stat as usize].restore_percent(percent);
    }

    pub fn damage(&mut self, stat: SType, amount: CType) {
        self.vitals[stat as usize].damage(amount);
    }

    pub fn damage_percent(&mut self, stat: SType, percent: CType) {
        self.vitals[stat as usize].damage_percent(percent);
    }

    /// Sets both current and max to `value` and stamps `last_check = 0`.
    /// Used during state initialisation.
    pub fn initialize(&mut self, stat: SType, value: CType) {
        self.vitals[stat as usize] = VitalState::Estimated {
            current_percent: 100,
            max: value,
            last_check: 0,
        };
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_at_getters() {
        let v = VitalState::KnownAt {
            current: 750,
            max: 1000,
            last_check: 42,
        };
        assert_eq!(v.get_current(), 750);
        assert_eq!(v.get_max(), 1000);
        assert_eq!(v.get_percent(), 75);
    }

    #[test]
    fn estimated_getters() {
        // 75 % of a 5000-max vital → current = 3750
        let v = VitalState::Estimated {
            current_percent: 75,
            max: 5000,
            last_check: 0,
        };
        assert_eq!(v.get_current(), 3750);
        assert_eq!(v.get_max(), 5000);
        assert_eq!(v.get_percent(), 75);
    }

    #[test]
    fn estimated_full_health_default() {
        // Default: 100 % of max 5000 = 5000 current
        let v = VitalState::default();
        assert_eq!(v.get_current(), 5000);
        assert_eq!(v.get_max(), 5000);
        assert_eq!(v.get_percent(), 100);
    }

    #[test]
    fn set_current_transitions_estimated_to_known() {
        let mut v = VitalState::Estimated {
            current_percent: 100,
            max: 5000,
            last_check: 0,
        };
        v.set_current(850);
        assert_eq!(v.get_current(), 850);
        assert_eq!(v.get_max(), 100);
        assert!(matches!(v, VitalState::KnownAt { .. }));
    }

    #[test]
    fn set_max_updates_estimated_in_place() {
        // set_max updates the stored max without transitioning variant;
        // get_current scales accordingly.
        let mut v = VitalState::Estimated {
            current_percent: 50,
            max: 5000,
            last_check: 0,
        };
        v.set_max(2000);
        assert_eq!(v.get_max(), 2000);
        assert_eq!(v.get_current(), 1000); // 50 % of 2000
        assert!(matches!(v, VitalState::Estimated { .. }));
    }

    #[test]
    fn restore_clamps_at_max() {
        let mut v = VitalState::KnownAt {
            current: 990,
            max: 1000,
            last_check: 0,
        };
        v.restore(20);
        assert_eq!(v.get_current(), 1000);
    }

    #[test]
    fn damage_clamps_at_zero() {
        let mut v = VitalState::KnownAt {
            current: 5,
            max: 1000,
            last_check: 0,
        };
        v.damage(10);
        assert_eq!(v.get_current(), 0);
    }

    #[test]
    fn restore_works_for_estimated() {
        let mut v = VitalState::Estimated {
            current_percent: 50,
            max: 1000,
            last_check: 0,
        };
        // current = 500, restore 200 → current = 700 → percent = 70
        v.restore(200);
        assert!(matches!(v, VitalState::Estimated { .. }));
        assert_eq!(v.get_current(), 700);
        assert_eq!(v.get_percent(), 70);
    }

    #[test]
    fn eq_and_hash_consistent_for_estimated() {
        let a = VitalState::Estimated {
            current_percent: 50,
            max: 5000,
            last_check: 0,
        };
        let b = VitalState::Estimated {
            current_percent: 50,
            max: 5000,
            last_check: 0,
        };
        assert_eq!(a, b);
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        let mut ha = DefaultHasher::new();
        let mut hb = DefaultHasher::new();
        a.hash(&mut ha);
        b.hash(&mut hb);
        assert_eq!(ha.finish(), hb.finish());
    }

    #[test]
    fn set_known_stamps_last_check() {
        let mut v = VitalState::default();
        v.set_known(800, 1000, 12345);
        assert_eq!(
            v,
            VitalState::KnownAt {
                current: 800,
                max: 1000,
                last_check: 12345,
            }
        );
    }

    #[test]
    fn vitals_state_initialize() {
        let mut vs = VitalsState::default();
        vs.initialize(SType::Health, 100);
        assert_eq!(vs.get_current(SType::Health), 100);
        assert_eq!(vs.get_max(SType::Health), 100);
        assert_eq!(vs.get_percent(SType::Health), 100);
    }

    #[test]
    fn vitals_state_default_is_full_estimated() {
        let vs = VitalsState::default();
        for i in 0..(SType::SIZE as usize) {
            assert!(matches!(
                vs.vitals[i],
                VitalState::Estimated {
                    current_percent: 100,
                    max: 5000,
                    last_check: 0,
                }
            ));
            // 100 % of max 5000 = 5000
            assert_eq!(vs.vitals[i].get_current(), 5000);
        }
    }
}
