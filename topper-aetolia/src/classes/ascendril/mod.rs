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

use crate::non_agent::{AetTimelineDenizenExt, AetTimelineRoomExt};

use super::{AetTimelineState, PhenomenaState};

pub fn phenomenon_in_room(
    agent_states: &AetTimelineState,
    room_id: i64,
    phenomenon: PhenomenaState,
) -> bool {
    if let Some(room) = agent_states.get_my_room() {
        for denizen_id in room.denizens.iter() {
            if let Some(denizen) = agent_states.borrow_denizen(*denizen_id) {
                if denizen.full_name.contains(phenomenon.name()) {
                    return true;
                }
            }
        }
        false
    } else {
        false
    }
}
