use behavior_bark::unpowered::*;
use serde::Deserialize;
use serde::Serialize;

use crate::classes::ascendril::AscendrilBehavior;
use crate::classes::bard::BardBehavior;
use crate::classes::enchants::EnchantmentBehavior;
use crate::classes::infiltrator::InfiltratorBehavior;
use crate::classes::predator::PredatorBehavior;
use crate::classes::sentinel::SentinelBehavior;
use crate::classes::siderealist::SiderealistBehavior;
use crate::classes::zealot::ZealotBehavior;
use crate::classes::LockType;
use crate::classes::VenomPlan;
use crate::curatives::FirstAidSetting;
use crate::defense::DefenseBehavior;
use crate::observables::PlainAction;
use crate::timeline::*;
use crate::types::*;

use super::AetTarget;
use super::LimbDescriptor;
use super::{BehaviorController, BehaviorModel};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AetBehavior {
    UnstackAffs(Vec<FType>),
    PushAff(FType),
    PushLockers(LockType),
    TagPlan(String),
    HintPlan(String, String),
    CopyHint(String, String),
    SetLimbHint(AetTarget, LimbDescriptor, String),
    TouchHammer(AetTarget),
    PlainQebBehavior(String),
    #[serde(untagged)]
    EnchantmentBehavior(EnchantmentBehavior),
    #[serde(untagged)]
    DefenseBehavior(DefenseBehavior),
    #[serde(untagged)]
    BardBehavior(BardBehavior),
    #[serde(untagged)]
    PredatorBehavior(PredatorBehavior),
    #[serde(untagged)]
    InfiltratorBehavior(InfiltratorBehavior),
    #[serde(untagged)]
    AscendrilBehavior(AscendrilBehavior),
    #[serde(untagged)]
    SentinelBehavior(SentinelBehavior),
    #[serde(untagged)]
    SiderealistBehavior(SiderealistBehavior),
    #[serde(untagged)]
    ZealotBehavior(ZealotBehavior),
}

impl UnpoweredFunction for AetBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            AetBehavior::UnstackAffs(unstacked) => {
                if let Some(priorities) = &mut controller.aff_priorities {
                    priorities.retain(|aff| !unstacked.contains(&aff.affliction()));
                    return UnpoweredFunctionState::Complete;
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::TagPlan(tag) => {
                controller.tag_plan(tag.clone());
                UnpoweredFunctionState::Complete
            }
            AetBehavior::HintPlan(hint_name, hint) => {
                controller.hint_plan(hint_name.clone(), hint.clone());
                UnpoweredFunctionState::Complete
            }
            AetBehavior::CopyHint(source_name, target_name) => {
                if let Some(hint) = controller.get_hint(source_name) {
                    controller.hint_plan(target_name.clone(), hint.clone());
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::SetLimbHint(target, limb_descriptor, hint_name) => {
                let limb = limb_descriptor.get_limb(model, controller, target);
                if let Some(limb) = limb {
                    controller.hint_plan(hint_name.clone(), limb.to_string().to_lowercase());
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::PushAff(aff) => {
                if let Some(priorities) = &mut controller.aff_priorities {
                    priorities.insert(0, VenomPlan::Stick(*aff));
                    return UnpoweredFunctionState::Complete;
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::PushLockers(lock_type) => {
                if let (Some(target), Some(priorities)) = (
                    AetTarget::Target.get_target(model, controller),
                    &mut controller.aff_priorities,
                ) {
                    let mut affs = lock_type.affs();
                    affs.retain(|aff| !target.is(*aff));
                    for aff in affs {
                        priorities.insert(0, VenomPlan::Stick(aff));
                    }
                    return UnpoweredFunctionState::Complete;
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            AetBehavior::TouchHammer(aet_target) => {
                let Some(target) = aet_target.get_target(model, controller) else {
                    return UnpoweredFunctionState::Failed;
                };
                if !target.is(FType::Shielded) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(PlainAction::new(format!(
                        "touch hammer {}",
                        aet_target.get_name(model, controller),
                    ))));
                UnpoweredFunctionState::Complete
            }
            AetBehavior::PlainQebBehavior(action) => {
                controller
                    .plan
                    .add_to_qeb(Box::new(PlainAction::new(action.clone())));
                UnpoweredFunctionState::Complete
            }
            AetBehavior::EnchantmentBehavior(enchantment_behavior) => {
                enchantment_behavior.resume_with(model, controller)
            }
            AetBehavior::DefenseBehavior(defense_behavior) => {
                defense_behavior.resume_with(model, controller)
            }
            AetBehavior::BardBehavior(bard_behavior) => {
                bard_behavior.resume_with(model, controller)
            }
            AetBehavior::PredatorBehavior(predator_behavior) => {
                predator_behavior.resume_with(model, controller)
            }
            AetBehavior::InfiltratorBehavior(infiltrator_behavior) => {
                infiltrator_behavior.resume_with(model, controller)
            }
            AetBehavior::AscendrilBehavior(ascendril_behavior) => {
                ascendril_behavior.resume_with(model, controller)
            }
            AetBehavior::SentinelBehavior(sentinel_behavior) => {
                sentinel_behavior.resume_with(model, controller)
            }
            AetBehavior::SiderealistBehavior(siderealist_behavior) => {
                siderealist_behavior.resume_with(model, controller)
            }
            AetBehavior::ZealotBehavior(zealot_behavior) => {
                zealot_behavior.resume_with(model, controller)
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        match self {
            _ => {}
        }
    }
}
