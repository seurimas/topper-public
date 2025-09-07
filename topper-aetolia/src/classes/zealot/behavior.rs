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
    AddComboAttackAtPriority(ZealotComboAction, i32),
    TakeComboAttacks(AetTarget),
    TakeComboAttacksIfOver(AetTarget, i32),
    FullCombo(AetTarget, ZealotComboAction, ZealotComboAction),
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
    // Smart pendulum
    PickBestPendulum(AetTarget, i32),
    HacklesWelt(AetTarget, bool),
}

impl UnpoweredFunction for ZealotBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match &self {
            ZealotBehavior::AddComboAttackAtPriority(action, priority) => {
                if let ClassController::Zealot {
                    combo_attack_priorities,
                } = &mut controller.class_controller
                {
                    combo_attack_priorities.push((*priority, action.clone()));
                    combo_attack_priorities.sort_by(|a, b| b.0.cmp(&a.0));
                    UnpoweredFunctionState::Complete
                } else {
                    println!("Failed to add combo attack: not a zealot");
                    UnpoweredFunctionState::Failed
                }
            }
            ZealotBehavior::TakeComboAttacks(aet_target)
            | ZealotBehavior::TakeComboAttacksIfOver(aet_target, _) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if let ClassController::Zealot {
                    combo_attack_priorities,
                } = &mut controller.class_controller
                {
                    if combo_attack_priorities.len() < 2 {
                        return UnpoweredFunctionState::Failed;
                    }
                    if let ZealotBehavior::TakeComboAttacksIfOver(_, limit) = &self {
                        if combo_attack_priorities[0].0 <= *limit {
                            return UnpoweredFunctionState::Failed;
                        }
                    }
                    let me = model.state.borrow_me();
                    let mut actual_attacks = vec![];
                    for (_, action) in combo_attack_priorities.drain(..) {
                        if !action.check_action(&me, target) {
                            continue;
                        }
                        actual_attacks.push(action);
                        if actual_attacks.len() >= 2 {
                            break;
                        }
                    }
                    if actual_attacks.len() < 2 {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(FlowAttack::new(
                        actual_attacks,
                        aet_target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    println!("Failed to take combo attacks: not a zealot");
                    return UnpoweredFunctionState::Failed;
                }
            }
            ZealotBehavior::FullCombo(aet_target, first, second) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                let me = model.state.borrow_me();
                if !first.check_action(&me, target) || !second.check_action(&me, target) {
                    return UnpoweredFunctionState::Failed;
                }
                controller.plan.add_to_qeb(Box::new(FlowAttack::new(
                    vec![*first, *second],
                    aet_target.get_name(model, controller),
                )));
                UnpoweredFunctionState::Complete
            }
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
            ZealotBehavior::PickBestPendulum(aet_target, min_damage_value) => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::pendulum()) {
                    return UnpoweredFunctionState::Failed;
                }
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                let pendulum_values = get_pendulum_values(&target, false);
                let reverse_pendulum_values = get_pendulum_values(&target, true);
                let pendulum_damage_value = pendulum_values.iter().sum::<i32>();
                let reverse_pendulum_damage_value = reverse_pendulum_values.iter().sum::<i32>();
                if pendulum_damage_value < *min_damage_value
                    && reverse_pendulum_damage_value < *min_damage_value
                {
                    return UnpoweredFunctionState::Failed;
                }
                if pendulum_damage_value >= reverse_pendulum_damage_value {
                    println!("Pendulum values: {:?}", pendulum_values);
                    controller.plan.add_to_qeb(Pendulum::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                } else {
                    println!("Pendulum values: {:?}", reverse_pendulum_values);
                    controller.plan.add_to_qeb(PendulumReverse::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                }
                UnpoweredFunctionState::Complete
            }
            ZealotBehavior::HacklesWelt(aet_target, avoid_restoring) => {
                let me = model.state.borrow_me();
                if me.get_qeb_balance() <= 0. || !me.balanced(BType::Secondary) {
                    return UnpoweredFunctionState::Failed;
                }
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if target.get_limb_state(LType::HeadDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::HeadDamage).is_restoring {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesUprise::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                } else if target.get_limb_state(LType::TorsoDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::TorsoDamage).is_restoring {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesDescent::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                } else if target.get_limb_state(LType::LeftArmDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::LeftArmDamage).is_restoring
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesWristlash::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                } else if target.get_limb_state(LType::RightArmDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::RightArmDamage).is_restoring
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesWristlash::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                } else if target.get_limb_state(LType::LeftLegDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::LeftLegDamage).is_restoring
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesAnklepin::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                } else if target.get_limb_state(LType::RightLegDamage).welt {
                    if *avoid_restoring && target.get_limb_state(LType::RightLegDamage).is_restoring
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(HacklesAnklepin::boxed(
                        model.who_am_i(),
                        aet_target.get_name(model, controller),
                    ));
                    return UnpoweredFunctionState::Complete;
                }
                UnpoweredFunctionState::Failed
            }
        }
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // No internal state to reset
    }
}
