use serde::Serialize;
use serde_json::Value;
use topper_aetolia::observables::ActiveTransition;

#[derive(Clone, Copy)]
pub(crate) struct ActionDef {
    pub id: &'static str,
    pub label: &'static str,
    pub targeted: bool,
    pub builder: fn(String, String) -> Box<dyn ActiveTransition>,
}

#[derive(Clone, Copy)]
pub(crate) struct PassiveDef {
    pub id: &'static str,
    pub label: &'static str,
    pub category: &'static str,
    pub skill: &'static str,
    pub targeted: bool,
    pub period_ms: i32,
}

#[derive(Serialize)]
pub(crate) struct ActionDescriptor {
    pub id: String,
    pub label: String,
    pub targeted: bool,
}

#[derive(Serialize)]
pub(crate) struct PassiveDescriptor {
    pub id: String,
    pub label: String,
    pub period_ms: i32,
    pub targeted: bool,
}

#[derive(Serialize)]
pub(crate) struct PlayerBattleStats {
    pub name: String,
    pub class_name: String,
    pub vitals: Value,
    pub balances: Value,
    pub afflictions: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct BattleStatsView {
    pub current_time: i32,
    pub learner: PlayerBattleStats,
    pub target: PlayerBattleStats,
}

#[derive(Serialize)]
pub(crate) struct ActionExecution {
    pub action_id: String,
    pub command: String,
    pub applied_time: i32,
    pub observations: Vec<String>,
    pub battle_stats: BattleStatsView,
}

#[derive(Serialize)]
pub(crate) struct EngineStateView {
    pub active_class: String,
    pub battle_stats: BattleStatsView,
}
