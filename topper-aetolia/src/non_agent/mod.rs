pub mod denizen;
pub mod persuasion;
pub mod rooms;
pub use denizen::*;
pub use persuasion::*;
pub use rooms::*;
use serde::Deserialize;
use topper_core::timeline::NonAgentState;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AetNonAgent {
    Room(Room),
    Denizen(Denizen),
    Players(Vec<String>),
}

impl AetNonAgent {
    pub fn as_denizen(&self) -> Option<&Denizen> {
        match self {
            AetNonAgent::Denizen(denizen) => Some(denizen),
            _ => None,
        }
    }
    pub fn as_denizen_mut(&mut self) -> Option<&mut Denizen> {
        match self {
            AetNonAgent::Denizen(denizen) => Some(denizen),
            _ => None,
        }
    }

    pub fn as_room(&self) -> Option<&Room> {
        match self {
            AetNonAgent::Room(room) => Some(room),
            _ => None,
        }
    }

    pub fn as_room_mut(&mut self) -> Option<&mut Room> {
        match self {
            AetNonAgent::Room(room) => Some(room),
            _ => None,
        }
    }
}

impl NonAgentState for AetNonAgent {
    fn wait(&mut self, time: i32) {
        match self {
            AetNonAgent::Room(room) => room.wait(time),
            // AetNonAgent::Denizen(denizen) => denizen.wait(time),
            _ => {}
        }
    }
}
