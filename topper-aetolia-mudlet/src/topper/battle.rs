use topper_aetolia::classes::get_attack;
use topper_aetolia::timeline::{AetTimeSlice, AetTimeline};
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::firstaid::FirstAidModule;

#[derive(Default)]
pub struct BattleModule;

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for BattleModule {
    type Siblings = (
        &'s mut FirstAidModule,
        &'s Option<String>,
        &'s AetTimeline,
        &'s AetMudletDatabaseModule,
    );
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        (firstaid, target, timeline, db): Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        let me = timeline.who_am_i();
        match message {
            TopperMessage::Request(request) => match request {
                TopperRequest::Attack(strategy) => {
                    if let Some(target) = target {
                        Ok(TopperResponse::qeb(get_attack(
                            &timeline,
                            &me,
                            &target,
                            &strategy,
                            Some(db),
                            firstaid.start_temporary_fa_settings(),
                        )))
                    } else {
                        Ok(TopperResponse::qeb(get_attack(
                            &timeline,
                            &me,
                            &"".to_string(),
                            &strategy,
                            Some(db),
                            firstaid.start_temporary_fa_settings(),
                        )))
                    }
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}
