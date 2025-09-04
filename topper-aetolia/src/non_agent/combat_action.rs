use regex::Regex;

use crate::db::AetDatabaseModule;
use crate::non_agent::AetTimelineDenizenExt;
use crate::non_agent::Appeals;
use crate::non_agent::Denizen;
use crate::timeline::*;
use crate::types::*;

use super::PersuasionStatus;

lazy_static! {
    static ref PERSUADE: Regex = Regex::new(r"persuade (\w+) for .*").unwrap();
    static ref SCRUTINISE: Regex = Regex::new(r"scrutinise (\w+)").unwrap();
    static ref CYCLIC: Regex = Regex::new(r"cyclic (\w+)").unwrap();
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    if let Some(captures) = PERSUADE.captures(command) {
        let me = agent_states.me.clone();
        agent_states.add_player_hint(
            &me,
            &"persuading",
            captures
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    } else if let Some(captures) = SCRUTINISE.captures(command) {
        let me = agent_states.me.clone();
        agent_states.add_player_hint(
            &me,
            &"persuading",
            captures
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    } else if let Some(captures) = CYCLIC.captures(command) {
        let me = agent_states.me.clone();
        agent_states.add_player_hint(
            &me,
            &"cyclic",
            captures
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
    db: Option<&impl AetDatabaseModule>,
) -> Result<(), String> {
    if let Some(card) = Appeals::from_name(&combat_action.skill) {
        for_agent(
            agent_states,
            &combat_action.caster,
            &move |me: &mut AgentState| {
                me.persuasion_state.appeal(card);
            },
        );
    } else {
        match combat_action.skill.as_str() {
            "Rhetoric" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        if combat_action.annotation == "end" {
                            me.persuasion_state.rhetoric_end();
                        } else {
                            me.persuasion_state.rhetoric_start();
                        }
                    },
                );
            }
            "Cyclic" => {
                if let Some(cyclic) = Appeals::from_name(&combat_action.annotation) {
                    for_agent(
                        agent_states,
                        &combat_action.caster,
                        &move |me: &mut AgentState| {
                            me.persuasion_state.cyclic = Some(cyclic);
                        },
                    );
                } else if combat_action.annotation == "already" {
                    if let Some(cycliced) = agent_states.get_my_hint(&"cyclic".to_string()) {
                        if let Ok(cyclic) = cycliced.parse::<Appeals>() {
                            for_agent(
                                agent_states,
                                &combat_action.caster,
                                &move |me: &mut AgentState| {
                                    me.persuasion_state.cyclic = Some(cyclic);
                                },
                            );
                        }
                    }
                } else if combat_action.annotation == "none" {
                    for_agent(
                        agent_states,
                        &combat_action.caster,
                        &move |me: &mut AgentState| {
                            me.persuasion_state.cyclic = None;
                        },
                    );
                }
            }
            "Persuade" => {
                let persuaded = get_persuasion_target(agent_states);
                if combat_action.annotation == "convinced" {
                    // Set the persuasion status to convinced.
                    if let Some(persuaded) = persuaded {
                        agent_states.for_denizen(persuaded, &|denizen: &mut Denizen| {
                            denizen.persuasion_status = PersuasionStatus::Convinced;
                        });
                    }
                } else if combat_action.annotation == "aversion" {
                    // Pretend they're unique, so we use a different persuasion.
                    if let Some(persuaded) = persuaded {
                        agent_states.for_denizen(persuaded, &|denizen: &mut Denizen| {
                            denizen.persuasion_status = PersuasionStatus::Scrutinised {
                                resolve: denizen.persuasion_status.resolve(),
                                max_resolve: denizen.persuasion_status.max_resolve(),
                                personality: denizen.persuasion_status.personality(),
                                weakened: vec![],
                                unique: true,
                            };
                        });
                    }
                } else {
                    if let Some(persuaded) = persuaded {
                        agent_states.for_denizen(persuaded, &|denizen: &mut Denizen| {
                            denizen.persuasion_status.start_persuasion();
                        });
                        for_agent(
                            agent_states,
                            &combat_action.caster,
                            &move |me: &mut AgentState| {
                                me.persuasion_state.start_persuasion(persuaded);
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn get_persuasion_target(
    agent_states: &topper_core::timeline::TimelineState<AgentState, crate::non_agent::AetNonAgent>,
) -> Option<i64> {
    if let Some(persuading) = agent_states.get_my_hint(&"persuading".to_string()) {
        if let Ok(id) = persuading.parse::<i64>() {
            Some(id)
        } else {
            agent_states
                .find_denizens_in_room(agent_states.borrow_me().room_id)
                .iter()
                .find(|denizen_id| {
                    let denizen = agent_states.borrow_denizen(**denizen_id).unwrap();
                    denizen.full_name.contains(&persuading)
                })
                .copied()
        }
    } else {
        None
    }
}
