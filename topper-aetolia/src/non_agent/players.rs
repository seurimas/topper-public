use std::collections::HashMap;

use serde::Deserialize;

use crate::timeline::AetTimelineState;

use super::AetNonAgent;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct PlayerInfo {
    pub priority: i32,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
pub struct Players {
    pub players: HashMap<String, PlayerInfo>,
}

impl Players {
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.players.keys()
    }

    pub fn get(&self, name: &str) -> Option<&PlayerInfo> {
        self.players.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut PlayerInfo> {
        self.players.get_mut(name)
    }
}

pub fn format_players_key(owner: &str, list: &str) -> String {
    format!("{}_{}", owner, list)
}

pub trait AetTimelinePlayersExt {
    fn for_players(&mut self, owner: &str, list: &str, action: &dyn Fn(&mut Players));

    fn get_players(&self, owner: &str, list: &str) -> Option<&Players>;

    fn is_player_list_missing(&self, owner: &str, list: &str) -> bool {
        self.get_players(owner, list).is_none()
    }

    fn is_player_in_list(&self, owner: &str, list: &str, name: &str) -> bool {
        self.get_players(owner, list)
            .map(|players| players.players.contains_key(name))
            .unwrap_or(false)
    }

    fn clear_players(&mut self, owner: &str, list: &str) {
        self.for_players(owner, list, &|players| {
            players.players.clear();
        });
    }

    fn add_player(&mut self, owner: &str, list: &str, name: &str) {
        let name = name.to_string();
        self.for_players(owner, list, &|players| {
            players.players.entry(name.clone()).or_default();
        });
    }

    fn remove_player(&mut self, owner: &str, list: &str, name: &str) {
        let name = name.to_string();
        self.for_players(owner, list, &|players| {
            players.players.remove(&name);
        });
    }

    fn set_player_priority(&mut self, owner: &str, list: &str, name: &str, priority: i32) {
        let name = name.to_string();
        self.for_players(owner, list, &|players| {
            let info = players.players.entry(name.clone()).or_default();
            info.priority = priority;
        });
    }

    fn get_players_by_priority(&self, owner: &str, list: &str) -> Vec<(String, i32)> {
        if let Some(players) = self.get_players(owner, list) {
            let mut player_list: Vec<(String, i32)> = players
                .players
                .iter()
                .map(|(name, info)| (name.clone(), info.priority))
                .collect();
            player_list.sort_by_key(|(_, priority)| *priority);
            player_list
        } else {
            vec![]
        }
    }
}

impl AetTimelinePlayersExt for AetTimelineState {
    fn for_players(&mut self, owner: &str, list: &str, action: &dyn Fn(&mut Players)) {
        let key = format_players_key(owner, list);
        if let Some(AetNonAgent::Players(players)) = self.non_agent_states.get_mut(&key) {
            action(players);
        } else {
            self.non_agent_states
                .insert(key.clone(), AetNonAgent::Players(Players::default()));
            self.for_players(owner, list, action);
        }
    }

    fn get_players(&self, owner: &str, list: &str) -> Option<&Players> {
        self.non_agent_states
            .get(&format_players_key(owner, list))
            .and_then(AetNonAgent::as_players)
    }
}
