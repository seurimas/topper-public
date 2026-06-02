use behavior_bark::unpowered::*;
use serde::*;
use topper_core::timeline::CType;

use crate::{
    bt::*, classes::VENOM_AFFLICTS, non_agent::AetTimelineRoomExt,
    timeline::apply_functions::apply_venom, types::*,
};

use super::{actions::*, phenomenon_in_room};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AscendrilPredicate {
    PhenomenaOn(Option<PhenomenaKind>),
    PhenomenaChasing(Option<PhenomenaKind>),
    SunspotOn,
    IciclesOn,
    ShatteringOn,
    AeroblastOn,
    FulcrumUp,
    FulcrumUpHere,
    FulcrumExpandedHere,
    NoResonance(Option<Element>),
    AnyResonance(Element),
    Resonance(Element),
    HalfResonance(Element),
    FireburstAtLeast(i32),
    AfterburnRaising,
    Afterburned,
    SchismOnHere,
    ImbalanceOnHere,
    IsResonantOrCanEnrich(Element),
    CapacitanceRaising,
    CapacitanceUp,
    CapacitanceWillDisrupt,
    DegradationOnHere,
    DegradationOn,
    SpiritriftOnHere,
    ShiftAvailable,
    HasEmberbrand,
    HasFrostbrand,
    HasThunderbrand,
}

impl TargetPredicate for AscendrilPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        let me = model.state.borrow_me();
        if let (target_name, Some(target)) = (
            aet_target.get_name(model, controller),
            aet_target.get_target(model, controller),
        ) {
            match self {
                AscendrilPredicate::PhenomenaOn(kind) => match kind {
                    Some(kind) => phenomenon_in_room(&model.state, *kind),
                    None => {
                        phenomenon_in_room(&model.state, PhenomenaKind::Blazewhirl)
                            || phenomenon_in_room(&model.state, PhenomenaKind::Glazeflow)
                            || phenomenon_in_room(&model.state, PhenomenaKind::Electrosphere)
                    }
                },
                AscendrilPredicate::PhenomenaChasing(kind) => me
                    .check_if_ascendril(&|me| me.phenomenon_chasing(None, &target_name, *kind))
                    .unwrap_or(false),
                AscendrilPredicate::SunspotOn => target.ascendril_board.sunspot_active(),
                AscendrilPredicate::IciclesOn => target.ascendril_board.icicles_active(),
                AscendrilPredicate::ShatteringOn => target.ascendril_board.shattering_active(),
                AscendrilPredicate::AeroblastOn => target.ascendril_board.aeroblast_active(),
                AscendrilPredicate::FulcrumUp => target
                    .check_if_ascendril(&|me| me.fulcrum_active())
                    .unwrap_or(false),
                AscendrilPredicate::FulcrumUpHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.fulcrum_active_here(room))
                        .unwrap_or(false)
                }
                AscendrilPredicate::FulcrumExpandedHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.fulcrum_expanded(room))
                        .unwrap_or(false)
                }
                AscendrilPredicate::SchismOnHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.schism_active(Some(room)))
                        .unwrap_or(false)
                }
                AscendrilPredicate::ImbalanceOnHere => {
                    let room = target.room_id;
                    target
                        .check_if_ascendril(&|me| me.imbalance_active(Some(room)))
                        .unwrap_or(false)
                }
                AscendrilPredicate::NoResonance(element) => target
                    .check_if_ascendril(&|me| {
                        if let Some(element) = element {
                            !me.resonance_active(element) && !me.half_resonance_active(element)
                        } else {
                            me.has_no_resonance()
                        }
                    })
                    .unwrap_or(false),
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
                AscendrilPredicate::IsResonantOrCanEnrich(element) => target
                    .check_if_ascendril(&|asc| {
                        asc.resonance_active(element) || asc.can_enrich(me.room_id, element)
                    })
                    .unwrap_or(false),
                AscendrilPredicate::CapacitanceRaising => me
                    .check_if_ascendril(&|me| me.capacitance_coming_up())
                    .unwrap_or(false),
                AscendrilPredicate::CapacitanceUp => me
                    .check_if_ascendril(&|me| me.capacitance_active())
                    .unwrap_or(false),
                AscendrilPredicate::CapacitanceWillDisrupt => me
                    .check_if_ascendril(&|me| me.capacitance_will_disrupt())
                    .unwrap_or(false),
                AscendrilPredicate::DegradationOnHere => {
                    let room = me.room_id;
                    me.check_if_ascendril(&|me| me.degradation_active(Some(room)))
                        .unwrap_or(false)
                }
                AscendrilPredicate::DegradationOn => me
                    .check_if_ascendril(&|me| me.degradation_active(None))
                    .unwrap_or(false),
                AscendrilPredicate::SpiritriftOnHere => {
                    let room = me.room_id;
                    me.check_if_ascendril(&|me| me.spiritrift_active(Some(room)))
                        .unwrap_or(false)
                }
                AscendrilPredicate::ShiftAvailable => {
                    me.check_if_ascendril(&|me| me.can_shift()).unwrap_or(false)
                }
                AscendrilPredicate::HasEmberbrand => target.is(FType::Emberbrand),
                AscendrilPredicate::HasFrostbrand => target.is(FType::Frostbrand),
                AscendrilPredicate::HasThunderbrand => target.is(FType::Thunderbrand),
            }
        } else {
            false
        }
    }
}
