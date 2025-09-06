use behavior_bark::unpowered::{UnpoweredFunction, UnpoweredFunctionState};
use serde::{Deserialize, Serialize};

use crate::{
    agent::*,
    bt::*,
    classes::{
        zealot::{actions::*, constants::*},
        Class,
    },
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DisableTargets {
    Aeon,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ZealotBehavior {
    // Combo actions
    AddZealotComboAttacks(Vec<ComboAttack>),
    // Simple actions
    Wrath,
    Swagger,
    Firefist,
    RespirationHold,
    PsiRecover,
    Zenith,
    Pyromania,
    PsiTorrent,
    Cinderkin(AetTarget),
    Immolation(AetTarget),
    PsiDisable(AetTarget, DisableTargets),
    Dull(AetTarget),
    Scorch(AetTarget),
    Quicken(AetTarget),
    Heatspear(AetTarget),
    Pendulum(AetTarget),
    PendulumReverse(AetTarget),
    Infernal(AetTarget),
    HacklesWhipburst(AetTarget),
    HacklesJawcrack(AetTarget),
    HacklesUprise(AetTarget),
    HacklesWristlash(AetTarget),
    HacklesAnklepin(AetTarget),
    HacklesDescent(AetTarget),
    HacklesTrammel(AetTarget),
}

impl UnpoweredFunction for ZealotBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            ZealotBehavior::Wrath => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::wrath()) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Wrath::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Swagger => {
                let me = model.state.borrow_me();
                let sapped = me.get_count(FType::SappedStrength);
                if me.is(FType::Swagger) {
                    return UnpoweredFunctionState::Failed;
                } else if sapped >= SWAGGER_LIMIT {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Swagger::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Firefist => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::firefist()) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Firefist::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::RespirationHold => {
                controller
                    .plan
                    .add_to_qeb(RespirationHold::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::PsiRecover => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(PsiRecover::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Zenith => {
                let me = model.state.borrow_me();
                if me.is(FType::Zenith) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Zenith::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Pyromania => {
                let me = model.state.borrow_me();
                let ClassState::Zealot(ZealotClassState { pyromania, .. }) = me.class_state else {
                    return UnpoweredFunctionState::Failed;
                };
                if pyromania.active() {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Pyromania::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::PsiTorrent => {
                controller
                    .plan
                    .add_to_qeb(PsiTorrent::boxed(model.who_am_i()));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Cinderkin(aet_target) => {
                controller.plan.add_to_qeb(Cinderkin::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Immolation(aet_target) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if target.get_count(FType::Ablaze) <= 12 || target.is(FType::Shielded) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Immolation::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::PsiDisable(aet_target, disable_target) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !target.is_prone() {
                    return UnpoweredFunctionState::Failed;
                }
                match disable_target {
                    DisableTargets::Aeon => {
                        if target.get_normalized_class() != Some(Class::Indorani) {
                            return UnpoweredFunctionState::Failed;
                        }
                    }
                }
                let me = model.state.borrow_me();
                if me.get_stat(SType::SP) <= 1000 {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(match disable_target {
                    DisableTargets::Aeon => PsiDisableAeon::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ),
                });
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Dull(aet_target) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if target.is(FType::Indifference) {
                    return UnpoweredFunctionState::Failed;
                }
                let me = model.state.borrow_me();
                if me.get_stat(SType::SP) <= 200 {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(PsiDull::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Scorch(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if target.is(FType::Ablaze) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Scorch::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Quicken(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !target.is(FType::Ablaze) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Quicken::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Heatspear(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !target.is(FType::Ablaze) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Heatspear::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Pendulum(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::pendulum()) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Pendulum::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::PendulumReverse(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::pendulum()) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(PendulumReverse::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::Infernal(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !target.get_limb_state(LType::TorsoDamage).broken {
                    return UnpoweredFunctionState::Failed;
                } else if target.is(FType::InfernalSeal) || target.is(FType::InfernalShroud) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Infernal::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesWhipburst(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesWhipburst::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesJawcrack(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesJawcrack::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesUprise(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesUprise::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesWristlash(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesWristlash::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesAnklepin(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesAnklepin::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesDescent(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesDescent::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesTrammel(aet_target) => {
                let me = model.state.borrow_me();
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                if !(target.get_limb_state(LType::LeftArmDamage).mangled
                    && !target.get_limb_state(LType::LeftArmDamage).amputated)
                    && !(target.get_limb_state(LType::RightArmDamage).mangled
                        && !target.get_limb_state(LType::RightArmDamage).amputated)
                {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(HacklesTrammel::boxed(
                    model.who_am_i(),
                    aet_target.get_name(model, controller),
                ));
                UnpoweredFunctionState::Complete
            }
        }
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // No internal state to reset
    }
}
