use std::ops::DerefMut;

use behavior_bark::unpowered::*;
use serde::*;
use topper_core::timeline::db::DummyDatabaseModule;

use crate::{
    bt::*,
    classes::{FitnessAction, ParryAction},
    db::AetDatabaseModule,
};

use super::PlainAction;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EnchantmentBehavior {
    Pestilence(AetTarget),
}

impl UnpoweredFunction for EnchantmentBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        match self {
            EnchantmentBehavior::Pestilence(target) => {
                if let Some(me) = AetTarget::Me.get_target(model, controller) {
                    if me.can_use_enchantment() {
                        controller
                            .plan
                            .add_to_qeb(Box::new(PlainAction::new(format!(
                                "point pestilence at {}",
                                target.get_name(model, controller),
                            ))));
                        return UnpoweredFunctionState::Complete;
                    }
                }
                UnpoweredFunctionState::Failed
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to do
    }
}
