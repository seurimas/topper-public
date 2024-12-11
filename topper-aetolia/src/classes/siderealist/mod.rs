pub mod observation_handling;
pub use observation_handling::*;
pub mod actions;
pub use actions::*;
pub mod offense;
pub use offense::*;
pub mod behavior;
pub use behavior::*;
pub mod predicate;
pub use predicate::*;

use super::Vibration;

use crate::non_agent::{AetTimelineDenizenExt, AetTimelineRoomExt};

use super::AetTimelineState;

pub fn vibration_in_room(
    agent_states: &AetTimelineState,
    room_id: i64,
    vibration: Vibration,
) -> bool {
    if let Some(room) = agent_states.get_my_room() {
        let me = &agent_states.me;
        room.has_vibration(vibration, me)
    } else {
        false
    }
}

pub fn vibration_dormant_in_room(
    agent_states: &AetTimelineState,
    room_id: i64,
    vibration: Vibration,
) -> bool {
    if let Some(room) = agent_states.get_my_room() {
        let me = &agent_states.me;
        room.vibration_dormant(vibration, me)
    } else {
        false
    }
}
