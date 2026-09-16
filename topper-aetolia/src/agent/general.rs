use crate::classes::Class;

use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ascii::AsciiExt;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use structdiff::{Difference, StructDiff};
use topper_core::timeline::BaseAgentState;
use topper_persuasion::PersuasionAff;

#[derive(Deserialize, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AreaStatus {
    #[default]
    OutOfArea,
    InArea,
    InAreaOutside,
    InAreaInside,
}

impl AreaStatus {
    pub fn is_in_area(&self) -> bool {
        matches!(
            self,
            AreaStatus::InArea | AreaStatus::InAreaOutside | AreaStatus::InAreaInside
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ObservationState {
    pub time_since_seen: CType,
    pub area_status: AreaStatus,
    pub time_since_area_status: CType,
}

impl ObservationState {
    pub fn wait(&mut self, duration: CType) {
        self.time_since_seen = self.time_since_seen.saturating_add(duration);
        self.time_since_area_status = self.time_since_area_status.saturating_add(duration);
    }

    pub fn observe_seen(&mut self) {
        self.time_since_seen = 0;
    }

    pub fn observe_area_status(&mut self, status: AreaStatus) {
        if self.area_status != status {
            self.area_status = status;
            self.time_since_area_status = 0;
        }
    }
}

// Balances
// BType and BalanceSet moved to balance.rs (re-exported via agent::mod).

// Stats
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum SType {
    Health,
    Mana,
    SP,

    SIZE,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Difference)]
pub enum WieldState {
    Normal {
        left: Option<String>,
        right: Option<String>,
    },
    TwoHanded(String),
}

impl WieldState {
    pub fn is_wielding(&self, substring: &str) -> bool {
        self.is_wielding_left(substring) || self.is_wielding_right(substring)
    }

    pub fn is_wielding_left(&self, substring: &str) -> bool {
        match self {
            Self::Normal { left, right } => left
                .as_ref()
                .map(|left| left.find(substring).is_some())
                .unwrap_or(false),
            Self::TwoHanded(both) => both.find(substring).is_some(),
        }
    }

    pub fn is_wielding_right(&self, substring: &str) -> bool {
        match self {
            Self::Normal { left, right } => right
                .as_ref()
                .map(|right| right.find(substring).is_some())
                .unwrap_or(false),
            Self::TwoHanded(both) => both.find(substring).is_some(),
        }
    }

    pub fn empty_hand(&self) -> bool {
        self.get_left().is_none() || self.get_right().is_none()
    }

    pub fn hands_empty(&self, left: bool, right: bool) -> bool {
        if left && self.get_left().is_some() {
            false
        } else if right && self.get_right().is_some() {
            false
        } else {
            true
        }
    }

    pub fn get_left(&self) -> Option<String> {
        match self {
            Self::Normal { left, .. } => left.clone(),
            Self::TwoHanded(left) => Some(left.clone()),
        }
    }

    pub fn get_right(&self) -> Option<String> {
        match self {
            Self::Normal { right, .. } => right.clone(),
            Self::TwoHanded(right) => Some(right.clone()),
        }
    }

    pub fn weave(&mut self, weaved_item: &str) {
        let left_hand = self.get_left().is_none();
        *self = match self {
            Self::Normal {
                left: old_left,
                right: old_right,
            } => Self::Normal {
                left: if left_hand {
                    Some(weaved_item.to_string())
                } else {
                    old_left.clone()
                },
                right: if !left_hand {
                    Some(weaved_item.to_string())
                } else {
                    old_right.clone()
                },
            },
            Self::TwoHanded(item) => Self::Normal {
                left: Some(weaved_item.to_string()),
                right: None,
            },
        };
    }

    pub fn unweave(&mut self, predicate: impl Fn(&String) -> bool) {
        *self = match self {
            WieldState::Normal {
                left: old_left,
                right: old_right,
            } => {
                if old_left.as_ref().map(&predicate).unwrap_or_default() {
                    WieldState::Normal {
                        left: None,
                        right: old_right.clone(),
                    }
                } else if old_right.as_ref().map(&predicate).unwrap_or_default() {
                    WieldState::Normal {
                        left: old_left.clone(),
                        right: None,
                    }
                } else {
                    WieldState::Normal {
                        left: old_left.clone(),
                        right: old_right.clone(),
                    }
                }
            }
            WieldState::TwoHanded(item) => {
                if predicate(item) {
                    WieldState::Normal {
                        left: None,
                        right: None,
                    }
                } else {
                    WieldState::TwoHanded(item.clone())
                }
            }
        };
    }
}

impl Default for WieldState {
    fn default() -> Self {
        WieldState::Normal {
            left: None,
            right: None,
        }
    }
}

const SOFT_COOLDOWN: f32 = 2.0;
const HARD_COOLDOWN: f32 = 6.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DodgeTimer {
    Ready,
    Cooldown(CType),
}

impl Default for DodgeTimer {
    fn default() -> Self {
        DodgeTimer::Ready
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DodgeType {
    Unknown,
    Melee,
    Ranged,
    Charge,
    Upset,
}

impl Default for DodgeType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Difference)]
pub struct DodgeState {
    pub dodge_type: DodgeType,
    dodge_timer: DodgeTimer,
}

impl DodgeState {
    pub fn wait(&mut self, duration: CType) {
        match self.dodge_timer {
            DodgeTimer::Ready => {}
            DodgeTimer::Cooldown(remaining) => {
                if remaining > duration {
                    self.dodge_timer = DodgeTimer::Cooldown(remaining - duration);
                } else {
                    self.dodge_timer = DodgeTimer::Ready;
                }
            }
        }
    }
    pub fn register_hit(&mut self) {
        match self.dodge_timer {
            DodgeTimer::Ready => {
                self.dodge_timer = DodgeTimer::Cooldown((SOFT_COOLDOWN * BALANCE_SCALE) as CType);
            }
            DodgeTimer::Cooldown(_) => {}
        }
    }
    pub fn register_dodge(&mut self) {
        self.dodge_timer = DodgeTimer::Cooldown((HARD_COOLDOWN * BALANCE_SCALE) as CType);
    }
    pub fn can_dodge(&self) -> bool {
        match self.dodge_timer {
            DodgeTimer::Ready => true,
            _ => false,
        }
    }
    pub fn can_dodge_at(&self, qeb: f32) -> bool {
        match self.dodge_timer {
            DodgeTimer::Ready => true,
            DodgeTimer::Cooldown(cooldown) => {
                if cooldown < ((qeb * BALANCE_SCALE) as CType) {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Difference)]
pub enum ClassState {
    Ascendril(AscendrilClassState),
    Bloodborn(AscendrilClassState), // Mirror of Ascendril
    Zealot(ZealotClassState),
    Ravager(ZealotClassState), // Mirror of Zealot
    Sentinel(SentinelClassState),
    Executor(SentinelClassState), // Mirror of Sentinel
    Predator(PredatorClassState),
    Monk(MonkClassState),
    Bard(BardClassState),
    Infiltrator(InfiltratorClassState),
    Siderealist(SiderealistClassState),
    Shifter(HowlingState),
    Other(Class),
    Unknown,
}

impl ClassState {
    pub fn wait(&mut self, duration: CType, cooldown_effect: CooldownEffect) {
        match self {
            ClassState::Zealot(ZealotClassState { zenith, pyromania })
            | ClassState::Ravager(ZealotClassState { zenith, pyromania }) => {
                if !cooldown_effect {
                    zenith.wait(duration);
                }
                pyromania.wait(duration);
            }
            ClassState::Bard(bard_class_state) => bard_class_state.wait(duration, cooldown_effect),
            ClassState::Predator(predator_class_state) => {
                predator_class_state.wait(duration, cooldown_effect)
            }
            ClassState::Sentinel(sentinel_class_state)
            | ClassState::Executor(sentinel_class_state) => sentinel_class_state.wait(duration),
            ClassState::Infiltrator(infiltrator_class_state) => {
                // infiltrator_class_state.wait(duration)
            }
            ClassState::Siderealist(siderealist_class_state) => {
                siderealist_class_state.wait(duration, cooldown_effect)
            }
            // ClassState::Shifter(howling_state) => howling_state.wait(duration),
            // ClassState::Monk(monk_class_state) => monk_class_state.wait(duration),
            ClassState::Ascendril(ascendril_class_state)
            | ClassState::Bloodborn(ascendril_class_state) => {
                ascendril_class_state.wait(duration, cooldown_effect)
            }
            _ => {}
        }
    }

    pub fn get_normalized_class(&self) -> Option<Class> {
        match self {
            Self::Bloodborn(_) | Self::Ascendril(_) => Some(Class::Ascendril),
            Self::Ravager(_) | Self::Zealot(_) => Some(Class::Zealot),
            Self::Predator(_) => Some(Class::Predator),
            Self::Executor(_) | Self::Sentinel(_) => Some(Class::Sentinel),
            Self::Bard(_) => Some(Class::Bard),
            Self::Infiltrator(_) => Some(Class::Infiltrator),
            Self::Siderealist(_) => Some(Class::Siderealist),
            Self::Shifter(_) => Some(Class::Shapeshifter),
            Self::Monk(_) => Some(Class::Monk),
            Self::Other(class) => Some(class.normal()),
            Self::Unknown => None,
        }
    }

    pub fn initialize_for_class(&mut self, class: Class) {
        if let Some(new_class_state) = match (&self, class) {
            // Already initialized,
            (Self::Ascendril(_), Class::Ascendril) => None,
            (Self::Bloodborn(_), Class::Bloodborn) => None,
            (Self::Ascendril(state), Class::Bloodborn) => Some(Self::Bloodborn(state.clone())),
            (Self::Bloodborn(state), Class::Ascendril) => Some(Self::Ascendril(state.clone())),
            (Self::Zealot(_), Class::Zealot) => None,
            (Self::Ravager(_), Class::Ravager) => None,
            (Self::Zealot(state), Class::Ravager) => Some(Self::Ravager(state.clone())),
            (Self::Ravager(state), Class::Zealot) => Some(Self::Zealot(state.clone())),
            (Self::Sentinel(_), Class::Sentinel) => None,
            (Self::Executor(_), Class::Executor) => None,
            (Self::Sentinel(state), Class::Executor) => Some(Self::Executor(state.clone())),
            (Self::Executor(state), Class::Sentinel) => Some(Self::Sentinel(state.clone())),
            (Self::Bard(_), Class::Bard) => None,
            (Self::Shifter(_), Class::Shapeshifter) => None,
            (Self::Predator(_), Class::Predator) => None,
            (Self::Siderealist(_), Class::Siderealist) => None,
            (Self::Infiltrator(_), Class::Infiltrator) => None,
            (Self::Monk(_), Class::Monk) => None,
            // Changed.
            (_, Class::Ascendril) => Some(Self::Ascendril(AscendrilClassState::default())),
            (_, Class::Bloodborn) => Some(Self::Bloodborn(AscendrilClassState::default())),
            (_, Class::Zealot) => Some(Self::Zealot(ZealotClassState::default())),
            (_, Class::Ravager) => Some(Self::Ravager(ZealotClassState::default())),
            (_, Class::Sentinel) => Some(Self::Sentinel(SentinelClassState::default())),
            (_, Class::Executor) => Some(Self::Executor(SentinelClassState::default())),
            (_, Class::Bard) => Some(Self::Bard(BardClassState::default())),
            (_, Class::Shapeshifter) => Some(Self::Shifter(HowlingState::default())),
            (_, Class::Predator) => Some(Self::Predator(PredatorClassState::default())),
            (_, Class::Siderealist) => Some(Self::Siderealist(SiderealistClassState::default())),
            (_, Class::Infiltrator) => Some(Self::Infiltrator(InfiltratorClassState::default())),
            (_, Class::Monk) => Some(Self::Monk(MonkClassState::default())),
            (_, observed) => {
                // Other initializations should have been covered above.
                Some(Self::Other(observed))
            }
        } {
            *self = new_class_state;
        }
    }

    pub fn is_mirrored(&self) -> bool {
        match self {
            ClassState::Bloodborn(_) | ClassState::Ravager(_) | ClassState::Executor(_) => true,
            _ => false,
        }
    }
}

impl Default for ClassState {
    fn default() -> ClassState {
        ClassState::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    Heelrush,
    Zeal, // Battlefury
    Direblow,
    Shatter,
    Death,
    Moonlet,
    Tumbling,
    Disintegrate,
    Winterheart,
    Stormwrath,
    Enrapture,
}

impl ChannelType {
    pub fn stopped_by(&self, aff: FType) -> bool {
        match (self, aff) {
            (_, FType::Asleep) => true,
            (ChannelType::Heelrush, FType::Paresis) => true,
            (ChannelType::Direblow, FType::Paresis) => true,
            (ChannelType::Disintegrate, FType::Paresis) => true,
            (ChannelType::Winterheart, FType::Paresis) => true,
            (ChannelType::Stormwrath, FType::Paresis) => true,
            (ChannelType::Moonlet, FType::LeftArmCrippled) => true,
            (ChannelType::Moonlet, FType::RightArmCrippled) => true,
            (ChannelType::Moonlet, FType::Paresis) => true,
            (ChannelType::Tumbling, FType::WritheImpaled) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Difference)]
pub enum ChannelState {
    Inactive,
    ChannelWithLimb {
        channel_type: ChannelType,
        timer: Timer,
        limb: LType,
    },
    Channeling {
        channel_type: ChannelType,
        timer: Timer,
    },
    ChannelWithTarget {
        channel_type: ChannelType,
        target: String,
        timer: Timer,
    },
}

impl ChannelState {
    pub fn wait(&mut self, duration: CType) {
        match self {
            ChannelState::ChannelWithLimb { timer, .. } => {
                timer.wait(duration);
                if !timer.is_active() {
                    *self = ChannelState::Inactive;
                }
            }
            ChannelState::Channeling { timer, .. } => {
                timer.wait(duration);
                if !timer.is_active() {
                    *self = ChannelState::Inactive;
                }
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        *self = ChannelState::Inactive;
    }

    pub fn channel_with_limb(&mut self, channel_type: ChannelType, limb: LType, duration: CType) {
        *self = ChannelState::ChannelWithLimb {
            channel_type,
            limb,
            timer: Timer::count_up_observe(duration, duration),
        };
    }

    pub fn channel_with_limb_seconds(
        &mut self,
        channel_type: ChannelType,
        limb: LType,
        duration: f32,
    ) {
        *self = ChannelState::ChannelWithLimb {
            channel_type,
            limb,
            timer: Timer::count_up_seconds(duration),
        };
    }

    pub fn channel(&mut self, channel_type: ChannelType, duration: CType) {
        *self = ChannelState::Channeling {
            channel_type,
            timer: Timer::count_up_observe(duration, duration),
        };
    }

    pub fn channel_seconds(&mut self, channel_type: ChannelType, duration: f32) {
        *self = ChannelState::Channeling {
            channel_type,
            timer: Timer::count_up_seconds(duration),
        };
    }

    pub fn channel_with_target(
        &mut self,
        channel_type: ChannelType,
        duration: CType,
        target: String,
    ) {
        *self = ChannelState::ChannelWithTarget {
            channel_type,
            target,
            timer: Timer::count_up_observe(duration, duration),
        };
    }

    pub fn channel_with_target_seconds(
        &mut self,
        channel_type: ChannelType,
        duration: f32,
        target: String,
    ) {
        *self = ChannelState::ChannelWithTarget {
            channel_type,
            target,
            timer: Timer::count_up_seconds(duration),
        };
    }

    pub fn get_channel_type(&self) -> Option<ChannelType> {
        match self {
            ChannelState::ChannelWithLimb { channel_type, .. } => Some(*channel_type),
            ChannelState::Channeling { channel_type, .. } => Some(*channel_type),
            _ => None,
        }
    }

    pub fn is_channeling(&self, channel: ChannelType) -> bool {
        match self {
            ChannelState::ChannelWithLimb { channel_type, .. } => channel == *channel_type,
            ChannelState::Channeling { channel_type, .. } => channel == *channel_type,
            _ => false,
        }
    }

    pub fn is_channeling_any(&self) -> bool {
        match self {
            ChannelState::ChannelWithLimb { .. } => true,
            ChannelState::Channeling { .. } => true,
            _ => false,
        }
    }
}

impl Default for ChannelState {
    fn default() -> ChannelState {
        ChannelState::Inactive
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimedFlagState {
    Inactive,
    Active(CType),
}

impl Default for TimedFlagState {
    fn default() -> Self {
        TimedFlagState::Inactive
    }
}

impl TimedFlagState {
    pub fn wait(&mut self, duration: CType) {
        match self.clone() {
            TimedFlagState::Inactive => {}
            TimedFlagState::Active(remaining) => {
                if remaining > duration {
                    *self = TimedFlagState::Active(remaining - duration);
                } else {
                    *self = TimedFlagState::Inactive;
                }
            }
        }
    }

    pub fn active(&self) -> bool {
        match self {
            TimedFlagState::Inactive => false,
            _ => true,
        }
    }

    pub fn activate(&mut self, duration: CType) {
        *self = TimedFlagState::Active(duration);
    }

    pub fn deactivate(&mut self) {
        *self = TimedFlagState::Inactive;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Elevation {
    Ground,
    Flying,
    Trees,
    Roof,
}

impl Default for Elevation {
    fn default() -> Self {
        Elevation::Ground
    }
}
