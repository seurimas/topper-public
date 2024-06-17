use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*,
    classes::{get_venoms_from_plan, group::*, ActiveTransition, Contemplate},
    curatives::get_cure_depth,
    items::{UnwieldAction, WieldAction},
    non_agent::AetTimelineRoomExt,
    observables::PlainAction,
    types::*,
};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AscendrilBehavior {
    // Fire spells
    Spark(AetTarget),
    AshenFeet(AetTarget),
    FireburstCast,
    Fireburst(AetTarget, bool),
    Blazewhirl(AetTarget),
    Conflagrate(AetTarget),
    Afterburn,
    Sunspot(AetTarget),
    Pyroclast(AetTarget, bool),
    Disintegrate(AetTarget, bool),
    // Water spells
    Coldsnap,
    Drench(AetTarget, bool),
    Iceray(AetTarget),
    Glazeflow(AetTarget),
    // Direfrost(AetTarget),
    Icicle(AetTarget),
    Shatter(AetTarget),
    Crystalise(AetTarget, bool, bool),
    Winterheart(bool),
    // Air spells
    Windlance(AetTarget),
    Pressurize(AetTarget),
    Arcbolt(AetTarget),
    Electrosphere(AetTarget),
    Thunderclap(AetTarget),
    Feedback(AetTarget),
    Aeroblast(AetTarget, bool),
    Stormwrath(bool),
    // Thaumaturgy spells
    Fulcrum,
    FulcrumExpand,
    Schism,
    Imbalance,
    Restore,
    EnrichFire,
    EnrichWater,
    EnrichAir,
    GlimpseFire,
    GlimpseWater,
    GlimpseAir,
    Flare(AetTarget, bool),
    Emberbrand(AetTarget, bool),
    Frostbrand(AetTarget, bool),
    Thunderbrand(AetTarget, bool),
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
                let action = Fireburst::from_target(target, model, controller);
                if *plain {
                    controller.plan.add_to_plain(Box::new(action));
                    return UnpoweredFunctionState::Complete;
                }
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Blazewhirl(target) => {
                let action = Blazewhirl::from_target(target, model, controller);
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
            AscendrilBehavior::Glazeflow(target) => {
                let action = Glazeflow::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            // AscendrilBehavior::Direfrost(target) => {
            //     let action = Direfrost::from_target(target, model, controller);
            //     controller.plan.add_to_qeb(Box::new(action));
            //     UnpoweredFunctionState::Complete
            // }
            AscendrilBehavior::Drench(target, contemplate) => {
                let action = Drench::from_target(target, model, controller);
                if *contemplate {
                    controller
                        .plan
                        .add_to_plain(Box::new(Contemplate::from_target(
                            target, model, controller,
                        )));
                }
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Icicle(target) => {
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
            AscendrilBehavior::Winterheart(allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Water,
                    *allow_enrich,
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
            AscendrilBehavior::Electrosphere(target) => {
                let action = Electrosphere::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Thunderclap(target) => {
                let action = Thunderclap::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Feedback(target) => {
                let action = Feedback::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Aeroblast(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Aeroblast::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Stormwrath(allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Stormwrath::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Fulcrum => {
                let action = Fulcrum::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::FulcrumExpand => {
                let action = FulcrumExpand::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Schism => {
                let action = Schism::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Imbalance => {
                let action = Imbalance::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Restore => {
                let action = Restore::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::EnrichFire => {
                let me = model.state.borrow_me();
                if me
                    .check_if_ascendril(&|me| me.can_enrich(&Element::Fire))
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
                    .check_if_ascendril(&|me| me.can_enrich(&Element::Water))
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
                    .check_if_ascendril(&|me| me.can_enrich(&Element::Air))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = EnrichAir::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::GlimpseFire => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|me| me.is_glimpse_active(None))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = GlimpseFire::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::GlimpseWater => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|me| me.is_glimpse_active(None))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = GlimpseWater::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::GlimpseAir => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                } else if me
                    .check_if_ascendril(&|me| me.is_glimpse_active(None))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = GlimpseAir::new(model.who_am_i());
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Flare(target, plain) => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                } else if !me
                    .check_if_ascendril(&|me| me.is_glimpse_active(None))
                    .unwrap_or(false)
                {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Flare::from_target(target, model, controller);
                if *plain {
                    controller.plan.add_to_plain(Box::new(action));
                    return UnpoweredFunctionState::Complete;
                }
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Emberbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Fire,
                    *allow_enrich,
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Emberbrand::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Frostbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Water,
                    *allow_enrich,
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Frostbrand::from_target(target, model, controller);
                controller.plan.add_to_qeb(Box::new(action));
                UnpoweredFunctionState::Complete
            }
            AscendrilBehavior::Thunderbrand(target, allow_enrich) => {
                let me = model.state.borrow_me();
                if !resonating_or_enrich(
                    &me,
                    model.who_am_i(),
                    controller,
                    Element::Air,
                    *allow_enrich,
                ) {
                    return UnpoweredFunctionState::Failed;
                }
                let action = Thunderbrand::from_target(target, model, controller);
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
    allow_enrich: bool,
) -> bool {
    if me
        .check_if_ascendril(&|me| me.resonance_active(&element))
        .unwrap_or(false)
    {
        return true;
    } else if allow_enrich
        && me
            .check_if_ascendril(&|me| me.can_enrich(&element))
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
    }
    false
}
