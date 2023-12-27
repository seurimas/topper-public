use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::*, classes::group::*, non_agent::AetTimelineRoomExt, observables::PlainAction, types::*,
};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MonkBehavior {
    MindLock(AetTarget),
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
                if let Some(target) = target.get_target(model, controller) {
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}
