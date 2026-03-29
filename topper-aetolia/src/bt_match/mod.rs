//! Types and logic for bt_match — comparing a behavior tree against a combat log.

use std::{
    collections::HashSet,
    fmt, fs,
    str::FromStr,
    sync::{Arc, Mutex, OnceLock},
};

use behavior_bark::unpowered::UnpoweredFunction;
use topper_core::timeline::{BaseAgentState, BaseTimeline, db::DummyDatabaseModule};

use crate::{
    bt::{BehaviorController, BehaviorModel, LOAD_TREE_FUNC, get_tree},
    classes::AFFLICT_VENOMS,
    observables::ActionPlan,
    timeline::{AetObservation, AetTimeSlice, AetTimeline, CombatAction},
    types::{AgentState, BType, FType},
};
mod bt;
mod config;
mod display;
mod divergence;
mod runner;
pub use bt::set_bt_dir;
pub use config::BtMatchConfig;
pub use display::{format_time, print_agent_state};
pub use divergence::Divergence;
pub use runner::{BranchPlan, MatchRunner};
