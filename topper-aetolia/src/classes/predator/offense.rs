use std::collections::HashMap;

use topper_bt::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{get_controller, get_stack, VenomPlan},
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
) -> ActionPlan {
    let mut controller = get_controller("predator", me, target, timeline, strategy, db);
    let tree_name = if strategy.eq("class") {
        format!("predator/base")
    } else {
        format!("predator/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        tree.resume_with(&timeline, &mut controller);
    }
    controller.plan
}
