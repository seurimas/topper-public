use super::battle_stats::BattleStats;
use super::db::AetMudletDatabaseModule;
use super::BattleModule;
use topper_aetolia::db::AetDatabaseModule;
use topper_aetolia::timeline::AetTimeline;
use topper_aetolia::types::*;
use topper_aetolia::{classes::*, curatives::*, timeline::AetTimeSlice};
use topper_core_mudlet::topper::*;

#[derive(Debug, Default)]
pub struct FirstAidModule {
    active: FirstAidConfig,
    in_flight: FirstAidConfig,
    battle_stats_fa_settings: Vec<FirstAidSetting>,
}

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for FirstAidModule {
    type Siblings = (&'s mut AetTimeline, &'s AetMudletDatabaseModule, &'s String);

    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::Request(TopperRequest::ModuleMsg(module, command)) => {
                if module.eq("firstaid") {
                    if command.eq("check") {
                        println!("FirstAidModule: check {:?}", self);
                    }
                }
                Ok(TopperResponse::silent())
            }
            TopperMessage::TimeSlice(slice) => {
                self.apply_seen_settings(slice);
                self.infer_predict_and_elevate_clears(siblings.0);
                self.send_setting_updates()
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}

impl FirstAidModule {
    pub fn battle_stats_fa_settings_mut(&mut self) -> &mut Vec<FirstAidSetting> {
        &mut self.battle_stats_fa_settings
    }

    fn apply_seen_settings(&mut self, slice: &AetTimeSlice) {
        for (line, _idx) in &slice.lines {
            let changed_settings = FirstAidSetting::get_from_line(line);
            for setting in changed_settings {
                println!("FirstAidModule: {:?}", setting);
                self.active.update(setting.clone());
                let previous = self.in_flight.update(setting.clone());
                if previous != setting {
                    println!("FirstAidModule out of sync: {:?}", setting);
                }
            }
        }
        if let Some((_name, priority_set)) = parse_priority_set(&slice.lines) {
            let settings = priority_set.to_settings();
            for setting in settings {
                println!("FirstAidModule: {:?}", setting);
                self.active.update(setting.clone());
                let previous = self.in_flight.update(setting.clone());
                if previous != setting {
                    println!("FirstAidModule out of sync: {:?}", setting);
                }
            }
        }
    }

    fn infer_predict_and_elevate_clears(&mut self, timeline: &AetTimeline) {
        let mut clear_settings = Vec::new();
        for predicted in &self.active.predicted {
            if let Some(states) = timeline.state.get_agent(&timeline.who_am_i()) {
                if states.iter().all(|state| !state.is(*predicted)) {
                    clear_settings.push(FirstAidSetting::UnPredict(*predicted));
                }
            }
        }
        for elevated in &self.active.elevated {
            if let Some(states) = timeline.state.get_agent(&timeline.who_am_i()) {
                if states.iter().all(|state| !state.is(*elevated)) {
                    clear_settings.push(FirstAidSetting::UnElevate(*elevated));
                }
            }
        }
        for cleared in clear_settings {
            self.active.update(cleared.clone());
            let previous = self.in_flight.update(cleared.clone());
            if previous != cleared {
                println!("FirstAidModule out of sync: {:?}", cleared);
            }
        }
    }

    fn send_setting_updates(&mut self) -> Result<TopperResponse<BattleStats>, String> {
        let mut result = TopperResponse::silent();
        for setting in self.battle_stats_fa_settings.drain(..) {
            let previous = self.in_flight.update(setting.clone());
            if previous != setting {
                println!("FirstAidModule sending: {:?}", setting);
                result = result.then(TopperResponse::passive(
                    "firstaid".to_string(),
                    setting.get_command(),
                ));
            }
        }
        Ok(result)
    }
}
