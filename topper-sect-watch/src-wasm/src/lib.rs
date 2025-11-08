mod utils;

use std::vec;

use serde_json::json;
use topper_aetolia::{
    explainer::{observations::OBSERVER, ExplainerPage},
    timeline::{
        AetObservation, AetTimeSlice, AetTimelineState, AetTimelineStateTrait, CombatAction,
    },
    types::{BType, FType},
};
use topper_core::timeline::db::DummyDatabaseModule;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTimeSlices(Vec<AetTimeSlice>);

#[wasm_bindgen]
impl WasmTimeSlices {
    /**
     * Critically, we need to parse the time slices from the explainer page and store them
     * for later use. The timeline module does not care about the plain text values.
     */
    #[wasm_bindgen(constructor)]
    pub fn new(page_string: &str) -> WasmTimeSlices {
        utils::set_panic_hook();

        let page = serde_json::from_str::<ExplainerPage>(page_string).unwrap();
        let slices = page.build_time_slices(&|slice| OBSERVER.observe(slice));
        WasmTimeSlices(slices)
    }

    #[wasm_bindgen]
    pub fn get_times(&self) -> Vec<i32> {
        self.0.iter().map(|slice| slice.time).collect()
    }
}

#[wasm_bindgen]
pub struct WasmTimeline(AetTimelineState);

#[wasm_bindgen]
impl WasmTimeline {
    #[wasm_bindgen(constructor)]
    pub fn new(me: &str) -> WasmTimeline {
        let mut timeline = AetTimelineState::new();
        timeline.me = me.to_string();
        WasmTimeline(timeline)
    }

    #[wasm_bindgen]
    pub fn get_current_time(&self) -> i32 {
        self.0.time
    }

    #[wasm_bindgen]
    pub fn get_limb_state(&self, who: &str) -> JsValue {
        let me = self.0.borrow_agent(&who.to_string());
        let limbs = me.get_limbs_state();
        serde_wasm_bindgen::to_value(&limbs).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_balances(&self, who: &str) -> JsValue {
        let me = self.0.borrow_agent(&who.to_string());
        serde_wasm_bindgen::to_value(&json!({
            "Tree": me.get_balance(BType::Tree),
            "Focus": me.get_balance(BType::Focus),
            "Fitness": me.get_balance(BType::Fitness),
            "ClassCure1": me.get_balance(BType::ClassCure1),
            "Rebounding": if me.is(FType::Rebounding) { None } else { Some(me.get_balance(BType::Rebounding)) },
        })).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_afflictions(&self, who: &str) -> JsValue {
        let me = self.0.borrow_agent(&who.to_string());
        let afflictions = me
            .flags
            .aff_iter()
            .map(|aff: FType| aff.to_name())
            .collect::<Vec<String>>();
        serde_wasm_bindgen::to_value(&afflictions).unwrap()
    }

    #[wasm_bindgen]
    pub fn set_timeline_time(&mut self, slices: &WasmTimeSlices, time: i32) -> Vec<String> {
        let mut combat_actions = vec![];
        let timeline = &mut self.0;
        let timeline_current_time = timeline.time;
        let me = timeline.me.clone();

        // We return combat actions that target me, so we can announce them.
        let mut add_combat_action = |obs: &AetObservation| match obs {
            AetObservation::CombatAction(CombatAction { skill, target, .. }) => {
                if target == &me {
                    combat_actions.push(skill.clone());
                }
            }
            _ => {}
        };

        if time > timeline_current_time {
            for slice in slices.0.iter() {
                if slice.time > timeline_current_time && slice.time <= time {
                    timeline.apply_time_slice(slice, None as Option<&DummyDatabaseModule>);
                    slice
                        .observations
                        .iter()
                        .flatten()
                        .for_each(&mut add_combat_action);
                } else if slice.time > time {
                    break;
                }
            }
        } else if time < timeline_current_time {
            let me = timeline.me.clone();
            *timeline = AetTimelineState::new();
            timeline.me = me;
            for slice in slices.0.iter() {
                if slice.time <= time {
                    timeline.apply_time_slice(slice, None as Option<&DummyDatabaseModule>);
                    slice
                        .observations
                        .iter()
                        .flatten()
                        .for_each(&mut add_combat_action);
                } else if slice.time > time {
                    break;
                }
            }
        }
        combat_actions
    }
}
