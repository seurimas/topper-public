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

use super::actions::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PredatorBehavior {
    DoCombo,
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
            PredatorBehavior::DoCombo => UnpoweredFunctionState::Failed,
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        // Nothing to reset.
    }
}
