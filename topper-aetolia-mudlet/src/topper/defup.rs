use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::BattleModule;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sled::Tree;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::AetTimeline;
use topper_aetolia::types::*;
use topper_aetolia::{classes::*, timeline::AetTimeSlice};
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

#[derive(Debug, Default)]
pub struct DefupModule {
    next_def: String,
    defence_order: Vec<String>,
    defence_class: Option<Class>,
    defence_player: Option<String>,
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for DefupModule {
    type Siblings = (&'s AetTimeline, &'s AetMudletDatabaseModule);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                let me = timeline.who_am_i();
                let my_agent = timeline.state.borrow_agent(&me);
                let mut require_update = false;
                if self.defence_player.as_ref() != Some(&me) {
                    self.defence_player = Some(me.clone());
                    require_update = true;
                }
                let my_class = db.get_class(&me).map(|c| c.normal());
                if self.defence_class != my_class {
                    self.defence_class = my_class;
                    require_update = true;
                }
                if require_update {
                    let mut player_defences = db.get_defense_order(&me).unwrap_or_default();
                    let class_defences = self
                        .defence_class
                        .and_then(|c| db.get_defense_order(&c.to_string()))
                        .unwrap_or_default();
                    player_defences.extend(class_defences);
                    self.defence_order = player_defences;
                }

                if let Some(next_def_cmd) =
                    get_next_defence_command(&self.defence_order, &my_agent, db)
                {
                    if next_def_cmd != self.next_def {
                        self.next_def = next_def_cmd.clone();
                        return Ok(TopperResponse::passive(
                            "defup".to_string(),
                            format!("qeb {}", next_def_cmd),
                        ));
                    }
                } else {
                    self.next_def.clear();
                    return Ok(TopperResponse::passive("defup".to_string(), "".to_string()));
                }
            }
            TopperMessage::Request(TopperRequest::ModuleMsg(module, message)) => {
                if !module.eq("defup") {
                    return Ok(TopperResponse::silent());
                }
                if let Some(command) = parse_defense_submodule(message) {
                    match command {
                        DefenseCommand::Check => {
                            println!(
                                "Defup: next defense {}, order {:?}",
                                self.next_def, self.defence_order
                            );
                            if let Some(class) = self.defence_class {
                                println!("Defup: class {:?}", class);
                            }
                            if let Some(player) = &self.defence_player {
                                println!("Defup: player {:?}", player);
                            }
                            println!("Defups list:");
                            for defense in &self.defence_order {
                                let command = db
                                    .get_defense_command(defense)
                                    .unwrap_or_else(|| "No command".to_string());
                                let triggers_count = count_defense_triggers(db, defense);
                                println!(" - {} ({}): {}", defense, triggers_count, command);
                            }
                            if !self.next_def.is_empty() {
                                return Ok(TopperResponse::passive(
                                    "defup".to_string(),
                                    self.next_def.clone(),
                                ));
                            }
                        }
                        DefenseCommand::AddCommand { command, defense } => {
                            db.add_defense_command(&defense, &command);
                        }
                        DefenseCommand::RemoveCommand { defense } => {
                            db.remove_defense_command(&defense);
                        }
                        DefenseCommand::AddUpTrigger { trigger, defense } => {
                            db.add_defense_trigger(&defense, &trigger);
                        }
                        DefenseCommand::RemoveUpTrigger { trigger } => {
                            db.remove_defense_trigger(&trigger);
                        }
                        DefenseCommand::AddDownTrigger { trigger, defense } => {
                            db.add_defense_missing_trigger(&defense, &trigger);
                        }
                        DefenseCommand::RemoveDownTrigger { trigger } => {
                            db.remove_defense_missing_trigger(&trigger);
                        }
                        DefenseCommand::SetPlayerOrder { order } => {
                            if let Some(player) = &self.defence_player {
                                db.set_defense_order(player, order);
                                self.defence_class = None;
                                self.defence_player = None;
                            } else {
                                return Err("No player set, cannot set player order".to_string());
                            }
                        }
                        DefenseCommand::SetClassOrder { order } => {
                            if let Some(class) = &self.defence_class {
                                db.set_defense_order(&class.to_string(), order);
                                self.defence_class = None;
                                self.defence_player = None;
                            } else {
                                return Err("No class set, cannot set class order".to_string());
                            }
                        }
                    }
                } else {
                    return Err("Cannot parse defup command".to_string());
                }
            }
            _ => {}
        }
        Ok(TopperResponse::silent())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DefenseCommand {
    Check,
    AddCommand { command: String, defense: String },
    RemoveCommand { defense: String },
    AddUpTrigger { trigger: String, defense: String },
    RemoveUpTrigger { trigger: String },
    AddDownTrigger { trigger: String, defense: String },
    RemoveDownTrigger { trigger: String },
    SetPlayerOrder { order: Vec<String> },
    SetClassOrder { order: Vec<String> },
}

fn parse_defense_submodule(message: &String) -> Option<DefenseCommand> {
    serde_json::from_str(message).ok()
}

fn get_next_defence(defence_order: &Vec<String>, agent: &AgentState) -> Option<String> {
    for def in defence_order {
        if let Some(flag) = FType::from_name(def) {
            if !agent.is(flag) {
                return Some(def.clone());
            }
        } else if !agent.unknown_flags.has_flag(def) {
            return Some(def.clone());
        }
    }
    None
}

fn get_next_defence_command(
    defence_order: &Vec<String>,
    agent: &AgentState,
    db: &AetMudletDatabaseModule,
) -> Option<String> {
    let next_def = get_next_defence(defence_order, agent)?;
    Some(
        db.get_defense_command(&next_def)
            .unwrap_or_else(|| format!("echo No command for {}", next_def)),
    )
}

fn count_defense_triggers(db: &AetMudletDatabaseModule, defense: &String) -> usize {
    let mut count = 0;
    if let Ok(triggers_tree) = db.db.open_tree("defense_triggers") {
        count += count_defense_triggers_by_tree(&triggers_tree, defense);
    }
    if let Ok(missing_triggers_tree) = db.db.open_tree("defense_missing_triggers") {
        count += count_defense_triggers_by_tree(&missing_triggers_tree, defense);
    }
    count
}

fn count_defense_triggers_by_tree(tree: &Tree, counted_defense: &String) -> usize {
    let mut count = 0;
    let iter = tree.iter();
    for item in iter {
        if let Ok((_trigger, defense)) = item {
            if let Ok(defense_str) = String::from_utf8(defense.to_vec()) {
                if defense_str.eq(counted_defense) {
                    count += 1;
                }
            }
        }
    }
    count
}
