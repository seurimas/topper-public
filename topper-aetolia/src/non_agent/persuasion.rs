use std::collections::HashMap;

use crate::{non_agent::*, timeline::AetTimeline};

fn pick_target(timeline: &AetTimeline) -> Option<i64> {
    let me = timeline.state.borrow_me();
    let Some(room) = timeline.state.get_my_room() else {
        return None;
    };
    let mut target = None;
    if room.denizens.is_empty() {
        return None;
    }
    for (id) in room.denizens.iter() {
        let Some(denizen) = timeline.state.borrow_denizen(*id) else {
            println!("Denizen not found: {}", id);
            continue;
        };
        if !denizen.has_tag(MOB_TAG) {
            continue;
        }
        match denizen.persuasion_status {
            PersuasionStatus::Unscrutinised => {
                target = Some(*id);
            }
            PersuasionStatus::Scrutinised { .. } | PersuasionStatus::Persuading { .. } => {
                return Some(*id);
            }
            PersuasionStatus::NonSentient | PersuasionStatus::Convinced => {
                continue;
            }
        }
    }
    target
}

pub fn auto_persuade(
    timeline: &AetTimeline,
    primary_persuasion: &str,
    charity_fallback: bool,
    strategy: &dyn Fn(&PersuasionState, &PersuasionStatus) -> Result<String, String>,
) -> Result<String, String> {
    let me = timeline.state.borrow_me();
    if let Some(target) = me.persuasion_state.get_target() {
        // We're in combat!
        let Some(target) = timeline.state.borrow_denizen(target) else {
            return Err("Target not found".to_string());
        };
        return strategy(&me.persuasion_state, &target.persuasion_status);
    }
    if let Some(target) = pick_target(timeline) {
        let Some(denizen) = timeline.state.borrow_denizen(target) else {
            return Err("Target not found".to_string());
        };
        match denizen.persuasion_status {
            PersuasionStatus::Unscrutinised => {
                return Ok(format!("scrutinise {}", target));
            }
            PersuasionStatus::Scrutinised { unique, .. } => {
                if unique {
                    if charity_fallback {
                        return Ok(format!("persuade {} for charity", target));
                    } else {
                        // No return.
                    }
                } else {
                    return Ok(format!("persuade {} for {}", target, primary_persuasion));
                }
            }
            PersuasionStatus::Persuading { .. } => {
                return Err("Target already being persuaded, but not set!".to_string());
            }
            PersuasionStatus::NonSentient => {
                return Err("Received non-sentient target".to_string());
            }
            PersuasionStatus::Convinced => {
                return Err("Received already convinced target".to_string());
            }
        }
    }
    Ok("pcb no target".to_string())
}
