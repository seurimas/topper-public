use std::{collections::HashMap, sync::RwLock};

use behavior_bark::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{VenomPlan, VenomType, get_controller, get_stack},
    curatives::{FirstAidSetting, get_cure_depth},
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
    let mut controller = get_controller("infiltrator", me, target, timeline, strategy, db);
    controller.init_infiltrator();
    *controller.hypno_stack() = get_hypno_stack(timeline, target, strategy, db);
    let tree_name = if strategy.eq("class") {
        format!("infiltrator/base")
    } else {
        format!("infiltrator/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Infiltrator: {:?}", you);
                }
            }
        }
        tree.resume_with(&timeline, &mut controller);
    }
    first_aid_settings.extend(controller.first_aid_settings.drain(..));
    controller.plan
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

pub fn add_delphs(you: &AgentState, venoms: &mut Vec<VenomType>, count: usize) -> bool {
    if you.is(FType::Allergies) || you.is(FType::Vomiting) {
        return false;
    }
    let mut delphing = false;
    if you.is(FType::Hypersomnia) {
        match (
            you.is(FType::Insomnia),
            you.is(FType::Asleep),
            you.is(FType::Instawake),
        ) {
            (true, false, true) => {
                if get_cure_depth(you, FType::Hypersomnia).cures > 1 {
                    venoms.insert(0, "delphinium");
                    delphing = true;
                }
            }
            (false, false, true) => {
                if count == 2 {
                    venoms.insert(0, "delphinium");
                    venoms.insert(0, "delphinium");
                    delphing = true;
                }
            }
            (true, _, _) | (_, false, _) | (_, _, true) => {
                venoms.insert(0, "delphinium");
                delphing = true;
            }
            _ => {}
        }
        if !delphing {
            return false;
        }
        if venoms.len() >= count && Some(&"darkshade") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
        if venoms.len() >= count && Some(&"euphorbia") == venoms.get(venoms.len() - count) {
            venoms.remove(venoms.len() - count);
        }
    }
    delphing
}

pub fn get_top_suggestion(target: &AgentState, hypnos: &Vec<Hypnosis>) -> Option<Hypnosis> {
    let mut hypno_idx = 0;
    for i in 0..target.hypno_state.suggestion_count() {
        if target.hypno_state.get_suggestion(i) == hypnos.get(hypno_idx) {
            hypno_idx += 1;
        }
    }
    if hypno_idx < hypnos.len() {
        hypnos.get(hypno_idx).map(|hypno| hypno.clone())
    } else {
        None
    }
}

pub fn get_hypno_stack_name(timeline: &AetTimeline, target: &String, strategy: &String) -> String {
    timeline
        .state
        .get_my_hint(&"HYPNO_STACK".to_string())
        .unwrap_or(strategy.to_string())
}

lazy_static! {
    static ref HARD_HYPNO: Vec<Hypnosis> = vec![
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
        Hypnosis::Aff(FType::Hypochondria),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Vertigo),
        Hypnosis::Aff(FType::Impatience),
        Hypnosis::Aff(FType::Loneliness),
    ];
}

pub fn get_hypno_stack<'s>(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> Vec<Hypnosis> {
    let stack_name = get_hypno_stack_name(timeline, target, strategy);
    get_hypno_stack_from_file(&stack_name)
        .or_else(|| {
            db.and_then(|db| {
                if stack_name == "normal" {
                    None // Default to HARD_HYPNO
                } else if stack_name == "class" {
                    if let Some(class) = db.get_class(target) {
                        db.get_hypno_plan(&class.to_string())
                    } else {
                        db.get_hypno_plan(&format!("hypno_{}", stack_name))
                    }
                } else {
                    db.get_hypno_plan(&format!("hypno_{}", stack_name))
                }
            })
        })
        .unwrap_or(HARD_HYPNO.to_vec())
}

pub static mut LOAD_HYPNO_STACK_FUNC: Option<fn(&String, &String) -> String> = None;

lazy_static! {
    pub static ref LOADED_HYPNO_STACKS: RwLock<HashMap<String, Option<Vec<Hypnosis>>>> =
        { RwLock::new(HashMap::new()) };
}

pub fn clear_hypnostacks() {
    let mut stacks = LOADED_HYPNO_STACKS.write().unwrap();
    stacks.clear();
}

pub fn get_hypno_stack_from_file(stack_name: &String) -> Option<Vec<Hypnosis>> {
    {
        let stacks = LOADED_HYPNO_STACKS.read().unwrap();
        if let Some(stack) = stacks.get(stack_name) {
            return stack.clone();
        }
    }
    {
        let mut trees = LOADED_HYPNO_STACKS.write().unwrap();
        let stack_json =
            unsafe { LOAD_HYPNO_STACK_FUNC.unwrap()(&"hypnosis".to_string(), stack_name) };
        // println!("Loading {} stack ({})", stack_name, stack_json.len());
        match serde_json::from_str::<Vec<Hypnosis>>(&stack_json) {
            Ok(stack_def) => {
                trees.insert(stack_name.clone(), Some(stack_def.clone()));
                Some(stack_def)
            }
            Err(err) => {
                println!("Failed to load {}: {:?}", stack_name, err);
                trees.insert(stack_name.clone(), None);
                None
            }
        }
    }
}
