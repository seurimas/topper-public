use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*,
    classes::get_venoms_from_plan,
    classes::group::*,
    items::{UnwieldAction, WieldAction},
    non_agent::AetTimelineRoomExt,
    observables::PlainAction,
    types::*,
};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PredatorBehavior {
    FastestComboWithAttacks(usize, Stance, Vec<ComboAttack>),
    HighValueComboWithAttacks(usize, Stance, Vec<ComboAttack>),
    AddComboAttacks(Vec<ComboAttack>),
    CalculateCombos,
    ResetComboAttacks,
    Bloodscourge,
    Dartshot,
    Twinshot,
    CirisosisDart,
}

impl UnpoweredFunction for PredatorBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            PredatorBehavior::ResetComboAttacks => {
                controller.predator_combo_attacks.clear();
                controller.predator_combos.clear();
                UnpoweredFunctionState::Complete
            }
            PredatorBehavior::AddComboAttacks(attacks) => {
                controller.predator_combo_attacks.extend(attacks.iter());
                UnpoweredFunctionState::Complete
            }
            PredatorBehavior::FastestComboWithAttacks(minimum, final_stance, attacks) => {
                let best_combo = controller
                    .predator_combos
                    .iter()
                    .filter(|combo| {
                        if *final_stance != Stance::None {
                            combo.get_final_stance() == *final_stance
                        } else {
                            true
                        }
                    })
                    .filter(|combo| {
                        combo.get_attacks().len() >= *minimum
                            && attacks
                                .iter()
                                .all(|attack| combo.get_attacks().contains(attack))
                    })
                    .min_by_key(|combo| combo.get_balance_time() - combo.get_attacks().len() as i32)
                    .cloned();
                unsafe {
                    if DEBUG_TREES {
                        println!("Fastest combo: {:?}", best_combo);
                        println!("All combos: {:?}", controller.predator_combos);
                    }
                }
                use_combo(model, controller, best_combo)
            }
            PredatorBehavior::HighValueComboWithAttacks(minimum, final_stance, attacks) => {
                let best_combo = controller
                    .predator_combos
                    .iter()
                    .filter(|combo| {
                        if *final_stance != Stance::None {
                            combo.get_final_stance() == *final_stance
                        } else {
                            true
                        }
                    })
                    .filter(|combo| {
                        combo.get_attacks().len() >= *minimum
                            && attacks
                                .iter()
                                .all(|attack| combo.get_attacks().contains(attack))
                    })
                    .min_by_key(|combo| (combo.estimate_aff_rate() * BALANCE_SCALE) as i32)
                    .cloned();
                unsafe {
                    if DEBUG_TREES {
                        println!("Value combo: {:?}", best_combo);
                        println!("All combos: {:?}", controller.predator_combos);
                    }
                }
                use_combo(model, controller, best_combo)
            }
            PredatorBehavior::CalculateCombos => {
                if let (me, Some(target)) = (
                    model.state.borrow_me(),
                    AetTarget::Target.get_target(model, controller),
                ) {
                    controller.predator_combos = find_combos(
                        me.get_predator_stance(),
                        &controller.predator_combo_attacks,
                        target.can_parry(),
                        target.is_prone(),
                        (if target.will_be_rebounding(me.get_qeb_balance()) {
                            1
                        } else {
                            0
                        }) + (if target.is(FType::Shielded) { 1 } else { 0 }),
                    );
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Bloodscourge => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    let venom = controller.get_venoms_from_plan(1, you);
                    controller.plan.add_to_qeb(Box::new(BloodscourgeAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                        if venom.is_empty() { "" } else { venom[0] },
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Dartshot => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    let me = model.state.borrow_me();
                    if me.check_if_predator(&|me| me.apex < 3).unwrap_or(true)
                        && you.will_be_rebounding(me.get_qeb_balance())
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venom = controller.get_venoms_from_plan(1, you);
                    controller.plan.add_to_qeb(Box::new(DartshotAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                        if venom.is_empty() { "" } else { venom[0] },
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Twinshot => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.will_be_rebounding(model.state.borrow_me().get_qeb_balance()) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = controller.get_venoms_from_plan(2, you);
                    if let (Some(venom_0), Some(venom_1)) = (venoms.get(0), venoms.get(1)) {
                        controller.plan.add_to_qeb(Box::new(TwinshotAction::new(
                            controller.target.clone().unwrap_or("".to_string()),
                            venom_0,
                            venom_1,
                        )));
                        UnpoweredFunctionState::Complete
                    } else {
                        UnpoweredFunctionState::Failed
                    }
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::CirisosisDart => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    let me = model.state.borrow_me();
                    if me.check_if_predator(&|me| me.apex >= 3).unwrap_or(true)
                        && you.will_be_rebounding(me.get_qeb_balance())
                    {
                        controller.plan.add_to_qeb(Box::new(DartshotAction::new(
                            controller.target.clone().unwrap_or("".to_string()),
                            "cirisosis",
                        )));
                        return UnpoweredFunctionState::Complete;
                    } else if you.will_be_rebounding(me.get_qeb_balance()) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venom = controller.get_venoms_from_plan(1, you);
                    controller.plan.add_to_qeb(Box::new(TwinshotAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                        "cirisosis",
                        if venom.is_empty() { "" } else { venom[0] },
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to reset.
    }
}

fn use_combo(
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    best_combo: Option<PredatorCombo>,
) -> UnpoweredFunctionState {
    if let (Some(you), Some(combo)) = (AetTarget::Target.get_target(model, controller), &best_combo)
    {
        let venom = controller.get_venoms_from_plan(1, you);
        controller
            .plan
            .add_to_qeb(Box::new(SeriesAttack::new_random_params(
                combo.get_attacks().to_vec(),
                controller.target.clone().unwrap_or("".to_string()),
                if venom.is_empty() { "" } else { venom[0] },
            )));
        UnpoweredFunctionState::Complete
    } else if let Some(combo) = &best_combo {
        // When the model is uninitialized, we still want to start up a fight.
        controller
            .plan
            .add_to_qeb(Box::new(SeriesAttack::new_random_params(
                combo.get_attacks().to_vec(),
                controller.target.clone().unwrap_or("".to_string()),
                "curare",
            )));
        UnpoweredFunctionState::Complete
    } else {
        UnpoweredFunctionState::Failed
    }
}
