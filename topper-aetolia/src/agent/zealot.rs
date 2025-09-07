use topper_core::timeline::BaseAgentState;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZenithState {
    Inactive,
    Rising(CType),
    Active(CType),
}

impl Default for ZenithState {
    fn default() -> Self {
        ZenithState::Inactive
    }
}

impl ZenithState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            ZenithState::Inactive => {}
            ZenithState::Rising(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Rising(remaining - duration);
                } else {
                    self.activate();
                }
            }
            ZenithState::Active(remaining) => {
                if remaining > duration {
                    *self = ZenithState::Active(remaining - duration);
                } else {
                    self.deactivate();
                }
            }
        }
    }
    pub fn initiate(&mut self) {
        *self = ZenithState::Rising((15.0 * BALANCE_SCALE) as CType);
    }
    pub fn activate(&mut self) {
        *self = ZenithState::Active((11.0 * BALANCE_SCALE) as CType);
    }
    pub fn deactivate(&mut self) {
        *self = ZenithState::Inactive;
    }
    pub fn can_initiate(&self) -> bool {
        match self {
            ZenithState::Inactive => true,
            _ => false,
        }
    }
    pub fn active(&self) -> bool {
        match self {
            ZenithState::Active(_) => true,
            _ => false,
        }
    }

    pub fn time_to_active(&self) -> Option<CType> {
        match self {
            ZenithState::Rising(remaining) => Some(*remaining),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ZealotClassState {
    pub zenith: ZenithState,
    pub pyromania: TimedFlagState,
}

pub fn get_pendulum_values(you: &AgentState, reverse: bool) -> Vec<CType> {
    let mut rotated = you.clone();
    rotated.limb_damage.first_person_restore = false;
    rotated.limb_damage.rotate(reverse);
    rotated.wait(BALANCE_SCALE as i32 * 10);
    let mut base = you.clone();
    base.limb_damage.first_person_restore = false;
    base.wait(BALANCE_SCALE as i32 * 10);
    let after_rotate_state = rotated.limb_damage;
    let after_base_state = base.limb_damage;
    vec![
        after_rotate_state.get_damage(LType::HeadDamage)
            - after_base_state.get_damage(LType::HeadDamage),
        after_rotate_state.get_damage(LType::TorsoDamage)
            - after_base_state.get_damage(LType::TorsoDamage),
        after_rotate_state.get_damage(LType::LeftArmDamage)
            - after_base_state.get_damage(LType::LeftArmDamage),
        after_rotate_state.get_damage(LType::RightArmDamage)
            - after_base_state.get_damage(LType::RightArmDamage),
        after_rotate_state.get_damage(LType::LeftLegDamage)
            - after_base_state.get_damage(LType::LeftLegDamage),
        after_rotate_state.get_damage(LType::RightLegDamage)
            - after_base_state.get_damage(LType::RightLegDamage),
    ]
}
