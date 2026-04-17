use super::*;

pub const RECENT_UNPARRIED_WINDOW: f32 = 2.0;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ParryState {
    parrying: Option<LType>,
    parry_known: bool,
    last_hit: Option<LType>,
    last_hit_timer: Timer,
}

impl ParryState {
    pub fn wait(&mut self, duration: i32) {
        self.last_hit_timer.wait(duration);
        if !self.last_hit_timer.is_active() {
            self.last_hit = None;
        }
    }

    pub fn clear_parrying(&mut self) {
        self.parrying = None;
        self.last_hit_timer = Timer::count_down_seconds(RECENT_UNPARRIED_WINDOW);
    }

    pub fn get_parrying(&self) -> Option<LType> {
        self.parrying
    }

    pub fn set_parrying(&mut self, limb: LType) {
        self.parrying = Some(limb);
    }

    pub fn is_known(&self) -> bool {
        self.parry_known
    }

    pub fn clear_known(&mut self) {
        self.parry_known = false;
    }

    pub fn is_definitely_not_parrying(&self, limb: LType) -> bool {
        if !self.parry_known {
            return false;
        }
        match self.parrying {
            Some(parrying_limb) => parrying_limb != limb,
            None => true,
        }
    }

    pub fn observe_unparried(&mut self, limb: LType) {
        self.last_hit = Some(limb);
        self.last_hit_timer = Timer::count_down_seconds(RECENT_UNPARRIED_WINDOW);
    }

    pub fn is_recently_unparried(&self, limb: LType) -> bool {
        (self.last_hit.is_none() || self.last_hit == Some(limb)) && self.last_hit_timer.is_active()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_unparried_tracking_expires() {
        let mut parry = ParryState::default();

        assert!(!parry.is_recently_unparried(LType::LeftArmDamage));

        parry.observe_unparried(LType::LeftArmDamage);
        assert!(parry.is_recently_unparried(LType::LeftArmDamage));
        assert!(!parry.is_recently_unparried(LType::RightArmDamage));

        parry.wait((RECENT_UNPARRIED_WINDOW * BALANCE_SCALE) as i32 + 1);
        assert!(!parry.is_recently_unparried(LType::LeftArmDamage));
    }
}
