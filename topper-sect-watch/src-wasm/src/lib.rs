mod utils;

use std::sync::{LazyLock, Mutex};

use serde::Serialize;
use topper_aetolia::{
    explainer::{observations::OBSERVER, ExplainerPage},
    timeline::{AetTimeSlice, AetTimelineState, AetTimelineStateTrait},
};
use topper_core::timeline::db::DummyDatabaseModule;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmTimeSlices(Vec<AetTimeSlice>);

#[wasm_bindgen]
impl WasmTimeSlices {
    #[wasm_bindgen]
    pub fn get_times(&self) -> Vec<i32> {
        self.0.iter().map(|slice| slice.time).collect()
    }
}

/**
 * Critically, we need to parse the time slices from the explainer page and store them
 * for later use. The timeline module does not care about the plain text values.
 */
#[wasm_bindgen]
pub fn get_time_slices(page_string: &str) -> WasmTimeSlices {
    utils::set_panic_hook();

    let page = serde_json::from_str::<ExplainerPage>(page_string).unwrap();
    let slices = page.build_time_slices(&|slice| OBSERVER.observe(slice));
    WasmTimeSlices(slices)
}

#[wasm_bindgen]
pub struct WasmTimeline(AetTimelineState);

#[wasm_bindgen]
pub fn initialize_timeline(me: &str) -> WasmTimeline {
    let mut timeline = AetTimelineState::new();
    timeline.me = me.to_string();
    WasmTimeline(timeline)
}

#[wasm_bindgen]
pub fn set_timeline_time(timeline: &mut WasmTimeline, slices: &WasmTimeSlices, time: i32) -> i32 {
    let mut applied = 0;
    let timeline = &mut timeline.0;
    let timeline_current_time = timeline.time;
    if time > timeline_current_time {
        for slice in slices.0.iter() {
            if slice.time > timeline_current_time && slice.time <= time {
                timeline.apply_time_slice(slice, None as Option<&DummyDatabaseModule>);
                applied += 1;
            } else if slice.time > time {
                break;
            }
        }
    } else {
        *timeline = AetTimelineState::new();
        for slice in slices.0.iter() {
            if slice.time <= time {
                timeline.apply_time_slice(slice, None as Option<&DummyDatabaseModule>);
                applied += 1;
            } else if slice.time > time {
                break;
            }
        }
    }
    applied
}

/**
 * The UI state will be much less rich than the full timeline state.
 */
#[derive(Serialize)]
pub struct UiTimelineState {
    pub time: i32,
    // My agent state and theirs!
}

#[wasm_bindgen]
pub fn get_current_state(timeline: &WasmTimeline, me: &str, you: &str) -> JsValue {
    let timeline = &timeline.0;
    let ui_state = UiTimelineState {
        time: timeline.time,
    };
    serde_wasm_bindgen::to_value(&ui_state).unwrap()
}
