use serde::*;
use topper_bt::unpowered::*;
use topper_core::timeline::CType;

use crate::{
    bt::*, classes::VENOM_AFFLICTS, non_agent::AetTimelineRoomExt,
    timeline::apply_functions::apply_venom, types::*,
};

use super::{actions::*, phenomenon_in_room};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AscendrilPredicate {
    BlazewhirlOn,
    GlazeflowOn,
    ElectrosphereOn,
    PhenomenaOn,
    SunspotOn,
    IciclesOn,
    ShatteringOn,
    AeroblastOn,
    AeroblastElectrified,
    FulcrumUp,
    FulcrumExpandedHere,
    AnyResonance(Element),
    Resonance(Element),
    HalfResonance(Element),
    FireburstAtLeast(i32),
    AfterburnRaising,
    Afterburned,
    SchismOnHere,
    ImbalanceOnHere,
}

impl TargetPredicate for AscendrilPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        let me = model.state.borrow_me();
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                AscendrilPredicate::BlazewhirlOn => {
                    phenomenon_in_room(&model.state, me.room_id, PhenomenaState::Blazewhirl)
                }
                AscendrilPredicate::GlazeflowOn => {
                    phenomenon_in_room(&model.state, me.room_id, PhenomenaState::Glazeflow)
                }
                AscendrilPredicate::ElectrosphereOn => {
                    phenomenon_in_room(&model.state, me.room_id, PhenomenaState::Electrosphere)
                }
                AscendrilPredicate::PhenomenaOn => {
                    phenomenon_in_room(&model.state, me.room_id, PhenomenaState::Blazewhirl)
                        || phenomenon_in_room(&model.state, me.room_id, PhenomenaState::Glazeflow)
                        || phenomenon_in_room(
                            &model.state,
                            me.room_id,
                            PhenomenaState::Electrosphere,
                        )
                }
                AscendrilPredicate::SunspotOn => target.ascendril_board.sunspot_active(),
                AscendrilPredicate::IciclesOn => target.ascendril_board.icicles_active(),
                AscendrilPredicate::ShatteringOn => target.ascendril_board.shattering_active(),
                AscendrilPredicate::AeroblastOn => target.ascendril_board.aeroblast_active(),
                AscendrilPredicate::AeroblastElectrified => {
                    target.ascendril_board.aeroblast_electrified()
                }
                AscendrilPredicate::FulcrumUp => target
                    .check_if_ascendril(&|me| me.fulcrum_active())
                    .unwrap_or(false),
                AscendrilPredicate::FulcrumExpandedHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                        .unwrap_or(false)
                }
                AscendrilPredicate::SchismOnHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.schism_active(room))
                        .unwrap_or(false)
                }
                AscendrilPredicate::ImbalanceOnHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.imbalance_active(room))
                        .unwrap_or(false)
                }
                AscendrilPredicate::AnyResonance(element) => target
                    .check_if_ascendril(&|me| {
                        me.resonance_active(element) || me.half_resonance_active(element)
                    })
                    .unwrap_or(false),
                AscendrilPredicate::Resonance(element) => target
                    .check_if_ascendril(&|me| me.resonance_active(element))
                    .unwrap_or(false),
                AscendrilPredicate::HalfResonance(element) => target
                    .check_if_ascendril(&|me| me.half_resonance_active(element))
                    .unwrap_or(false),
                AscendrilPredicate::FireburstAtLeast(stacks) => target
                    .check_if_ascendril(&|me| me.fireburst_stacks() >= *stacks)
                    .unwrap_or(false),
                AscendrilPredicate::AfterburnRaising => target
                    .check_if_ascendril(&|me| me.afterburn_coming_up())
                    .unwrap_or(false),
                AscendrilPredicate::Afterburned => target
                    .check_if_ascendril(&|me| me.afterburn_active())
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
