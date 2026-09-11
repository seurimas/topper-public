use topper_aetolia::classes::get_attack;
use topper_aetolia::non_agent::AetNonAgent;
use topper_aetolia::timeline::{
    AetObservation, AetPrompt, AetTimeSlice, AetTimelineStateTrait, AetTimelineTrait,
    REALTIME_STAT_NAMES,
};
use topper_aetolia::types::AgentState;
use topper_core::timeline::{BaseTimeline, CType};
use topper_core_mudlet::topper::{
    TimelineModule, TopperMessage, TopperModule, TopperRequest, TopperResponse,
};

use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;

pub type AetTimelineModule = TimelineModule<AetObservation, AetPrompt, AgentState, AetNonAgent>;

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for AetTimelineModule {
    type Siblings = (&'s AetMudletDatabaseModule, &'s Option<String>);
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        let (db, target) = siblings;
        match message {
            TopperMessage::TimeSlice(timeslice) => {
                self.timeline.push_time_slice(timeslice.clone(), Some(db))?;
                if self.snapshotting {
                    self.snapshots.push((timeslice.time, self.timeline.clone()));
                }
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
                TopperRequest::ModuleMsg(module, command) if module.eq("timeline") => {
                    if command.eq("snapshots on") {
                        self.snapshotting = true;
                        self.snapshots.clear();
                        println!("Snapshotting on!");
                        Ok(TopperResponse::silent())
                    } else if command.eq("snapshots off") {
                        self.snapshotting = false;
                        println!("Snapshotting off!");
                        Ok(TopperResponse::silent())
                    } else if let Some(rest) = command.strip_prefix("snapshot check ") {
                        println!("{}", snapshot_check(&self.snapshots, rest, target, db));
                        Ok(TopperResponse::silent())
                    } else {
                        Ok(TopperResponse::silent())
                    }
                }
                _ => Ok(TopperResponse::silent()),
            },
            _ => Ok(TopperResponse::silent()),
        }
    }
}

fn snapshot_check(
    snapshots: &Vec<(CType, topper_aetolia::timeline::AetTimeline)>,
    rest: &str,
    target: &Option<String>,
    db: &AetMudletDatabaseModule,
) -> String {
    let (time_str, strategy) = match rest.split_once(' ') {
        Some((time_str, strategy)) => (time_str, strategy.to_string()),
        None => (rest, String::new()),
    };
    let time = match time_str.parse::<CType>() {
        Ok(time) => time,
        Err(_) => return format!("Invalid snapshot time: {}", time_str),
    };
    let target = match target {
        Some(target) => target,
        None => return "No target set.".to_string(),
    };
    let snapshot = snapshots
        .iter()
        .rev()
        .find(|(snapshot_time, _)| *snapshot_time <= time)
        .map(|(_, timeline)| timeline);
    match snapshot {
        Some(timeline) => get_attack(
            timeline,
            &timeline.who_am_i(),
            target,
            &strategy,
            Some(db),
            &mut Vec::new(),
        ),
        None => format!("No snapshot found at or before time {}", time),
    }
}
