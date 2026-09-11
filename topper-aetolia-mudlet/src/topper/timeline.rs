use topper_aetolia::non_agent::AetNonAgent;
use topper_aetolia::timeline::{
    AetObservation, AetPrompt, AetTimeSlice, AetTimelineStateTrait, AetTimelineTrait,
    REALTIME_STAT_NAMES,
};
use topper_aetolia::types::AgentState;
use topper_core::timeline::BaseTimeline;
use topper_core_mudlet::topper::{
    TimelineModule, TopperMessage, TopperModule, TopperRequest, TopperResponse,
};

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;

pub type AetTimelineModule = TimelineModule<AetObservation, AetPrompt, AgentState, AetNonAgent>;

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for AetTimelineModule {
    type Siblings = (&'s AetMudletDatabaseModule,);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                self.timeline
                    .push_time_slice(timeslice.clone(), Some(siblings.0))?;
                Ok(TopperResponse::silent())
            }
            TopperMessage::Request(request) => match request {
                TopperRequest::BattleStats(when) => {
                    self.timeline.update_time(*when, &REALTIME_STAT_NAMES)?;
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Hint(who, hint, value) => {
                    self.timeline
                        .state
                        .add_player_hint(&who, &hint, value.to_string());
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Assume(who, aff_or_def, value) => {
                    self.timeline
                        .state
                        .set_flag_for_agent(&who, &aff_or_def, *value);
                    Ok(TopperResponse::silent())
                }
                TopperRequest::Reset(reset_type) => {
                    self.timeline.reset(reset_type.eq("full"));
                    Ok(TopperResponse::silent())
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}
