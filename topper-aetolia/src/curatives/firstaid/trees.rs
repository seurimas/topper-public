use topper_bt::unpowered::*;

use crate::{
    bt::{get_tree, BehaviorController},
    classes::Class,
    curatives::AetTimeline,
    observables::ActionPlan,
};

use super::FirstAidSetting;

fn get_first_aid_controller(target: Option<String>) -> BehaviorController {
    BehaviorController {
        target,

        ..Default::default()
    }
}

pub fn get_firstaid_settings_for_class(
    timeline: &AetTimeline,
    me: &String,
    target: &Option<String>,
    class: Class,
) -> Vec<FirstAidSetting> {
    let mut controller = get_first_aid_controller(target.clone());
    let tree_name = format!(
        "firstaid/classes/{}",
        class.normal().to_string().to_lowercase()
    );
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        tree.resume_with(&timeline, &mut controller);
    }
    controller.first_aid_settings
}

pub fn get_firstaid_settings_no_class(
    timeline: &AetTimeline,
    me: &String,
    target: &Option<String>,
) -> Vec<FirstAidSetting> {
    let mut controller = get_first_aid_controller(target.clone());
    let tree_name = "firstaid/classless".to_string();
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        tree.resume_with(&timeline, &mut controller);
    }
    controller.first_aid_settings
}
