use std::collections::HashMap;

use behavior_bark::unpowered::*;

use super::*;

use crate::{
    bt::*,
    classes::{VenomPlan, get_controller, get_stack},
    combat::offense_display::heal_pressure_row,
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
    let mut controller = get_controller("ascendril", me, target, timeline, strategy, db);
    add_hints(db, &mut controller);
    let tree_name = if strategy.eq("class") {
        format!("ascendril/base")
    } else {
        format!("ascendril/{}", strategy)
    };
    let tree = get_tree(&tree_name);
    if let Ok(mut tree) = tree.lock() {
        unsafe {
            if DEBUG_TREES {
                if let Some(you) = AetTarget::Target.get_target(&timeline, &controller) {
                    println!("Ascendril: {:?}", you);
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
        if let Some(element) = db.get_hint(&"PRIMARY_ELEMENT".to_string()) {
            controller.hint_plan("PRIMARY_ELEMENT".to_string(), element);
        }
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

pub fn get_class_state(
    timeline: &AetTimeline,
    target: &String,
    strategy: &String,
    db: Option<&impl AetDatabaseModule>,
) -> String {
    let me = timeline.state.borrow_me();
    let you = timeline.state.borrow_agent(target);
    let room = me.room_id;

    let brands = format!(
        "{} {} {}",
        if you.is(FType::Emberbrand) {
            "<red>EMBER"
        } else {
            "<gray>ember"
        },
        if you.is(FType::Frostbrand) {
            "<cyan>FROST"
        } else {
            "<gray>frost"
        },
        if you.is(FType::Thunderbrand) {
            "<yellow>THUNDER"
        } else {
            "<gray>thunder"
        },
    );

    let fulcrum = me
        .check_if_ascendril(&|asc| {
            if asc.fulcrum_expanded(room) {
                "<magenta>FULCRUM*"
            } else if asc.fulcrum_active() {
                "<green>FULCRUM"
            } else {
                "<gray>fulcrum"
            }
        })
        .unwrap_or("<gray>fulcrum");
    let schism = me
        .check_if_ascendril(&|asc| {
            if asc.schism_active(room) {
                "<red>SCH"
            } else {
                "<gray>sch"
            }
        })
        .unwrap_or("<gray>sch");
    let imbalance = me
        .check_if_ascendril(&|asc| {
            if asc.imbalance_active(room) {
                "<yellow>IMB"
            } else {
                "<gray>imb"
            }
        })
        .unwrap_or("<gray>imb");
    let degradation = me
        .check_if_ascendril(&|asc| {
            if asc.degradation_active(room) {
                "<magenta>DEG"
            } else {
                "<gray>deg"
            }
        })
        .unwrap_or("<gray>deg");
    let spiritrift = me
        .check_if_ascendril(&|asc| {
            if asc.spiritrift_active(room) {
                "<blue>RIFT"
            } else {
                "<gray>rift"
            }
        })
        .unwrap_or("<gray>rift");

    let resonance = me
        .check_if_ascendril(&|asc| {
            if asc.resonance_active(&Element::Fire) {
                "<red>RF2"
            } else if asc.half_resonance_active(&Element::Fire) {
                "<red>RF1"
            } else if asc.resonance_active(&Element::Water) {
                "<cyan>RW2"
            } else if asc.half_resonance_active(&Element::Water) {
                "<cyan>RW1"
            } else if asc.resonance_active(&Element::Air) {
                "<yellow>RA2"
            } else if asc.half_resonance_active(&Element::Air) {
                "<yellow>RA1"
            } else {
                "<gray>R0"
            }
        })
        .unwrap_or("<gray>R0");

    let fireburst = me
        .check_if_ascendril(&|asc| asc.fireburst_stacks())
        .map(|stacks| {
            if stacks > 0 {
                format!("<red>FB{}", stacks)
            } else {
                "<gray>fb0".to_string()
            }
        })
        .unwrap_or("<gray>fb0".to_string());
    let afterburn = me
        .check_if_ascendril(&|asc| {
            if asc.afterburn_active() {
                "<red>AB"
            } else if asc.afterburn_coming_up() {
                "<yellow>AB+"
            } else {
                "<gray>ab"
            }
        })
        .unwrap_or("<gray>ab");
    let capacitance = me
        .check_if_ascendril(&|asc| {
            if asc.capacitance_active() && asc.capacitance_will_disrupt() {
                "<red>CAP!"
            } else if asc.capacitance_active() {
                "<green>CAP"
            } else if asc.capacitance_coming_up() {
                "<yellow>CAP+"
            } else {
                "<gray>cap"
            }
        })
        .unwrap_or("<gray>cap");

    let mut situational_effects = Vec::new();
    if you.ascendril_board.sunspot_active() {
        situational_effects.push("<yellow>SUN");
    }
    if you.ascendril_board.shattering_active() {
        situational_effects.push("<red>SHAT");
    } else if you.ascendril_board.icicles_active() {
        situational_effects.push("<cyan>ICE");
    }
    if you.ascendril_board.aeroblast_active() {
        situational_effects.push("<blue>AERO");
    }
    if you.ascendril_board.aeroblast_stun_active() {
        situational_effects.push("<magenta>STUN");
    }

    let phenomenon = if phenomenon_in_room(&timeline.state, room, PhenomenaState::Blazewhirl) {
        "<red>PH:FIRE"
    } else if phenomenon_in_room(&timeline.state, room, PhenomenaState::Glazeflow) {
        "<cyan>PH:WATER"
    } else if phenomenon_in_room(&timeline.state, room, PhenomenaState::Electrosphere) {
        "<yellow>PH:AIR"
    } else {
        "<gray>ph:none"
    };

    let heal_row = heal_pressure_row(timeline);
    if situational_effects.is_empty() {
        format!(
            "{}\n{} {} {} {} {}\n{} {} {} {}\n{}\n{}",
            brands,
            fulcrum,
            schism,
            imbalance,
            degradation,
            spiritrift,
            resonance,
            fireburst,
            afterburn,
            capacitance,
            phenomenon,
            heal_row,
        )
    } else {
        format!(
            "{}\n{} {} {} {} {}\n{} {} {} {}\n{}\n{}\n{}",
            brands,
            fulcrum,
            schism,
            imbalance,
            degradation,
            spiritrift,
            resonance,
            fireburst,
            afterburn,
            capacitance,
            situational_effects.join(" "),
            phenomenon,
            heal_row,
        )
    }
}
