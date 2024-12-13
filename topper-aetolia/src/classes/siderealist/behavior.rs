use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    bt::*,
    classes::{get_venoms_from_plan, group::*, VenomType, AFFLICT_VENOMS, VENOM_AFFLICTS},
    items::{UnwieldAction, WieldAction},
    non_agent::AetTimelineRoomExt,
    observables::PlainAction,
    types::*,
};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SiderealistBehavior {
    Embed(Vibration),
    Embeds(Vec<Vibration>),
    Tones(AetTarget, Vibration),
    Ray(AetTarget),
    Erode(AetTarget),
    Rotate,
    Enigma,
    EnigmaAttack(AetTarget),
    Embody,
    EmbodyAttack(AetTarget),
    Dustring(AetTarget),
    Asterism(AetTarget),
    Moonlet(AetTarget),
    Gleam,
    GleamInflict(AetTarget, GleamColor),
    Eventide,
    Foresight,
    Centrum,
    Equinox,
    Stillness(AetTarget),
    Parallax(AetTarget, i32, String),
    ParallaxWithAb(AetTarget, i32, String, String),
    Alteration(AetTarget, FType, FType),
    Redshift(AetTarget),
    Chromaflare(AetTarget),
    Syzygy(AetTarget),
    Reenact(Regalia, Vec<Regalia>),
    Illgrasp(AetTarget),
    Bolt(AetTarget, LimbDescriptor),
    VayuaAttack(AetTarget),
    EjaKodosaMend,
    EjaKodosaKill(AetTarget),
}

