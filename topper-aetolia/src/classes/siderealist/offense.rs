use std::collections::HashMap;

use behavior_bark::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{get_controller, get_stack, VenomPlan},
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
    let mut controller = get_controller("siderealist", me, target, timeline, strategy, db);
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("siderealist/base")
    } else {
        format!("siderealist/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Siderealist: {:?}", you);
                }
            }
        }
        tree.resume_with(&timeline, &mut controller);
    }
    first_aid_settings.extend(controller.first_aid_settings.drain(..));
    controller.plan
}

fn add_hints(db: Option<&impl AetDatabaseModule>, controller: &mut BehaviorController) {
    // if let Some(db) = db {
    //     if let Some(mawcrush_freely) = db.get_hint(&MAWCRUSH_FREELY_HINT.to_string()) {
    //         controller.hint_plan(MAWCRUSH_FREELY_HINT.to_string(), mawcrush_freely);
    //     }
    // }
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

pub fn get_class_state(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let gleam = if let Some(stars) = me
        .check_if_siderealist(&|me| me.gleam_stars.clone())
        .flatten()
    {
        format!(
            "<red>{}<orange>{}<yellow>{}<green>{}<blue>{}<white>{}<magenta>{}",
            if stars.red.is_active() { " " } else { "*" },
            if stars.orange.is_active() { " " } else { "*" },
            if stars.yellow.is_active() { " " } else { "*" },
            if stars.green.is_active() { " " } else { "*" },
            if stars.blue.is_active() { " " } else { "*" },
            if stars.indigo.is_active() { " " } else { "*" },
            if stars.violet.is_active() { " " } else { "*" },
        )
    } else {
        "<white>-------".to_string()
    };
    let moonlet = if you.siderealist_board.has_moonlet() {
        "<green>M".to_string()
    } else {
        "<white>-".to_string()
    };
    let asterism = if you.siderealist_board.has_asterism() {
        "<green>A".to_string()
    } else {
        "<white>-".to_string()
    };
    let dustring = if you.siderealist_board.has_dustring() {
        "<green>D".to_string()
    } else {
        "<white>-".to_string()
    };
    let parallax = me
        .check_if_siderealist(&|me| {
            if let Some((time, spell, p_target)) = me.get_parallax() {
                if target.eq_ignore_ascii_case(p_target) {
                    format!(
                        "<magenta>{} {}: {:.2}",
                        spell,
                        p_target,
                        time.get_time_left_seconds()
                    )
                } else {
                    format!(
                        "<green>{} {}: {:.2}",
                        spell,
                        p_target,
                        time.get_time_left_seconds()
                    )
                }
            } else {
                "<white>-------".to_string()
            }
        })
        .unwrap_or("<white>-------".to_string());

    format!(
        "{}{}{}\n\n{}\n{}",
        moonlet, asterism, dustring, gleam, parallax,
    )
}
