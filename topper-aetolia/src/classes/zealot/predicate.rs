use serde::{Deserialize, Serialize};

use crate::{agent::*, bt::*};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ZealotPredicate {
    CanWrath,
    HasDisable,
    SappedStrengthOver(u8),
    SappedStrengthUnder(u8),
    HasPyromania,
    HasInfernalAny,
    HasInfernalFull,
}

impl TargetPredicate for ZealotPredicate {
    fn check(
        &self,
        target: &AetTarget,
        world: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        let Some(target) = target.get_target(world, controller) else {
            return false;
        };
        match self {
            ZealotPredicate::CanWrath => target.balanced(BType::wrath()),
            ZealotPredicate::HasDisable => target.balanced(BType::disable()),
            ZealotPredicate::SappedStrengthOver(amount) => {
                target.get_count(FType::SappedStrength) > *amount
            }
            ZealotPredicate::SappedStrengthUnder(amount) => {
                target.get_count(FType::SappedStrength) < *amount
            }
            ZealotPredicate::HasPyromania => target
                .check_if_zealot(&|z| z.pyromania.active())
                .unwrap_or(false),
            ZealotPredicate::HasInfernalAny => {
                target.is(FType::InfernalSeal) || target.is(FType::InfernalShroud)
            }
            ZealotPredicate::HasInfernalFull => target.is(FType::InfernalShroud),
        }
    }
}
