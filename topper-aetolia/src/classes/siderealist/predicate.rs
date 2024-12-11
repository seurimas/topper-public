use behavior_bark::unpowered::*;
use serde::*;
use topper_core::timeline::CType;

use crate::{
    bt::*, classes::VENOM_AFFLICTS, non_agent::AetTimelineRoomExt,
    timeline::apply_functions::apply_venom, types::*,
};

use super::{actions::*, vibration_dormant_in_room, vibration_in_room};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SiderealistPredicate {
    VibrationInRoom(Vibration),
    VibrationDormant(Vibration),
    VibrationSomewhere(Vibration),
    EnigmaAlive,
    EmbodyAlive,
    HasDustring,
    HasAsterism,
    HasMoonlet,
    HasGleam,
    HasGleamStar(GleamColor),
    HasParallax,
    Parallaxing(String),
    Irradiated(LimbDescriptor),
    HasRegalia(Regalia),
}

impl TargetPredicate for SiderealistPredicate {
    fn check(
        &self,
        aet_target: &AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
    ) -> bool {
        let me = model.state.borrow_me();
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                SiderealistPredicate::VibrationInRoom(vibration) => {
                    vibration_in_room(&model.state, me.room_id, *vibration)
                }
                SiderealistPredicate::VibrationDormant(vibration) => {
                    vibration_dormant_in_room(&model.state, me.room_id, *vibration)
                }
                SiderealistPredicate::VibrationSomewhere(vibration) => me
                    .check_if_siderealist(&|me| me.has_vibration(*vibration))
                    .unwrap_or(false),
                SiderealistPredicate::EnigmaAlive => me
                    .check_if_siderealist(&|me| me.has_glimmercrest())
                    .unwrap_or(false),
                SiderealistPredicate::EmbodyAlive => me
                    .check_if_siderealist(&|me| me.has_sprite())
                    .unwrap_or(false),
                SiderealistPredicate::HasDustring => me.siderealist_board.has_dustring(),
                SiderealistPredicate::HasAsterism => me.siderealist_board.has_asterism(),
                SiderealistPredicate::HasMoonlet => me.siderealist_board.has_moonlet(),
                SiderealistPredicate::HasGleam => me
                    .check_if_siderealist(&|me| me.has_gleam())
                    .unwrap_or(false),
                SiderealistPredicate::HasGleamStar(color) => me
                    .check_if_siderealist(&|me| me.has_gleam_star(*color))
                    .unwrap_or(false),
                SiderealistPredicate::HasParallax => me
                    .check_if_siderealist(&|me| me.has_parallax())
                    .unwrap_or(false),
                SiderealistPredicate::Parallaxing(spell) => me
                    .check_if_siderealist(&|me| me.is_parallaxing(spell))
                    .unwrap_or(false),
                SiderealistPredicate::Irradiated(limb) => me.siderealist_board.has_irradiated_limb(
                    limb.get_limb(model, controller, aet_target)
                        .unwrap_or(LType::SIZE),
                ),
                SiderealistPredicate::HasRegalia(regalia) => me
                    .check_if_siderealist(&|me| me.has_regalia(*regalia))
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