impl UnpoweredFunction for SiderealistBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        let me = model.state.borrow_me();
        match self {
            SiderealistBehavior::Embed(vibration) => {
                if vibration_in_room(&model.state, me.room_id, *vibration) {
                    return UnpoweredFunctionState::Failed;
                }
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Embed::new(model.who_am_i(), *vibration)));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Embeds(vibrations) => {
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                for vibration in vibrations {
                    if !vibration_in_room(&model.state, me.room_id, *vibration)
                        && !me
                            .check_if_siderealist(&|me| me.has_vibration(*vibration))
                            .unwrap_or(false)
                    {
                        controller
                            .plan
                            .add_to_qeb(Box::new(Embed::new(model.who_am_i(), *vibration)));
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
            SiderealistBehavior::Tones(target, vibration) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !vibration_in_room(&model.state, me.room_id, *vibration) {
                        println!("No {} in room!", *vibration);
                        return UnpoweredFunctionState::Failed;
                    }
                    if !me.balanced(BType::Secondary) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if *vibration == Vibration::Crystalforest
                        && me
                            .check_if_siderealist(&|me| me.can_crystalforest())
                            .unwrap_or(false)
                    {
                        controller.stripping_shield = true;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(Tones::from_target(
                        *target, model, controller, *vibration,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Ray(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if target_agent.is(FType::Shielded) && controller.stripping_shield {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Ray::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Erode(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if controller.stripping_shield {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Erode::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Rotate => {
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Rotate::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Enigma => {
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Enigma::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::EnigmaAttack(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if me.is(FType::Disfigurement) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(EnigmaAttack::from_target(
                            target, model, controller,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Embody => {
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Embody::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::EmbodyAttack(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if me.is(FType::Disfigurement) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(EmbodyAttack::from_target(
                            target, model, controller,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Dustring(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if (target_agent.is(FType::Shielded) && !controller.stripping_shield) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Dustring::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Asterism(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if (target_agent.is(FType::Shielded) && !controller.stripping_shield) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Asterism::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Moonlet(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if (target_agent.is(FType::Shielded) && !controller.stripping_shield) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Moonlet::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Gleam => {
                if me
                    .check_if_siderealist(&|me| me.has_gleam() || !me.can_gleam())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if !centrum_check(model, controller) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Gleam::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::GleamInflict(target, color) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_gleam_star(*color))
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if target_agent.is(FType::Shielded) && !controller.stripping_shield {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(GleamInflict::from_target(
                            *target, model, controller, *color,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Eventide => {
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Eventide::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Equinox => {
                if !me.balanced(BType::ClassCure2) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Equinox::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Foresight => {
                if me.is(FType::Foresight)
                    || me
                        .check_if_siderealist(&|me| !me.can_foresight())
                        .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Foresight::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Centrum => {
                if me.is(FType::Centrum)
                    || !me
                        .check_if_siderealist(&|me| me.can_centrum())
                        .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Centrum::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Stillness(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !target_agent.is(FType::Echoes)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Stillness::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Parallax(target, time, spell) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if me
                        .check_if_siderealist(&|me| me.has_parallax())
                        .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Parallax::new_with_target(
                            model.who_am_i(),
                            *time,
                            spell.clone(),
                            target.get_name(model, controller),
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::ParallaxWithAb(target, time, spell, ab) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if me
                        .check_if_siderealist(&|me| me.has_parallax())
                        .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Parallax::new_with_target_and_ab(
                            model.who_am_i(),
                            *time,
                            spell.clone(),
                            target.get_name(model, controller),
                            ab.clone(),
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Alteration(target, source, result) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me.balanced(BType::Secondary) || !target_agent.is(*source) {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(Alteration::from_target(
                        *target, model, controller, *source, *result,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Redshift(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Redshift::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Chromaflare(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_gleam())
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Chromaflare::from_target(
                            target, model, controller,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Syzygy(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !target_agent.siderealist_board.has_asterism()
                        || !target_agent.siderealist_board.has_moonlet()
                        || !target_agent.siderealist_board.has_dustring()
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !centrum_check(model, controller) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Syzygy::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SiderealistBehavior::Reenact(regalia, regalias) => {
                if me
                    .check_if_siderealist(&|me| me.has_regalia(*regalia))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if me
                    .check_if_siderealist(&|me| me.has_two_regalias())
                    .unwrap_or(false)
                {
                    if let Some(regalia) = me
                        .check_if_siderealist(&|me| me.first_regalia_of(regalias))
                        .flatten()
                    {
                        controller
                            .plan
                            .add_to_qeb(Box::new(Forfeit::new(model.who_am_i(), regalia)));
                    } else {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Reenactment::new(model.who_am_i(), *regalia)));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::Illgrasp(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_regalia(Regalia::Ontesme))
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(Illgrasp::from_target(target, model, controller)));
                    UnpoweredFunctionState::Complete
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            SiderealistBehavior::Bolt(target, limb) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_regalia(Regalia::Averroes))
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !regalia_uncrippled(&me, model, controller, "staff") {
                        return UnpoweredFunctionState::Failed;
                    }
                    let limb = limb
                        .get_limb(model, controller, target)
                        .unwrap_or(LType::HeadDamage);
                    controller.plan.add_to_qeb(Box::new(Bolt::from_target(
                        *target, model, controller, limb,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
            SiderealistBehavior::VayuaAttack(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_regalia(Regalia::Vayua))
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !regalia_uncrippled(&me, model, controller, "sword") {
                        return UnpoweredFunctionState::Failed;
                    }
                    if let Some(venom) = controller.aff_priorities.as_ref().and_then(|affs| {
                        get_venoms_from_plan(&affs, 1, target_agent)
                            .first()
                            .cloned()
                    }) {
                        controller
                            .plan
                            .add_to_qeb(Box::new(VayuaAttack::from_target(
                                *target,
                                model,
                                controller,
                                venom.to_string(),
                            )));
                        return UnpoweredFunctionState::Complete;
                    }
                }
                return UnpoweredFunctionState::Failed;
            }
            SiderealistBehavior::EjaKodosaMend => {
                if !me
                    .check_if_siderealist(&|me| me.has_regalia(Regalia::EjaKodosa) && me.can_mend())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(EjaKodosaMend::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SiderealistBehavior::EjaKodosaKill(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_siderealist(&|me| me.has_regalia(Regalia::EjaKodosa))
                        .unwrap_or(false)
                        || (target_agent.is(FType::Shielded) && !controller.stripping_shield)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller
                        .plan
                        .add_to_qeb(Box::new(EjaKodosaKill::from_target(
                            target, model, controller,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    return UnpoweredFunctionState::Failed;
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            _ => (),
        }
    }
}

fn centrum_check(model: &BehaviorModel, controller: &mut BehaviorController) -> bool {
    let me = model.state.borrow_me();
    if me.arms_free() {
        return true; // Arms are free
    } else if me.arm_free() && me.is(FType::Centrum) {
        return true; // Centrum is active
    } else if me.arm_free()
        && me
            .check_if_siderealist(&|me| me.can_centrum())
            .unwrap_or(false)
    {
        controller
            .plan
            .add_to_front_of_qeb(Box::new(Centrum::new(model.who_am_i())));
        return true; // Centrum is not active, but can be activated
    } else {
        return false; // Centrum is not active and cannot be activated
    }
}

fn regalia_uncrippled(
    me: &AgentState,
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    wielded: &str,
) -> bool {
    if me.wield_state.is_wielding_left(wielded) && !me.arm_free_left() {
        return false;
    } else if me.wield_state.is_wielding_right(wielded) && !me.arm_free_right() {
        return false;
    }
    true
}
