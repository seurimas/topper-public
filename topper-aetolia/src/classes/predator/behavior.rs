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
    // Combo attacks
    FastestCombo(Vec<ComboPredicate>, Vec<LType>),
    AffRateCombo(Vec<ComboPredicate>, Vec<LType>),
    GradedCombo(Vec<ComboPredicate>, Vec<ComboGrader>, Vec<LType>),
    AddComboAttacks(Vec<ComboAttack>),
    CalculateCombos,
    ResetComboAttacks,
    // Special knifeplay attacks
    Fleshbane,
    Bloodscourge,
    // Darts
    Dartshot,
    Twinshot,
    CirisosisDart,
    // Spider
    Acid,
    Intoxicate,
    // Orgyuk
    Rake,
    Swipe,
    Throw,
    Roar,
    Weaken,
    Pummel(LType),
    Mawcrush,
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
                controller.predator_combo_store = Default::default();
                controller.predator_combos.clear();
                UnpoweredFunctionState::Complete
            }
            PredatorBehavior::AddComboAttacks(attacks) => {
                controller.predator_combo_store.add_attacks(attacks.iter());
                UnpoweredFunctionState::Complete
            }
            PredatorBehavior::FastestCombo(predicates, preferred_limbs) => {
                let best_combo = controller.predator_combos.get_fastest_combo(&predicates);
                unsafe {
                    if DEBUG_TREES {
                        println!("Solver: {:?}", controller.predator_combo_store);
                        println!("Fastest combo: {:?}", best_combo);
                        println!("All combos: {:?}", controller.predator_combos);
                    }
                }
                use_combo(model, controller, best_combo, preferred_limbs)
            }
            PredatorBehavior::AffRateCombo(predicates, preferred_limbs) => {
                let best_combo = controller
                    .predator_combos
                    .get_highest_aff_rate_combo(&predicates);
                unsafe {
                    if DEBUG_TREES {
                        println!("Solver: {:?}", controller.predator_combo_store);
                        println!("Value combo: {:?}", best_combo);
                        println!("All combos: {:?}", controller.predator_combos);
                    }
                }
                use_combo(model, controller, best_combo, preferred_limbs)
            }
            PredatorBehavior::GradedCombo(predicates, graders, preferred_limbs) => {
                let best_combo = controller
                    .predator_combos
                    .get_highest_scored_combo(&predicates, &graders);
                unsafe {
                    if DEBUG_TREES {
                        println!("Solver: {:?}", controller.predator_combo_store);
                        println!("Value combo: {:?}", best_combo);
                        println!("All combos: {:?}", controller.predator_combos);
                    }
                }
                use_combo(model, controller, best_combo, preferred_limbs)
            }
            PredatorBehavior::CalculateCombos => {
                if let (me, Some(target)) = (
                    model.state.borrow_me(),
                    AetTarget::Target.get_target(model, controller),
                ) {
                    controller
                        .predator_combo_store
                        .set_stance(me.get_predator_stance());
                    controller
                        .predator_combo_store
                        .set_parry(target.can_parry())
                        .set_prone(target.is_prone())
                        .set_rebounds(
                            (if target.will_be_rebounding(me.get_qeb_balance()) {
                                1
                            } else {
                                0
                            }) + (if target.is(FType::Shielded) { 1 } else { 0 }),
                        );
                    controller.predator_combos = controller.predator_combo_store.find_combos();
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Bloodscourge => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
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
            PredatorBehavior::Fleshbane => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venom = controller.get_venoms_from_plan(1, you);
                    controller.plan.add_to_qeb(Box::new(FleshbaneAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                        if venom.is_empty() { "" } else { venom[0] },
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Intoxicate => {
                if let (Some(me), Some(you)) = (
                    AetTarget::Me.get_target(model, controller),
                    AetTarget::Target.get_target(model, controller),
                ) {
                    if me
                        .check_if_predator(&|me| {
                            !me.has_spider()
                                || me.is_intoxicating(
                                    &controller.target.clone().unwrap_or("".to_string()),
                                )
                        })
                        .unwrap_or(true)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(IntoxicateAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Acid => {
                if let (Some(me), Some(you)) = (
                    AetTarget::Me.get_target(model, controller),
                    AetTarget::Target.get_target(model, controller),
                ) {
                    if you.is(FType::Acid) {
                        return UnpoweredFunctionState::Failed;
                    } else if !me.check_if_predator(&|me| me.has_spider()).unwrap_or(false) {
                        return UnpoweredFunctionState::Failed;
                    } else if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(AcidAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
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
                    } else if you.is(FType::Shielded) {
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
                    } else if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venoms = controller.get_venoms_from_plan(2, you);
                    if let (Some(venom_0), Some(venom_1)) = (venoms.get(0), venoms.get(1)) {
                        controller.plan.add_to_qeb(Box::new(TwinshotAction::new(
                            controller.target.clone().unwrap_or("".to_string()),
                            venom_1,
                            venom_0,
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
                    } else if you.is(FType::Shielded) {
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
            PredatorBehavior::Rake => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let me = model.state.borrow_me();
                    if me.check_if_predator(&|me| me.is_raking()).unwrap_or(false) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(RakeAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Swipe => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(SwipeAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Throw => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) || you.is(FType::Density) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(ThrowAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Weaken => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(WeakenAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Roar => {
                controller.plan.add_to_qeb(Box::new(RoarAction::new()));
                UnpoweredFunctionState::Complete
            }
            PredatorBehavior::Pummel(limb) => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(PummelAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
                        limb.clone(),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            PredatorBehavior::Mawcrush => {
                if let Some(you) = AetTarget::Target.get_target(model, controller) {
                    if you.is(FType::Shielded) || !you.get_limb_state(LType::TorsoDamage).broken {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(MawcrushAction::new(
                        controller.target.clone().unwrap_or("".to_string()),
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
    preferred_limbs: &Vec<LType>,
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
                preferred_limbs,
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
                preferred_limbs,
            )));
        UnpoweredFunctionState::Complete
    } else {
        UnpoweredFunctionState::Failed
    }
}
