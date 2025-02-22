use regex::Regex;
use std::collections::{HashMap, HashSet};
use topper_aetolia::non_agent::auto_persuade;
use topper_aetolia::timeline::{
    for_agent, AetObservation, AetTimeSlice, AetTimeline, CombatAction,
};
use topper_core::observations::strip_ansi;
use topper_core::timeline::db::DatabaseModule;
use topper_core::timeline::CType;
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use topper_persuasion::simple_strategy::simple_strategy;
use topper_persuasion::*;

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;

lazy_static! {
    static ref EVAL: Regex = Regex::new(r#"^"(\w+)"\s+(.*)\. \((.*)\)$"#).unwrap();
    static ref START: Regex = Regex::new(r"^start (\w+)$").unwrap();
}

fn get_eval(line: String) -> Option<(String, String, String)> {
    if let Some(captures) = EVAL.captures(&line) {
        let id = captures.get(1).unwrap().as_str().to_string();
        let full_name = captures.get(2).unwrap().as_str().to_string();
        let status = captures.get(3).unwrap().as_str().to_string();
        Some((id, full_name, status))
    } else {
        None
    }
}

#[derive(Default, Debug)]
pub struct BasherModule {
    active_area: Option<String>,
    my_aggros: HashSet<String>,
    primary_persuasion: String,
    charity_fallback: bool,
}

impl BasherModule {
    pub fn new() -> Self {
        Self {
            active_area: None,
            my_aggros: HashSet::new(),
            primary_persuasion: "charity".to_string(),
            charity_fallback: false,
        }
    }
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for BasherModule {
    type Siblings = (&'s mut AetTimeline, &'s AetMudletDatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (mut timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        let mut calls = None;
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                for line in timeslice.lines.iter() {
                    let line = strip_ansi(&line.0);
                    if let Some(captures) = EVAL.captures(&line) {}
                }
                if timeslice.observations.iter().flatten().any(|obs| {
                    if let AetObservation::Scrutinise {
                        who,
                        personality,
                        resolve,
                        max_resolve,
                    } = obs
                    {
                        who.eq("NonSentient")
                    } else {
                        false
                    }
                }) {
                    calls = Some("pcb non sentient".to_string());
                }
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if module.eq("basher") {
                    match command.as_ref() {
                        "persuade" => match auto_persuade(
                            timeline,
                            &self.primary_persuasion,
                            self.charity_fallback,
                            &simple_strategy,
                        ) {
                            Ok(action) => {
                                calls = Some(action);
                            }
                            Err(e) => {
                                calls = Some(format!("echo {}", e));
                            }
                        },
                        "charity" => {
                            self.primary_persuasion = "charity".to_string();
                            self.charity_fallback = true;
                        }
                        "pickpocket" => {
                            self.primary_persuasion = "pickpocket".to_string();
                            self.charity_fallback = true;
                        }
                        "pickpocket only" => {
                            self.primary_persuasion = "pickpocket".to_string();
                            self.charity_fallback = false;
                        }
                        "debug" => match auto_persuade(
                            timeline,
                            &self.primary_persuasion,
                            self.charity_fallback,
                            &simple_strategy,
                        ) {
                            Ok(action) => {
                                calls = Some(format!("echo {}", action));
                            }
                            Err(e) => {
                                calls = Some(format!("echo {}", e));
                            }
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if let Some(calls) = calls {
            Ok(TopperResponse::passive("basher".to_string(), calls))
        } else {
            Ok(TopperResponse::silent())
        }
    }
}

#[cfg(test)]
mod basher_tests {
    use crate::topper::basher::*;
    #[test]
    fn eval_works() {
        let tlingor = r#""tlingor165930"     a baby tlingor. (uninjured)"#;
        assert_eq!(
            get_eval(tlingor.to_string()),
            Some((
                "tlingor165930".to_string(),
                "a baby tlingor".to_string(),
                "uninjured".to_string()
            ))
        );
    }
}
