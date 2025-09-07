use std::collections::HashMap;

use behavior_bark::unpowered::*;
use topper_core::timeline::BaseAgentState;

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
    let mut controller = get_controller("zealot", me, target, timeline, strategy, db);
    controller.init_zealot();
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("zealot/base")
    } else {
        format!("zealot/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Zealot: {:?}", you);
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

fn format_pendulum(difference: CType) -> String {
    if difference > 1000 {
        format!("<green>{}", difference / 100)
    } else if difference < -1000 {
        format!("<red>{}", difference / 100)
    } else {
        format!("<white>{}", difference / 100)
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
    let pyromania = me
        .check_if_zealot(&|z| z.pyromania.active())
        .unwrap_or(false);
    let zenith = me
        .check_if_zealot(&|z| {
            if z.zenith.active() {
                "<yellow>ZENITH"
            } else if z.zenith.can_initiate() {
                "<white>------"
            } else if let Some(remaining) = z.zenith.time_to_active() {
                match (remaining / BALANCE_SCALE as i32) {
                    0 => "<gray>ZENITH",
                    1 => "<gray>ZENIT-",
                    2 => "<gray>ZENI--",
                    3 => "<gray>ZEN---",
                    4 => "<gray>ZE----",
                    5 => "<gray>Z-----",
                    _ => "<gray>------",
                }
            } else {
                "<gray>------"
            }
        })
        .unwrap_or("<white>------");
    let pendulum_display = if me.balanced(BType::pendulum()) && you.get_restoring().is_some() {
        let pendulum_values = get_pendulum_values(&you, false);
        let reverse_pendulum_values = get_pendulum_values(&you, true);
        let best_values =
            if pendulum_values.iter().sum::<i32>() > reverse_pendulum_values.iter().sum::<i32>() {
                pendulum_values
            } else {
                reverse_pendulum_values
            };
        format!(
            "{:>3} {:>3}\n{:>3} {:>3}\n{:>3} {:>3}",
            format_pendulum(best_values[0]),
            format_pendulum(best_values[1]),
            format_pendulum(best_values[2]),
            format_pendulum(best_values[3]),
            format_pendulum(best_values[4]),
            format_pendulum(best_values[5]),
        )
    } else {
        "<gray>------".to_string()
    };
    format!(
        "{}\n{}\n{}",
        if pyromania {
            "<red>PYROMANIA"
        } else {
            "<white>---------"
        },
        zenith,
        pendulum_display
    )
}
