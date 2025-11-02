use std::cmp::Ordering;

use topper_aetolia::timeline::{AetTimeSlice, AetTimeline, AetTimelineTrait, BaseTimeline};
use topper_core::timeline::db::DummyDatabaseModule;

mod loader;
pub mod observations;
mod parser;

pub use loader::*;
pub use parser::parse_prompt_time;
pub use parser::{AetoliaSectParser, is_prompt, parse_me_and_you};
