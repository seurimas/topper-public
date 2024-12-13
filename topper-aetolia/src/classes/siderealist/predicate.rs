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
        if let Some(target) = aet_target.get_target(model, controller) {
            match self {
                SiderealistPredicate::VibrationInRoom(vibration) => {
                    let me = model.state.borrow_me();
                    vibration_in_room(&model.state, me.room_id, *vibration)
                }
                SiderealistPredicate::VibrationDormant(vibration) => {
                    let me = model.state.borrow_me();
                    vibration_dormant_in_room(&model.state, me.room_id, *vibration)
                }
                SiderealistPredicate::VibrationSomewhere(vibration) => target
                    .check_if_siderealist(&|me| me.has_vibration(*vibration))
                    .unwrap_or(false),
                SiderealistPredicate::EnigmaAlive => target
                    .check_if_siderealist(&|me| me.has_glimmercrest())
                    .unwrap_or(false),
                SiderealistPredicate::EmbodyAlive => target
                    .check_if_siderealist(&|me| me.has_sprite())
                    .unwrap_or(false),
                SiderealistPredicate::HasDustring => target.siderealist_board.has_dustring(),
                SiderealistPredicate::HasAsterism => target.siderealist_board.has_asterism(),
                SiderealistPredicate::HasMoonlet => target.siderealist_board.has_moonlet(),
                SiderealistPredicate::HasGleam => target
                    .check_if_siderealist(&|me| me.has_gleam())
                    .unwrap_or(false),
                SiderealistPredicate::HasGleamStar(color) => target
                    .check_if_siderealist(&|me| me.has_gleam_star(*color))
                    .unwrap_or(false),
                SiderealistPredicate::HasParallax => target
                    .check_if_siderealist(&|me| me.has_parallax())
                    .unwrap_or(false),
                SiderealistPredicate::Parallaxing(spell) => target
                    .check_if_siderealist(&|me| me.is_parallaxing(spell))
                    .unwrap_or(false),
                SiderealistPredicate::Irradiated(limb) => {
                    target.siderealist_board.has_irradiated_limb(
                        limb.get_limb(model, controller, aet_target)
                            .unwrap_or(LType::SIZE),
                    )
                }
                SiderealistPredicate::HasRegalia(regalia) => target
                    .check_if_siderealist(&|me| me.has_regalia(*regalia))
                    .unwrap_or(false),
            }
        } else {
            false
        }
    }
}
