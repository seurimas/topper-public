use std::collections::HashSet;

use serde::Deserialize;

use crate::timeline::AetTimelineState;

use super::{AetNonAgent, AetTimelineRoomExt, PersuasionStatus};

pub const MOB_TAG: &str = "mob";
pub const CONVINCED_TAG: &str = "convinced";

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum EvalStatus {
    Uninjured,
    SlightlyBruised,
    HeavilyBruised,
    SeveralOpenWounds,
    CoveredInBlood,
    BleedingHeavily,
    AlmostDead,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Denizen {
    pub room_id: i64,
    pub full_name: String,
    pub status: EvalStatus,
    pub aggroed: bool,
    pub persuasion_status: PersuasionStatus,
    pub tags: HashSet<String>,
}

impl Denizen {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

pub fn format_denizen_id(room_id: i64) -> String {
    format!("npc_{}", room_id)
}

pub trait AetTimelineDenizenExt {
    fn add_denizen(
        &mut self,
        denizen_id: i64,
        room_id: i64,
        full_name: String,
        status: Option<EvalStatus>,
    );

    fn borrow_denizen(&self, denizen_id: i64) -> Option<&Denizen>;

    fn for_denizen(&mut self, denizen_id: i64, action: &Fn(&mut Denizen));

    fn find_denizens_in_room(&self, room_id: i64) -> Vec<i64>;

    fn kill_denizen(&mut self, denizen_id: i64);

    fn observe_denizen_out_of_room(&mut self, denizen_id: i64, room_id: i64);

    fn observe_denizen_in_room(&mut self, denizen_id: i64, room_id: i64, convinced: Option<bool>);

    fn check_denizen<R>(&self, denizen_id: i64, predicate: &Fn(&Denizen) -> R) -> Option<R>;

    fn denizen_has_tag(&self, denizen_id: i64, tag: String) -> bool {
        self.check_denizen(denizen_id, &|denizen: &Denizen| denizen.tags.contains(&tag))
            .unwrap_or(false)
    }
}

impl AetTimelineDenizenExt for AetTimelineState {
    fn add_denizen(
        &mut self,
        denizen_id: i64,
        room_id: i64,
        full_name: String,
        status: Option<EvalStatus>,
    ) {
        let key = format_denizen_id(denizen_id);
        if let Some(existing) = self
            .non_agent_states
            .get(&key)
            .and_then(AetNonAgent::as_denizen)
        {
            self.for_room(existing.room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
            self.for_room(room_id, &|mut room| {
                room.denizens.insert(denizen_id);
            });
            self.for_denizen(denizen_id, &|mut denizen| {
                denizen.room_id = room_id;
            });
        } else {
            self.non_agent_states.insert(
                key,
                AetNonAgent::Denizen(Denizen {
                    full_name,
                    room_id,
                    status: status.unwrap_or(EvalStatus::Uninjured),
                    aggroed: false,
                    persuasion_status: PersuasionStatus::Unscrutinised,
                    tags: HashSet::new(),
                }),
            );
            self.for_room(room_id, &|mut room| {
                room.denizens.insert(denizen_id);
            });
        }
    }

    fn borrow_denizen(&self, denizen_id: i64) -> Option<&Denizen> {
        self.non_agent_states
            .get(&format_denizen_id(denizen_id))
            .and_then(AetNonAgent::as_denizen)
    }

    fn kill_denizen(&mut self, denizen_id: i64) {
        if let Some(room_id) = self.check_denizen(denizen_id, &|denizen| denizen.room_id) {
            self.for_room(room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
        }
        self.non_agent_states.remove(&format_denizen_id(denizen_id));
    }

    fn observe_denizen_out_of_room(&mut self, denizen_id: i64, room_id: i64) {
        let previous_room_id = self.check_denizen(denizen_id, &|denizen| denizen.room_id);
        if let Some(previous_room_id) = previous_room_id {
            self.for_room(previous_room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
        }
    }

    fn observe_denizen_in_room(&mut self, denizen_id: i64, room_id: i64, convinced: Option<bool>) {
        let previous_room_id = self.check_denizen(denizen_id, &|denizen| denizen.room_id);
        if let Some(previous_room_id) = previous_room_id {
            self.for_room(previous_room_id, &|mut room| {
                room.denizens.remove(&denizen_id);
            });
        }
        self.for_denizen(denizen_id, &|mut denizen| {
            denizen.room_id = room_id;
            if let Some(convinced) = convinced {
                if convinced {
                    denizen.persuasion_status = PersuasionStatus::Convinced;
                }
            }
        });
        self.for_room(room_id, &|mut room| {
            room.denizens.insert(denizen_id);
        })
    }

    fn for_denizen(&mut self, denizen_id: i64, action: &Fn(&mut Denizen)) {
        if denizen_id == 0 {
            // Do not do anything to the null room. It's wasted breath.
            return;
        }

        if let Some(denizen) = self
            .non_agent_states
            .get_mut(&format_denizen_id(denizen_id))
            .and_then(AetNonAgent::as_denizen_mut)
        {
            action(denizen);
        }
    }

    fn find_denizens_in_room(&self, room_id: i64) -> Vec<i64> {
        if let Some(room) = self.get_room(room_id) {
            room.denizens.iter().copied().collect()
        } else {
            vec![]
        }
    }

    fn check_denizen<R>(&self, denizen_id: i64, predicate: &Fn(&Denizen) -> R) -> Option<R> {
        if let Some(denizen) = self
            .non_agent_states
            .get(&format_denizen_id(denizen_id))
            .and_then(AetNonAgent::as_denizen)
        {
            Some(predicate(denizen))
        } else {
            None
        }
    }
}
