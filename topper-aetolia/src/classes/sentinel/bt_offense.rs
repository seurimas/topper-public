use std::collections::HashMap;

use behavior_bark::unpowered::*;
use topper_core::timeline::BaseAgentState;

use super::*;

use crate::{
    bt::*,
    classes::{VenomPlan, get_controller, get_stack},
    curatives::FirstAidSetting,
    db::*,
    defense::*,
    non_agent::AetNonAgent,
    observables::*,
    timeline::*,
    types::*,
};

pub fn get_action_plan(
    timeline: &AetTimeline,
    me: &String,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
    first_aid_settings: &mut Vec<FirstAidSetting>,
) -> ActionPlan {
    let mut controller = get_controller("sentinel", me, target, timeline, strategy, db);
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("sentinel/base")
    } else {
        format!("sentinel/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Sentinel: {:?}", you);
                }
            }
        }
        tree.resume_with(&timeline, &mut controller);
    }
    first_aid_settings.extend(controller.first_aid_settings.drain(..));
    controller.plan
}

fn add_hints(db: Option<&impl AetDatabaseModule>, controller: &mut BehaviorController) {
    if let Some(db) = db {
        // if let Some(mawcrush_freely) = db.get_hint(&MAWCRUSH_FREELY_HINT.to_string()) {
        //     controller.hint_plan(MAWCRUSH_FREELY_HINT.to_string(), mawcrush_freely);
        // }
    }
}

pub fn get_attack(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
    first_aid_settings: &mut Vec<FirstAidSetting>,
) -> String {
    let action_plan = get_action_plan(
        &timeline,
        &timeline.who_am_i(),
        &target,
        &strategy,
        db,
        first_aid_settings,
    );
    action_plan.get_inputs(&timeline)
}

fn resin_state(resin: &ResinState) -> String {
    let ResinState {
        hot,
        cold,
        burning,
        ticks_left,
    } = resin;
    let hot_cold = if hot.is_none() && cold.is_none() {
        "<gray>no resin".to_string()
    } else if cold.is_none() {
        format!("<red>{:?} (hot)", hot.as_ref().unwrap())
    } else if hot.is_none() {
        format!("<blue>{:?} (cold)", cold.as_ref().unwrap())
    } else {
        format!(
            "<blue>{:?} -> <red>{:?} ({} ticks left)",
            cold.as_ref().unwrap(),
            hot.as_ref().unwrap(),
            ticks_left
        )
    };
    if !burning.is_active() {
        format!("{} <white>(not burning)", hot_cold)
    } else {
        format!(
            "{} <red>(TTH: {:02}s)",
            hot_cold,
            burning.get_time_left_seconds()
        )
    }
}

pub fn get_class_state(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let resin = resin_state(&you.resin_state);
    format!("{}", resin)
}
