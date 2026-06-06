use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    bt::*,
    classes::{ActiveTransition, Contemplate, get_venoms_from_plan, group::*},
    curatives::get_cure_depth,
    items::{UnwieldAction, WieldAction},
    non_agent::AetTimelineRoomExt,
    observables::PlainAction,
    types::*,
};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum EnrichOption {
    Resonating,
    Enrich,
    Catalyze,
    EnrichOrCatalyze,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AscendrilBehavior {
    // Fire spells
    Spark(AetTarget),
    AshenFeet(AetTarget),
    FireburstCast,
    Fireburst(AetTarget, bool),
    Conflagrate(AetTarget),
    Afterburn,
    Sunspot(AetTarget),
    Pyroclast(AetTarget, EnrichOption),
    Disintegrate(AetTarget, EnrichOption),
    // Water spells
    Coldsnap,
    Drench(AetTarget, bool),
    Iceray(AetTarget),
    Direfrost(AetTarget),
    Icicle(AetTarget),
    Shatter(AetTarget),
    Crystalise(AetTarget, EnrichOption, bool),
    Winterheart(AetTarget, EnrichOption),
    // Air spells
    Windlance(AetTarget),
    Pressurize(AetTarget),
    Arcbolt(AetTarget),
    Thunderclap(AetTarget),
    Feedback(AetTarget),
    AeroblastFast(AetTarget, EnrichOption),
    AeroblastSlow(AetTarget, EnrichOption),
    Stormwrath(AetTarget, EnrichOption),
    Capacitance,
    // Thaumaturgy spells
    Fulcrum,
    FulcrumExpand,
    FulcrumCallback,
    FulcrumInterfuse,
    FulcrumPush,
    Schism,
    Imbalance,
    Restore,
    EnrichFire,
    EnrichWater,
    EnrichAir,
    Emberbrand(AetTarget, bool),
    Frostbrand(AetTarget, bool),
    Thunderbrand(AetTarget, bool),
    CatalystEmber(AetTarget),
    CatalystFrost(AetTarget),
    CatalystThunder(AetTarget),
    Enrapture(AetTarget),
    FulcrumDetect(AetTarget),
    Shift,
    Degradation,
    Spiritrift,
    PhenomenonAt(PhenomenaKind, AetTarget),
    TraceGlyph(Glyph),
    TraceGlyphAegis(Glyph, String),
    TraceTargettedGlyph(Glyph, AetTarget),
}

impl UnpoweredFunction for AscendrilBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            AscendrilBehavior::Spark(target) => {
                let action = Spark::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::AshenFeet(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if target.is(FType::AshenFeet) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = AshenFeet::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FireburstCast => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|me| me.fireburst_stacks() > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FireburstCast::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Fireburst(target, plain) => {
                let me = model.state.borrow_me();
                if !me
                    .check_if_ascendril(&|me| me.fireburst_stacks() > 0)
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if me.get_balance(BType::LeftHandBalance) >= QUEUE_TIME
                    && me.get_balance(BType::RightHandBalance) >= QUEUE_TIME
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Fireburst::from_target(target, model, controller);
                if *plain {
                    controller.plan.add_to_plain(Box::new(action));
                    return UnpoweredFunctionState::Complete;
                }
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Conflagrate(target) => {
                let action = Conflagrate::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Afterburn => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|me| me.afterburn_active() || me.afterburn_coming_up())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Afterburn::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Sunspot(target) => {
                let action = Sunspot::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Pyroclast(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Fire,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Pyroclast::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Disintegrate(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Fire,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Disintegrate::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Coldsnap => {
                let action = Coldsnap::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Iceray(target) => {
                let action = Iceray::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Direfrost(target) => {
                let action = Direfrost::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Drench(target, contemplate) => {
                let action = Drench::from_target(target, model, controller);
                if *contemplate {
                    controller
                        .plan
                        .add_to_front_of_qeb(Box::new(Contemplate::from_target(
                            target, model, controller,
                        )));
                }
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Icicle(target) => {
                let me = model.state.borrow_me();
                if me.ascendril_board.icicles_active() {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Icicle::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Shatter(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if !target.ascendril_board.icicles_active()
                        || target.ascendril_board.shattering_active()
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = Shatter::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Crystalise(target, allow_enrich, contemplate) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Water,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                if *contemplate {
                    controller
                        .plan
                        .add_to_front_of_qeb(Box::new(Contemplate::from_target(
                            target, model, controller,
                        )));
                }
                let action = Crystalise::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Winterheart(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Water,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Winterheart::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Windlance(target) => {
                let action = Windlance::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Pressurize(target) => {
                let action = Pressurize::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Arcbolt(target) => {
                let action = Arcbolt::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Thunderclap(target) => {
                let action = Thunderclap::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Feedback(target) => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| !asc.feedback_available())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Feedback::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::AeroblastFast(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = AeroblastFast::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::AeroblastSlow(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = AeroblastSlow::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Stormwrath(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Stormwrath::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Capacitance => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|me| me.capacitance_active() || me.capacitance_coming_up())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Capacitance::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Fulcrum => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| asc.fulcrum_active())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Fulcrum::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumExpand => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| !asc.fulcrum_interfused())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumExpand::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumCallback => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| {
                        asc.fulcrum_expanded(me.room_id) || asc.fulcrum_interfused()
                    })
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumCallback::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumInterfuse => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| !asc.fulcrum_expanded(me.room_id))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumInterfuse::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumPush => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| !asc.fulcrum_interfused())
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumPush::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Schism => {
                let me = model.state.borrow_me();
                if !me
                    .check_if_ascendril(&|asc| asc.fulcrum_active_here(me.room_id))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|asc| asc.schism_active(Some(me.room_id)))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Schism::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Imbalance => {
                let me = model.state.borrow_me();
                if !me
                    .check_if_ascendril(&|asc| asc.fulcrum_active_here(me.room_id))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|asc| asc.imbalance_active(Some(me.room_id)))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Imbalance::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Restore => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                } else if !me
                    .check_if_ascendril(&|asc| asc.fulcrum_active_here(me.room_id))
                    .unwrap_or(false)
                {
                    println!("Restore failed because fulcrum not active");
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumRestore::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::EnrichFire => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| asc.can_enrich(me.room_id, &Element::Fire))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = EnrichFire::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::EnrichWater => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| asc.can_enrich(me.room_id, &Element::Water))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = EnrichWater::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::EnrichAir => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| asc.can_enrich(me.room_id, &Element::Air))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = EnrichAir::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Emberbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if let Some(target) = target.get_target(model, controller) {
                    if target.is(FType::Emberbrand) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Fire,
                    if *allow_enrich {
                        EnrichOption::Enrich
                    } else {
                        EnrichOption::Resonating
                    },
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Emberbrand::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Frostbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if let Some(target) = target.get_target(model, controller) {
                    if target.is(FType::Frostbrand) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Water,
                    if *allow_enrich {
                        EnrichOption::Enrich
                    } else {
                        EnrichOption::Resonating
                    },
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Frostbrand::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Thunderbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if let Some(target) = target.get_target(model, controller) {
                    if target.is(FType::Thunderbrand) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    if *allow_enrich {
                        EnrichOption::Enrich
                    } else {
                        EnrichOption::Resonating
                    },
                    target.get_target(model, controller),
                    target.get_name(model, controller),
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Thunderbrand::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::CatalystEmber(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if !target.is(FType::Emberbrand) || !target.is(FType::Etherflux) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = CatalystEmber::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::CatalystFrost(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if !target.is(FType::Frostbrand) || !target.is(FType::Etherflux) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = CatalystFrost::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::CatalystThunder(target) => {
                if let Some(target) = target.get_target(model, controller) {
                    if !target.is(FType::Thunderbrand) || !target.is(FType::Etherflux) {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = CatalystThunder::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Enrapture(target) => {
                let me = model.state.borrow_me();
                let room = me.room_id;
                if !me
                    .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                if let Some(target) = target.get_target(model, controller) {
                    if !target.is(FType::Fallen)
                        && !me
                            .check_if_ascendril(&|me| me.enrapture_accelerated(target))
                            .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                }
                let action = Enrapture::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumDetect(target) => {
                let me = model.state.borrow_me();
                let room = me.room_id;
                if !me
                    .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if !(me.get_mana() > 1000) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = FulcrumDetect::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Shift => {
                let me = model.state.borrow_me();
                if !me.check_if_ascendril(&|me| me.can_shift()).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                let room = me.room_id;
                if !me
                    .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Shift::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Degradation => {
                let me = model.state.borrow_me();
                let room = me.room_id;
                if !me
                    .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|asc| asc.degradation_active(Some(me.room_id)))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Degradation::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Spiritrift => {
                let me = model.state.borrow_me();
                let room = me.room_id;
                if !me
                    .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|asc| asc.spiritrift_active(Some(me.room_id)))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Spiritrift::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::PhenomenonAt(phenomenon, target) => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|asc| asc.phenomenon_active(None))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                match phenomenon {
                    PhenomenaKind::Blazewhirl => controller
                        .plan
                        .add_to_qeb(Box::new(Blazewhirl::from_target(target, model, controller))),
                    PhenomenaKind::Glazeflow => controller
                        .plan
                        .add_to_qeb(Box::new(Glazeflow::from_target(target, model, controller))),
                    PhenomenaKind::Electrosphere => {
                        controller
                            .plan
                            .add_to_qeb(Box::new(Electrosphere::from_target(
                                target, model, controller,
                            )))
                    }
                }
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::TraceGlyph(glyph) => {
                let me = model.state.borrow_me();
                if !model.state.has_room_for_more_glyphs(me.room_id)
                    || model.state.has_glyph_in_room(me.room_id, *glyph)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = GlyphTraceAction::new(model.who_am_i(), *glyph);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::TraceGlyphAegis(glyph, aegis_name) => {
                let me = model.state.borrow_me();
                // TODO Aegis handling
                // if false {
                //     return UnpoweredFunctionState::Failed;
                // }
                let action = GlyphTraceAction::new(model.who_am_i(), *glyph).with_aegis();
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::TraceTargettedGlyph(glyph, aet_target) => {
                let me = model.state.borrow_me();
                if !model.state.has_room_for_more_glyphs(me.room_id)
                    || model.state.has_glyph_in_room(me.room_id, *glyph)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = GlyphTraceAction::new(model.who_am_i(), *glyph)
                    .with_target(aet_target.get_name(model, controller));
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}

pub fn resonating_or_enrich(
    me: &AgentState,
    my_name: String,
    controller: &mut BehaviorController,
    element: Element,
    allow_enrich: EnrichOption,
    target: Option<&AgentState>,
    target_name: String,
) -> bool {
    if me
        .check_if_ascendril(&|me| me.resonance_active(&element))
        .unwrap_or(false)
    {
        return true;
    } else if (allow_enrich == EnrichOption::Enrich
        || allow_enrich == EnrichOption::EnrichOrCatalyze)
        && me
            .check_if_ascendril(&|asc| asc.can_enrich(me.room_id, &element))
            .unwrap_or(false)
    {
        let action: Box<dyn ActiveTransition> = match element {
            Element::Fire => Box::new(EnrichFire::new(my_name)),
            Element::Water => Box::new(EnrichWater::new(my_name)),
            Element::Air => Box::new(EnrichAir::new(my_name)),
            Element::Spirit => panic!("Spirit cannot be enriched"),
        };
        controller.plan.add_to_qeb(action);
        return true;
    } else if (allow_enrich == EnrichOption::Catalyze
        || allow_enrich == EnrichOption::EnrichOrCatalyze)
        && me
            .check_if_ascendril(&|asc| {
                target.map_or(false, |t| asc.can_catalyze(me.room_id, t, &element))
            })
            .unwrap_or(false)
    {
        let action: Box<dyn ActiveTransition> = match element {
            Element::Fire => Box::new(CatalystEmber::new(my_name, target_name)),
            Element::Water => Box::new(CatalystFrost::new(my_name, target_name)),
            Element::Air => Box::new(CatalystThunder::new(my_name, target_name)),
            Element::Spirit => panic!("Spirit cannot be catalyzed"),
        };
        controller.plan.add_to_qeb(action);
        return true;
    }
    false
}
