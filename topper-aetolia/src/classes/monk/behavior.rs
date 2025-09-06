use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    bt::*, classes::group::*, non_agent::AetTimelineRoomExt, observables::PlainAction, types::*,
};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MonkBehavior {
    // Class cure.
    Push(AetTarget),
    // Combo attacks
    AddMonkComboAttacks(Vec<MonkComboAttack>),
    Combo(AetTarget, Vec<MonkComboGrader>, Option<CType>),
    // Non-combo Tekura actions
    Backbreaker(AetTarget),
    // Kaido attacks
    Choke(AetTarget),
    Cripple(AetTarget),
    Strike(AetTarget),
    Enfeeble(AetTarget),
    // Telepathy actions
    MindLock(AetTarget),
    // Telepathy attacks
    Fear(AetTarget),
    Paralyse(AetTarget),
    Confuse(AetTarget),
    Recklessness(AetTarget),
    Epilepsy(AetTarget),
    Pacify(AetTarget),
    Stupidity(AetTarget),
    Anorexia(AetTarget),
    Amnesia(AetTarget),
    Deadening(AetTarget),
    Strip(AetTarget),
    Crush(AetTarget),
    Batter(AetTarget),
}

impl UnpoweredFunction for MonkBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            MonkBehavior::MindLock(target) => {
                let target_name = target.get_name(model, controller);
                let me = model.state.borrow_me();
                if me
                    .check_if_monk(&move |monk| monk.has_lock(&target_name))
                    .unwrap_or(false)
                {
                    UnpoweredFunctionState::Failed
                } else {
                    controller
                        .plan
                        .add_to_qeb(MindLock::from_target_boxed(target, model, controller));
                    UnpoweredFunctionState::Complete
                }
            }
            MonkBehavior::Push(target) => {
                let me = model.state.borrow_me();
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(MindPush::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Backbreaker(target) => {
                controller
                    .plan
                    .add_to_qeb(Backbreaker::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Choke(target) => {
                let me = model.state.borrow_me();
                if !me.check_if_monk(&|monk| monk.has_kai(20)).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(KaiChoke::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Cripple(target) => {
                let me = model.state.borrow_me();
                if !me.check_if_monk(&|monk| monk.has_kai(40)).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(KaiCripple::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Strike(target) => {
                let me = model.state.borrow_me();
                if !me.check_if_monk(&|monk| monk.has_kai(20)).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(KaiStrike::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Enfeeble(target) => {
                let me = model.state.borrow_me();
                if !me.check_if_monk(&|monk| monk.has_kai(70)).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(KaiEnfeeble::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Fear(target) => {
                controller
                    .plan
                    .add_to_qeb(MindFear::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Paralyse(target) => {
                controller
                    .plan
                    .add_to_qeb(MindParalyse::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Confuse(target) => {
                controller
                    .plan
                    .add_to_qeb(MindConfuse::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Recklessness(target) => {
                controller
                    .plan
                    .add_to_qeb(MindRecklessness::from_target_boxed(
                        target, model, controller,
                    ));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Epilepsy(target) => {
                controller
                    .plan
                    .add_to_qeb(MindEpilepsy::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Pacify(target) => {
                controller
                    .plan
                    .add_to_qeb(MindPacify::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Stupidity(target) => {
                controller
                    .plan
                    .add_to_qeb(MindStupidity::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Anorexia(target) => {
                controller
                    .plan
                    .add_to_qeb(MindAnorexia::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Amnesia(target) => {
                controller
                    .plan
                    .add_to_qeb(MindAmnesia::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Deadening(target) => {
                controller
                    .plan
                    .add_to_qeb(MindDeadening::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Strip(target) => {
                controller
                    .plan
                    .add_to_qeb(MindStrip::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Crush(target) => {
                controller
                    .plan
                    .add_to_qeb(MindCrush::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Batter(target) => {
                controller
                    .plan
                    .add_to_qeb(MindBatter::from_target_boxed(target, model, controller));
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::AddMonkComboAttacks(attacks) => {
                controller
                    .monk_combo_generator()
                    .add_attacks(attacks.iter());
                UnpoweredFunctionState::Complete
            }
            MonkBehavior::Combo(target, graders, kk) => {
                if let Some(target) = target.get_target(model, controller) {
                    let me = model.state.borrow_me();

                    // controller.plan.add_to_qeb(MonkCombo::from_target_boxed(
                    //     target, graders, kk, model, controller,
                    // ));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}
