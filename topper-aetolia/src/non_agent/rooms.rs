use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::{
    agent::{Timer, Vibration},
    timeline::*,
};

use super::AetNonAgent;

#[derive(Debug, Deserialize, PartialEq, Clone, Hash, Eq)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    Up,
    Down,
    In,
    Out,
}

impl Direction {
    pub fn from_short(short_name: &str) -> Option<Direction> {
        match short_name {
            "n" => Some(Direction::North),
            "ne" => Some(Direction::Northeast),
            "e" => Some(Direction::East),
            "se" => Some(Direction::Southeast),
            "s" => Some(Direction::South),
            "sw" => Some(Direction::Southwest),
            "w" => Some(Direction::West),
            "nw" => Some(Direction::Northwest),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "in" => Some(Direction::In),
            "out" => Some(Direction::Out),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VibrationState {
    owners: HashMap<String, Option<Timer>>,
    unknown_at: Option<Timer>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Room {
    pub id: i64,
    pub players: HashSet<String>,
    pub denizens: HashSet<i64>,
    pub exits: HashMap<Direction, i64>,
    pub vibrations: HashMap<Vibration, VibrationState>,
    tags: HashSet<String>,
}

impl Default for Room {
    fn default() -> Self {
        Room {
            id: 0,
            players: HashSet::new(),
            denizens: HashSet::new(),
            exits: HashMap::new(),
            tags: HashSet::new(),
            vibrations: HashMap::new(),
        }
    }
}

impl Room {
    pub fn default_state(room_id: i64) -> AetNonAgent {
        AetNonAgent::Room(Room {
            id: room_id,
            players: HashSet::new(),
            denizens: HashSet::new(),
            exits: HashMap::new(),
            tags: HashSet::new(),
            vibrations: HashMap::new(),
        })
    }

    pub fn add_tag(&mut self, tag: impl ToString) {
        self.tags.insert(tag.to_string());
    }

    pub fn remove_tag(&mut self, tag: impl ToString) {
        self.tags.remove(&tag.to_string());
    }

    pub fn has_tag(&self, tag: impl ToString) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

pub fn format_room_id(room_id: i64) -> String {
    format!("room_{}", room_id)
}

pub trait AetTimelineRoomExt {
    fn for_room(&mut self, room_id: i64, action: &Fn(&mut Room));

    fn get_my_room(&self) -> Option<&Room>;

    fn get_room_id(&self) -> i64;

    fn get_my_room_mut(&mut self) -> Option<&mut Room>;

    fn get_room(&self, room_id: i64) -> Option<&Room>;

    fn get_room_mut(&mut self, room_id: i64) -> Option<&mut Room>;

    fn set_player_room(&mut self, room_id: i64, player: &str);

    fn observe_vibration_effect(&mut self, room_id: i64, vibration: Vibration, affected: &str);

    fn observe_vibration_in_room(
        &mut self,
        room_id: i64,
        vibration: Vibration,
        owner: &str,
        timer: Option<Timer>,
    );

    fn begin_vibrations_list(&mut self, room_id: i64);
}

impl AetTimelineRoomExt for AetTimelineState {
    fn for_room(&mut self, room_id: i64, action: &Fn(&mut Room)) {
        if room_id == 0 {
            // Do not do anything to the null room. It's wasted breath.
            return;
        }
        if let Some(AetNonAgent::Room(room)) =
            self.non_agent_states.get_mut(&format_room_id(room_id))
        {
            action(room);
        } else {
            self.non_agent_states
                .insert(format_room_id(room_id), Room::default_state(room_id));
            self.for_room(room_id, action);
        }
    }

    fn get_room_id(&self) -> i64 {
        self.borrow_me().room_id
    }

    fn get_my_room(&self) -> Option<&Room> {
        let room_id = self.borrow_me().room_id;
        self.non_agent_states
            .get(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room)
    }

    fn get_my_room_mut(&mut self) -> Option<&mut Room> {
        let room_id = self.borrow_me().room_id;
        self.non_agent_states
            .get_mut(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room_mut)
    }

    fn get_room(&self, room_id: i64) -> Option<&Room> {
        self.non_agent_states
            .get(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room)
    }

    fn get_room_mut(&mut self, room_id: i64) -> Option<&mut Room> {
        self.non_agent_states
            .get_mut(&format_room_id(room_id))
            .and_then(AetNonAgent::as_room_mut)
    }

    fn set_player_room(&mut self, room_id: i64, player: &str) {
        let player = player.to_string();
        let old_room_id = self.borrow_agent(&player).room_id;
        self.for_room(old_room_id, &|room| {
            room.players.remove(&player);
        });
        self.for_room(room_id, &|room| {
            room.players.insert(player.clone());
        });
        self.for_agent(&player, &|me| {
            me.room_id = room_id;
        });
    }

    fn observe_vibration_effect(&mut self, room_id: i64, vibration: Vibration, affected: &str) {
        self.for_room(room_id, &|room| {
            let vibration_state =
                room.vibrations
                    .entry(vibration)
                    .or_insert_with(|| VibrationState {
                        owners: HashMap::new(),
                        unknown_at: None,
                    });
            if vibration_state
                .owners
                .iter()
                .any(|(owner, _)| owner != affected)
            {
                // Do nothing, we're accounted for.
            } else {
                vibration_state.unknown_at = Some(Timer::count_down_seconds(15.));
            }
        });
    }

    fn observe_vibration_in_room(
        &mut self,
        room_id: i64,
        vibration: Vibration,
        owner: &str,
        timer: Option<Timer>,
    ) {
        self.for_room(room_id, &|room| {
            let vibration_state =
                room.vibrations
                    .entry(vibration)
                    .or_insert_with(|| VibrationState {
                        owners: HashMap::new(),
                        unknown_at: None,
                    });
            vibration_state.owners.insert(owner.to_string(), timer);
        });
    }

    fn begin_vibrations_list(&mut self, room_id: i64) {
        self.for_room(room_id, &|room| {
            room.vibrations.clear();
        });
    }
}
