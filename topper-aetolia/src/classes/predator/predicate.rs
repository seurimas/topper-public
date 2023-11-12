use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{bt::*, classes::VENOM_AFFLICTS, timeline::apply_functions::apply_venom, types::*};

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PredatorPredicate {
    InStance(Stance),
    CanFeint,
    Fleshbaned,
    Bloodscourged,
    TidalslashReady,
    Veinripped,
    Intoxicating(AetTarget),
    ApexAtLeast(u32),
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
                PredatorPredicate::Bloodscourged => target.predator_board.bloodscourge.is_active(),
                PredatorPredicate::TidalslashReady => target
                    .check_if_predator(&|predator| predator.tidal_charge == 2)
                    .unwrap_or(false),
                PredatorPredicate::Veinripped => target.predator_board.veinrip.is_active(),
                PredatorPredicate::Intoxicating(other_target) => {
                    if let Some(target_name) = other_target.get_name(model, controller) {
                        target
                            .check_if_predator(&|predator| predator.is_intoxicating(&target_name))
                            .unwrap_or(false)
                    } else {
                        false
                    }
                }
                PredatorPredicate::ApexAtLeast(apex) => target
                    .check_if_predator(&|predator| predator.apex >= *apex)
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
