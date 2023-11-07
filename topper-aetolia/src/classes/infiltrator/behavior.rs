use serde::*;
use topper_bt::unpowered::*;

use crate::{
    bt::{BehaviorController, BehaviorModel},
    with_defense_db,
};

use super::{actions::*, choose_venoms};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum FlayType {
    None,
    Shield,
    Rebounding,
    Cloak,
    Speed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum InfiltratorVenomAttack {
    Doublestab,
    Flay(FlayType),
    Slit,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum InfiltratorBehavior {
    VenomAttack(InfiltratorVenomAttack),
    Bedazzle,
    StackSuggest,
    Seal,
    ShrugVenom,
    Bind,
    Bite(String),
}

impl UnpoweredFunction for InfiltratorBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        let me = model.state.borrow_me();
        // match self {
        //     InfiltratorBehavior::VenomAttack(attack) => match attack {
        //         InfiltratorVenomAttack::Doublestab => {
        //             if me.stuck_fallen() {
        //                 return UnpoweredFunctionState::Failed;
        //             }
        //             if let Some(target) = controller.target.clone() {
        //                 with_defense_db!(db, {
        //                     let two_venoms = choose_venoms(&model, &model.who_am_i(), &target, strategy, venom_plan, controller., 2)
        //                     controller.plan.add_to_qeb(Box::new(DoublestabAction::new(model.who_am_i(), target, v1, v2)));
        //                 });
        //             }
        //         }
        //         InfiltratorVenomAttack::Flay(flay_type) => {
        //             controller.flay(flay_type);
        //         }
        //         InfiltratorVenomAttack::Slit => {
        //             controller.slit();
        //         }
        //     },
        //     InfiltratorBehavior::Bedazzle => {
        //         controller.bedazzle();
        //     }
        //     InfiltratorBehavior::StackSuggest => {
        //         controller.stack_suggest();
        //     }
        //     InfiltratorBehavior::Seal => {
        //         controller.seal();
        //     }
        //     InfiltratorBehavior::ShrugVenom => {
        //         controller.shrug_venom();
        //     }
        //     InfiltratorBehavior::Bind => {
        //         controller.bind();
        //     }
        //     InfiltratorBehavior::Bite(target) => {
        //         controller.bite(target);
        //     }
        // }
        UnpoweredFunctionState::Failed
    }

    fn reset(self: &mut Self, model: &Self::Model) {}
}
