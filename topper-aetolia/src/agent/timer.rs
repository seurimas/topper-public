use super::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use structdiff::{Difference, StructDiff};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Timer {
    CountDown(CType),
    CountUpObserve {
        expire_at: CType,
        up_to: CType,
        progress: CType,
    },
}

impl Default for Timer {
    fn default() -> Self {
        Timer::CountDown(0)
    }
}

impl Timer {
    pub fn count_down(time: CType) -> Self {
        Timer::CountDown(time)
    }

    pub fn count_down_seconds(time: f32) -> Self {
        Timer::CountDown((time * BALANCE_SCALE) as CType)
    }

    pub fn count_up(up_to: CType) -> Self {
        Timer::CountUpObserve {
            up_to,
            expire_at: up_to,
            progress: 0,
        }
    }

    pub fn count_up_seconds(up_to: f32) -> Self {
        Timer::CountUpObserve {
            up_to: (up_to * BALANCE_SCALE) as CType,
            expire_at: (up_to * BALANCE_SCALE) as CType,
            progress: 0,
        }
    }

    pub fn count_up_seconds_off(up_to: f32) -> Self {
        Timer::CountUpObserve {
            up_to: (up_to * BALANCE_SCALE) as CType,
            expire_at: (up_to * BALANCE_SCALE) as CType,
            progress: (up_to * BALANCE_SCALE) as CType,
        }
    }

    pub fn count_up_observe(up_to: CType, expire_at: CType) -> Self {
        Timer::CountUpObserve {
            up_to,
            expire_at,
            progress: 0,
        }
    }

    pub fn count_up_observe_seconds(up_to: f32, expire_at: f32) -> Self {
        Timer::CountUpObserve {
            up_to: (up_to * BALANCE_SCALE) as CType,
            expire_at: (expire_at * BALANCE_SCALE) as CType,
            progress: 0,
        }
    }

    pub fn start_count_down(&mut self, time: CType) {
        *self = Timer::CountDown(time);
    }

    pub fn start_count_down_seconds(&mut self, time: f32) {
        *self = Timer::CountDown((time * BALANCE_SCALE) as CType);
    }

    pub fn reset(&mut self) {
        match self {
            Timer::CountDown(_) => *self = Timer::CountDown(0),
            Timer::CountUpObserve { progress, .. } => *progress = 0,
        }
    }

    pub fn expire(&mut self) {
        match self {
            Timer::CountDown(_) => *self = Timer::CountDown(0),
            Timer::CountUpObserve {
                expire_at,
                progress,
                ..
            } => *progress = *expire_at + 1,
        }
    }

    pub fn wait(&mut self, duration: CType) {
        match self {
            Timer::CountDown(remaining) => {
                if *remaining > duration {
                    *remaining -= duration;
                } else {
                    *self = Timer::CountDown(0);
                }
            }
            Timer::CountUpObserve {
                expire_at,
                progress,
                ..
            } => {
                if *progress <= *expire_at {
                    *progress += duration;
                }
            }
        }
    }

    pub fn is_active(&self) -> bool {
        match self {
            Timer::CountDown(remaining) => *remaining > 0,
            Timer::CountUpObserve {
                expire_at,
                progress,
                ..
            } => *progress <= *expire_at,
        }
    }

    pub fn get_time_left(&self) -> CType {
        match self {
            Timer::CountDown(remaining) => *remaining,
            Timer::CountUpObserve {
                up_to, progress, ..
            } => *up_to - *progress,
        }
    }

    pub fn get_time_left_seconds(&self) -> f32 {
        self.get_time_left() as f32 / BALANCE_SCALE as f32
    }

    pub fn abs_diff(&self, target_time: CType) -> CType {
        self.get_time_left().abs_diff(target_time) as CType
    }
}
