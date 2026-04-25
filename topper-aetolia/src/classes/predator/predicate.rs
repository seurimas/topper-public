use serde::*;
use behavior_bark::unpowered::*;
use topper_core::timeline::CType;

use crate::{
    bt::*,
    classes::{AFFLICT_VENOMS, VENOM_AFFLICTS},
    timeline::apply_functions::apply_venom,
    types::*,
};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PredatorPredicate {
    InStance(KnifeStance),
    CanFeint,
    Fleshbaned,
    FleshbanedOver(u32),
    Bloodscourged,
    TidalslashReady,
    Veinripped,
    OrelAboutToHit(AetTarget, f32, Vec<FType>),
    OrelSwooping(Option<AetTarget>),
    Intoxicating(AetTarget),
    Intoxicated,
    Negated,
    ApexAtLeast(u32),
    HasOrgyuk,
    HasSpider,
    HasOrel,
    OrelWithMe,
    OrelHoistingAny,
}

impl TargetPredicate for PredatorPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                PredatorPredicate::InStance(stance) => target
                    .check_if_predator(&|predator| predator.stance == *stance)
                    .unwrap_or(false),
                PredatorPredicate::CanFeint => target
                    .check_if_predator(&|predator| predator.feint_time < 0)
                    .unwrap_or(false),
                PredatorPredicate::Fleshbaned => target.predator_board.fleshbane.is_active(),
                PredatorPredicate::FleshbanedOver(count) => {
                    target.predator_board.fleshbane_count >= *count
                }
                PredatorPredicate::Bloodscourged => target.predator_board.bloodscourge.is_active(),
                PredatorPredicate::TidalslashReady => target
                    .check_if_predator(&|predator| predator.tidalslash)
                    .unwrap_or(false),
                PredatorPredicate::Veinripped => target.predator_board.veinrip.is_active(),
                PredatorPredicate::OrelAboutToHit(attack_target, buffer, venom_check) => target
                    .check_if_predator(&|predator| {
                        if let Some((target_name, timer, (venom_0, venom_1))) =
                            predator.get_swooping()
                        {
                            if timer.get_time_left_seconds() > *buffer {
                                return false;
                            }
                            if target_name != attack_target.get_name(model, controller) {
                                return false;
                            }
                            return venom_check.iter().all(|aff| {
                                venom_0.eq_ignore_ascii_case(AFFLICT_VENOMS.get(aff).unwrap_or(&""))
                                    || venom_1.eq_ignore_ascii_case(
                                        AFFLICT_VENOMS.get(aff).unwrap_or(&""),
                                    )
                            });
                        }
                        false
                    })
                    .unwrap_or(false),
                PredatorPredicate::OrelSwooping(attack_target) => target
                    .check_if_predator(&|predator| {
                        if let Some((target_name, _, _)) = predator.get_swooping() {
                            if let Some(attack_target) = attack_target {
                                return target_name == attack_target.get_name(model, controller);
                            } else {
                                return true;
                            }
                        }
                        false
                    })
                    .unwrap_or(false),
                PredatorPredicate::Intoxicating(other_target) => target
                    .check_if_predator(&|predator| {
                        predator.is_intoxicating(&other_target.get_name(model, controller))
                    })
                    .unwrap_or(false),
                PredatorPredicate::Intoxicated => target.predator_board.is_intoxicated(),
                PredatorPredicate::Negated => target.predator_board.is_negated(),
                PredatorPredicate::ApexAtLeast(apex) => target
                    .check_if_predator(&|predator| predator.apex >= *apex)
                    .unwrap_or(false),
                PredatorPredicate::HasOrgyuk => target
                    .check_if_predator(&|predator| predator.has_orgyuk())
                    .unwrap_or(false),
                PredatorPredicate::HasSpider => target
                    .check_if_predator(&|predator| predator.has_spider())
                    .unwrap_or(false),
                PredatorPredicate::HasOrel => target
                    .check_if_predator(&|predator| predator.has_orel())
                    .unwrap_or(false),
                PredatorPredicate::OrelWithMe => target
                    .check_if_predator(&|predator| predator.is_orel_with_me())
                    .unwrap_or(false),
                PredatorPredicate::OrelHoistingAny => target
                    .check_if_predator(&|predator| predator.is_orel_hoisting_any())
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
