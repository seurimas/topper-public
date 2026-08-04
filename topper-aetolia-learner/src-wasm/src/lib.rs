mod utils;

use serde::{Deserialize, Serialize};
use topper_aetolia::timeline::{
    simulation_slice, AetObservation, AetTimelineState, AetTimelineStateTrait, CombatAction,
};
use topper_core::timeline::db::DummyDatabaseModule;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize)]
struct LearnerScenario {
    attacker: String,
    target: String,
    horizon_ms: i32,
    candidate_actions: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LearnerResult {
    best_action_order: Vec<String>,
    score: f32,
    expected_observations: usize,
    used_timeline: bool,
    used_observables: bool,
    notes: Vec<String>,
}

#[wasm_bindgen]
pub fn simulate_plan(scenario_json: &str) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let scenario: LearnerScenario =
        serde_json::from_str(scenario_json).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut timeline = AetTimelineState::new();
    timeline.me = scenario.attacker.clone();

    // Seed a tiny simulation slice so we exercise timeline/observation plumbing now.
    let opener = scenario
        .candidate_actions
        .first()
        .cloned()
        .unwrap_or_else(|| "wait".to_string());
    let observations = vec![AetObservation::CombatAction(CombatAction {
        caster: scenario.attacker.clone(),
        category: "Learner".to_string(),
        skill: opener.clone(),
        annotation: "seed simulation".to_string(),
        target: scenario.target.clone(),
    })];
    let _ = timeline.apply_time_slice(
        &simulation_slice(observations.clone(), scenario.horizon_ms.max(1)),
        None as Option<&DummyDatabaseModule>,
    );

    let result = LearnerResult {
        best_action_order: scenario.candidate_actions.clone(),
        score: score_actions(&scenario.candidate_actions, scenario.horizon_ms),
        expected_observations: observations.len(),
        used_timeline: true,
        used_observables: true,
        notes: vec![
            "Initial scorer is deterministic and placeholder-only.".to_string(),
            "Extend ActiveTransition::simulate coverage to improve branch realism.".to_string(),
            format!("Seed action was: {opener}"),
        ],
    };

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn score_actions(actions: &[String], horizon_ms: i32) -> f32 {
    if actions.is_empty() {
        return 0.0;
    }

    let action_bonus = (actions.len() as f32).recip();
    let horizon_penalty = (horizon_ms.max(1) as f32) / 10000.0;
    (1.0 + action_bonus - horizon_penalty).max(0.0)
}
