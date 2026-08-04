mod utils;

use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use topper_aetolia::classes::Class;
use topper_aetolia::classes::ascendril::*;
use topper_aetolia::observables::{ActiveTransition, ProbableEvent};
use topper_aetolia::timeline::{
    AetObservation, AetTimeline, AetTimelineStateTrait, CombatAction, simulation_slice,
};
use topper_aetolia::types::{BType, FType, SType};
use topper_core::timeline::db::DummyDatabaseModule;
use wasm_bindgen::prelude::*;

const LEARNER_NAME: &str = "Learner";
const TARGET_NAME: &str = "Target";

#[derive(Clone, Copy)]
struct ActionDef {
    id: &'static str,
    label: &'static str,
    targeted: bool,
    builder: fn(String, String) -> Box<dyn ActiveTransition>,
}

#[derive(Clone, Copy)]
struct PassiveDef {
    id: &'static str,
    label: &'static str,
    category: &'static str,
    skill: &'static str,
    targeted: bool,
    period_ms: i32,
}

macro_rules! targeted_builder {
    ($fn_name:ident, $action_type:ident) => {
        fn $fn_name(caster: String, target: String) -> Box<dyn ActiveTransition> {
            $action_type::boxed(caster, target)
        }
    };
}

macro_rules! untargeted_builder {
    ($fn_name:ident, $action_type:ident) => {
        fn $fn_name(caster: String, _target: String) -> Box<dyn ActiveTransition> {
            $action_type::boxed(caster)
        }
    };
}

targeted_builder!(build_spark, Spark);
targeted_builder!(build_ashenfeet, AshenFeet);
untargeted_builder!(build_fireburst_cast, FireburstCast);
targeted_builder!(build_fireburst, Fireburst);
targeted_builder!(build_blazewhirl, Blazewhirl);
targeted_builder!(build_conflagrate, Conflagrate);
untargeted_builder!(build_afterburn, Afterburn);
targeted_builder!(build_sunspot, Sunspot);
targeted_builder!(build_pyroclast, Pyroclast);
targeted_builder!(build_disintegrate, Disintegrate);
untargeted_builder!(build_coldsnap, Coldsnap);
targeted_builder!(build_drench, Drench);
targeted_builder!(build_iceray, Iceray);
targeted_builder!(build_glazeflow, Glazeflow);
targeted_builder!(build_direfrost, Direfrost);
targeted_builder!(build_icicle, Icicle);
untargeted_builder!(build_shatter, Shatter);
targeted_builder!(build_crystalise, Crystalise);
untargeted_builder!(build_winterheart, Winterheart);
targeted_builder!(build_windlance, Windlance);
targeted_builder!(build_pressurize, Pressurize);
targeted_builder!(build_arcbolt, Arcbolt);
targeted_builder!(build_electrosphere, Electrosphere);
targeted_builder!(build_thunderclap, Thunderclap);
targeted_builder!(build_feedback, Feedback);
targeted_builder!(build_aeroblast_fast, AeroblastFast);
targeted_builder!(build_aeroblast_slow, AeroblastSlow);
untargeted_builder!(build_stormwrath, Stormwrath);
untargeted_builder!(build_capacitance, Capacitance);
untargeted_builder!(build_fulcrum, Fulcrum);
untargeted_builder!(build_fulcrum_expand, FulcrumExpand);
untargeted_builder!(build_fulcrum_callback, FulcrumCallback);
untargeted_builder!(build_fulcrum_interfuse, FulcrumInterfuse);
untargeted_builder!(build_fulcrum_push, FulcrumPush);
untargeted_builder!(build_schism, Schism);
untargeted_builder!(build_imbalance, Imbalance);
untargeted_builder!(build_fulcrum_restore, FulcrumRestore);
untargeted_builder!(build_enrich_fire, EnrichFire);
untargeted_builder!(build_enrich_water, EnrichWater);
untargeted_builder!(build_enrich_air, EnrichAir);
targeted_builder!(build_emberbrand, Emberbrand);
targeted_builder!(build_frostbrand, Frostbrand);
targeted_builder!(build_thunderbrand, Thunderbrand);
targeted_builder!(build_catalyst_ember, CatalystEmber);
targeted_builder!(build_catalyst_frost, CatalystFrost);
targeted_builder!(build_catalyst_thunder, CatalystThunder);
targeted_builder!(build_enrapture, Enrapture);
targeted_builder!(build_fulcrum_detect, FulcrumDetect);
untargeted_builder!(build_shift, Shift);
untargeted_builder!(build_degradation, Degradation);
untargeted_builder!(build_spiritrift, Spiritrift);

const ASCENDRIL_ACTIONS: &[ActionDef] = &[
    ActionDef {
        id: "spark",
        label: "Spark",
        targeted: true,
        builder: build_spark,
    },
    ActionDef {
        id: "ashenfeet",
        label: "AshenFeet",
        targeted: true,
        builder: build_ashenfeet,
    },
    ActionDef {
        id: "fireburst_cast",
        label: "FireburstCast",
        targeted: false,
        builder: build_fireburst_cast,
    },
    ActionDef {
        id: "fireburst",
        label: "Fireburst",
        targeted: true,
        builder: build_fireburst,
    },
    ActionDef {
        id: "blazewhirl",
        label: "Blazewhirl",
        targeted: true,
        builder: build_blazewhirl,
    },
    ActionDef {
        id: "conflagrate",
        label: "Conflagrate",
        targeted: true,
        builder: build_conflagrate,
    },
    ActionDef {
        id: "afterburn",
        label: "Afterburn",
        targeted: false,
        builder: build_afterburn,
    },
    ActionDef {
        id: "sunspot",
        label: "Sunspot",
        targeted: true,
        builder: build_sunspot,
    },
    ActionDef {
        id: "pyroclast",
        label: "Pyroclast",
        targeted: true,
        builder: build_pyroclast,
    },
    ActionDef {
        id: "disintegrate",
        label: "Disintegrate",
        targeted: true,
        builder: build_disintegrate,
    },
    ActionDef {
        id: "coldsnap",
        label: "Coldsnap",
        targeted: false,
        builder: build_coldsnap,
    },
    ActionDef {
        id: "drench",
        label: "Drench",
        targeted: true,
        builder: build_drench,
    },
    ActionDef {
        id: "iceray",
        label: "Iceray",
        targeted: true,
        builder: build_iceray,
    },
    ActionDef {
        id: "glazeflow",
        label: "Glazeflow",
        targeted: true,
        builder: build_glazeflow,
    },
    ActionDef {
        id: "direfrost",
        label: "Direfrost",
        targeted: true,
        builder: build_direfrost,
    },
    ActionDef {
        id: "icicle",
        label: "Icicle",
        targeted: true,
        builder: build_icicle,
    },
    ActionDef {
        id: "shatter",
        label: "Shatter",
        targeted: false,
        builder: build_shatter,
    },
    ActionDef {
        id: "crystalise",
        label: "Crystalise",
        targeted: true,
        builder: build_crystalise,
    },
    ActionDef {
        id: "winterheart",
        label: "Winterheart",
        targeted: false,
        builder: build_winterheart,
    },
    ActionDef {
        id: "windlance",
        label: "Windlance",
        targeted: true,
        builder: build_windlance,
    },
    ActionDef {
        id: "pressurize",
        label: "Pressurize",
        targeted: true,
        builder: build_pressurize,
    },
    ActionDef {
        id: "arcbolt",
        label: "Arcbolt",
        targeted: true,
        builder: build_arcbolt,
    },
    ActionDef {
        id: "electrosphere",
        label: "Electrosphere",
        targeted: true,
        builder: build_electrosphere,
    },
    ActionDef {
        id: "thunderclap",
        label: "Thunderclap",
        targeted: true,
        builder: build_thunderclap,
    },
    ActionDef {
        id: "feedback",
        label: "Feedback",
        targeted: true,
        builder: build_feedback,
    },
    ActionDef {
        id: "aeroblast_fast",
        label: "AeroblastFast",
        targeted: true,
        builder: build_aeroblast_fast,
    },
    ActionDef {
        id: "aeroblast_slow",
        label: "AeroblastSlow",
        targeted: true,
        builder: build_aeroblast_slow,
    },
    ActionDef {
        id: "stormwrath",
        label: "Stormwrath",
        targeted: false,
        builder: build_stormwrath,
    },
    ActionDef {
        id: "capacitance",
        label: "Capacitance",
        targeted: false,
        builder: build_capacitance,
    },
    ActionDef {
        id: "fulcrum",
        label: "Fulcrum",
        targeted: false,
        builder: build_fulcrum,
    },
    ActionDef {
        id: "fulcrum_expand",
        label: "FulcrumExpand",
        targeted: false,
        builder: build_fulcrum_expand,
    },
    ActionDef {
        id: "fulcrum_callback",
        label: "FulcrumCallback",
        targeted: false,
        builder: build_fulcrum_callback,
    },
    ActionDef {
        id: "fulcrum_interfuse",
        label: "FulcrumInterfuse",
        targeted: false,
        builder: build_fulcrum_interfuse,
    },
    ActionDef {
        id: "fulcrum_push",
        label: "FulcrumPush",
        targeted: false,
        builder: build_fulcrum_push,
    },
    ActionDef {
        id: "schism",
        label: "Schism",
        targeted: false,
        builder: build_schism,
    },
    ActionDef {
        id: "imbalance",
        label: "Imbalance",
        targeted: false,
        builder: build_imbalance,
    },
    ActionDef {
        id: "fulcrum_restore",
        label: "FulcrumRestore",
        targeted: false,
        builder: build_fulcrum_restore,
    },
    ActionDef {
        id: "enrich_fire",
        label: "EnrichFire",
        targeted: false,
        builder: build_enrich_fire,
    },
    ActionDef {
        id: "enrich_water",
        label: "EnrichWater",
        targeted: false,
        builder: build_enrich_water,
    },
    ActionDef {
        id: "enrich_air",
        label: "EnrichAir",
        targeted: false,
        builder: build_enrich_air,
    },
    ActionDef {
        id: "emberbrand",
        label: "Emberbrand",
        targeted: true,
        builder: build_emberbrand,
    },
    ActionDef {
        id: "frostbrand",
        label: "Frostbrand",
        targeted: true,
        builder: build_frostbrand,
    },
    ActionDef {
        id: "thunderbrand",
        label: "Thunderbrand",
        targeted: true,
        builder: build_thunderbrand,
    },
    ActionDef {
        id: "catalyst_ember",
        label: "CatalystEmber",
        targeted: true,
        builder: build_catalyst_ember,
    },
    ActionDef {
        id: "catalyst_frost",
        label: "CatalystFrost",
        targeted: true,
        builder: build_catalyst_frost,
    },
    ActionDef {
        id: "catalyst_thunder",
        label: "CatalystThunder",
        targeted: true,
        builder: build_catalyst_thunder,
    },
    ActionDef {
        id: "enrapture",
        label: "Enrapture",
        targeted: true,
        builder: build_enrapture,
    },
    ActionDef {
        id: "fulcrum_detect",
        label: "FulcrumDetect",
        targeted: true,
        builder: build_fulcrum_detect,
    },
    ActionDef {
        id: "shift",
        label: "Shift",
        targeted: false,
        builder: build_shift,
    },
    ActionDef {
        id: "degradation",
        label: "Degradation",
        targeted: false,
        builder: build_degradation,
    },
    ActionDef {
        id: "spiritrift",
        label: "Spiritrift",
        targeted: false,
        builder: build_spiritrift,
    },
];

const ASCENDRIL_PASSIVES: &[PassiveDef] = &[
    PassiveDef {
        id: "ascendril_attunement",
        label: "Ascendril Attunement",
        category: "Passive",
        skill: "AscendrilAttunementPulse",
        targeted: false,
        period_ms: 50,
    },
    PassiveDef {
        id: "ascendril_flux",
        label: "Ascendril Flux",
        category: "Passive",
        skill: "AscendrilFluxPulse",
        targeted: true,
        period_ms: 100,
    },
];

#[derive(Serialize)]
struct ActionDescriptor {
    id: String,
    label: String,
    targeted: bool,
}

#[derive(Serialize)]
struct PassiveDescriptor {
    id: String,
    label: String,
    period_ms: i32,
    targeted: bool,
}

#[derive(Serialize)]
struct PlayerBattleStats {
    name: String,
    class_name: String,
    vitals: serde_json::Value,
    balances: serde_json::Value,
    afflictions: Vec<String>,
}

#[derive(Serialize)]
struct BattleStatsView {
    current_time: i32,
    learner: PlayerBattleStats,
    target: PlayerBattleStats,
}

#[derive(Serialize)]
struct ActionExecution {
    action_id: String,
    command: String,
    applied_time: i32,
    observations: Vec<String>,
    battle_stats: BattleStatsView,
}

#[derive(Serialize)]
struct EngineStateView {
    active_class: String,
    battle_stats: BattleStatsView,
}

#[wasm_bindgen]
pub struct LearnerEngine {
    timeline: AetTimeline,
    learner_class: String,
    rng_state: u64,
}

#[wasm_bindgen]
impl LearnerEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LearnerEngine {
        utils::set_panic_hook();

        let mut engine = LearnerEngine {
            timeline: AetTimeline::new(),
            learner_class: "Ascendril".to_string(),
            rng_state: 0x8d9a_b3c4_0f15_2e7d,
        };
        engine.timeline.state.me = LEARNER_NAME.to_string();
        engine.initialize_players();
        engine
    }

    #[wasm_bindgen]
    pub fn get_supported_classes(&self) -> Vec<String> {
        vec!["Ascendril".to_string()]
    }

    #[wasm_bindgen]
    pub fn set_active_class(&mut self, class_name: &str) -> Result<JsValue, JsValue> {
        if class_name != "Ascendril" {
            return Err(JsValue::from_str("Only Ascendril is currently supported."));
        }
        self.learner_class = class_name.to_string();
        self.initialize_players();
        self.get_state()
    }

    #[wasm_bindgen]
    pub fn get_actions(&self) -> Result<JsValue, JsValue> {
        let actions = actions_for_class(&self.learner_class)
            .iter()
            .map(|action| ActionDescriptor {
                id: action.id.to_string(),
                label: action.label.to_string(),
                targeted: action.targeted,
            })
            .collect::<Vec<_>>();
        serde_wasm_bindgen::to_value(&actions).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_passives(&self) -> Result<JsValue, JsValue> {
        let passives = passives_for_class(&self.learner_class)
            .iter()
            .map(|passive| PassiveDescriptor {
                id: passive.id.to_string(),
                label: passive.label.to_string(),
                period_ms: passive.period_ms,
                targeted: passive.targeted,
            })
            .collect::<Vec<_>>();
        serde_wasm_bindgen::to_value(&passives).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn simulate_action(&mut self, action_id: &str) -> Result<JsValue, JsValue> {
        let action = actions_for_class(&self.learner_class)
            .iter()
            .find(|action| action.id == action_id)
            .ok_or_else(|| JsValue::from_str("Unknown action id."))?;

        let transition = (action.builder)(LEARNER_NAME.to_string(), TARGET_NAME.to_string());
        let command = transition
            .act(&self.timeline)
            .map_err(|e| JsValue::from_str(&e))?;

        let simulated = catch_unwind(AssertUnwindSafe(|| transition.simulate(&self.timeline)))
            .map_err(|_| JsValue::from_str("Action simulate panicked for this action."))?;
        let event = self
            .select_weighted_event(&simulated)
            .ok_or_else(|| JsValue::from_str("Action simulate produced no probable events."))?;

        let mut observations = event.observations().clone();

        let start_time = self.timeline.state.time;
        let next_time = start_time + 1;
        observations.extend(self.collect_passive_observations(start_time, next_time));
        self.apply_observations(next_time, observations.clone())?;

        let result = ActionExecution {
            action_id: action.id.to_string(),
            command,
            applied_time: self.timeline.state.time,
            observations: observations
                .iter()
                .map(|observation| format!("{observation:?}"))
                .collect(),
            battle_stats: self.build_battle_stats(),
        };

        serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn advance_by(&mut self, delta_ms: i32) -> Result<JsValue, JsValue> {
        let clamped = delta_ms.max(0);
        let start_time = self.timeline.state.time;
        let next_time = start_time + clamped;
        let passive_observations = self.collect_passive_observations(start_time, next_time);
        self.apply_observations(next_time, passive_observations)?;
        self.get_state()
    }

    #[wasm_bindgen]
    pub fn advance_to_qeb(&mut self) -> Result<JsValue, JsValue> {
        let learner = self.timeline.state.borrow_agent(&LEARNER_NAME.to_string());
        let qeb_remaining = learner
            .get_raw_balance(BType::Balance)
            .max(learner.get_raw_balance(BType::Equil))
            .max(0);
        self.advance_by(qeb_remaining)
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> Result<JsValue, JsValue> {
        let state = EngineStateView {
            active_class: self.learner_class.clone(),
            battle_stats: self.build_battle_stats(),
        };
        serde_wasm_bindgen::to_value(&state).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl LearnerEngine {
    fn next_u32(&mut self) -> u32 {
        // xorshift64* variant; deterministic and wasm-safe.
        self.rng_state ^= self.rng_state >> 12;
        self.rng_state ^= self.rng_state << 25;
        self.rng_state ^= self.rng_state >> 27;
        let mixed = self.rng_state.wrapping_mul(0x2545_F491_4F6C_DD1D);
        (mixed >> 32) as u32
    }

    fn select_weighted_event<'a>(
        &mut self,
        events: &'a [ProbableEvent],
    ) -> Option<&'a ProbableEvent> {
        if events.is_empty() {
            return None;
        }

        let total_weight: u32 = events.iter().map(|event| event.weight()).sum();
        if total_weight == 0 {
            return events.first();
        }

        let mut roll = self.next_u32() % total_weight;
        for event in events {
            let weight = event.weight();
            if roll < weight {
                return Some(event);
            }
            roll -= weight;
        }

        events.last()
    }

    fn initialize_players(&mut self) {
        self.timeline.state.me = LEARNER_NAME.to_string();
        let class = Class::from_str(&self.learner_class).unwrap_or(Class::Ascendril);
        let learner_name = LEARNER_NAME.to_string();
        let target_name = TARGET_NAME.to_string();

        self.timeline.state.for_agent(&learner_name, &|agent| {
            agent.class_state.initialize_for_class(class);
            agent.set_known_stat(SType::Health, 5000, 5000, 0);
            agent.set_known_stat(SType::Mana, 5000, 5000, 0);
            agent.set_known_stat(SType::SP, 3000, 3000, 0);
            agent.set_balance(BType::Balance, 0.0);
            agent.set_balance(BType::Equil, 0.0);
        });

        self.timeline.state.for_agent(&target_name, &|agent| {
            agent.class_state.initialize_for_class(class);
            agent.set_known_stat(SType::Health, 5000, 5000, 0);
            agent.set_known_stat(SType::Mana, 5000, 5000, 0);
            agent.set_known_stat(SType::SP, 3000, 3000, 0);
            agent.set_balance(BType::Balance, 0.0);
            agent.set_balance(BType::Equil, 0.0);
        });
    }

    fn apply_observations(
        &mut self,
        time: i32,
        observations: Vec<AetObservation>,
    ) -> Result<(), JsValue> {
        let next_time = time.max(self.timeline.state.time);
        self.timeline
            .state
            .update_time_with_stats(next_time)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;

        let mut slice = simulation_slice(observations, next_time);
        slice.me = LEARNER_NAME.to_string();
        self.timeline
            .state
            .apply_time_slice(&slice, None as Option<&DummyDatabaseModule>)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;
        Ok(())
    }

    fn collect_passive_observations(&self, start_time: i32, end_time: i32) -> Vec<AetObservation> {
        if end_time <= start_time {
            return Vec::new();
        }

        let mut observations = Vec::new();
        for passive in passives_for_class(&self.learner_class) {
            if passive.period_ms <= 0 {
                continue;
            }

            let mut t = ((start_time / passive.period_ms) + 1) * passive.period_ms;
            while t <= end_time {
                observations.push(CombatAction::observation(
                    LEARNER_NAME,
                    passive.category,
                    passive.skill,
                    "passive",
                    if passive.targeted { TARGET_NAME } else { "" },
                ));
                t += passive.period_ms;
            }
        }
        observations
    }

    fn build_battle_stats(&self) -> BattleStatsView {
        BattleStatsView {
            current_time: self.timeline.state.time,
            learner: self.player_battle_stats(LEARNER_NAME),
            target: self.player_battle_stats(TARGET_NAME),
        }
    }

    fn player_battle_stats(&self, name: &str) -> PlayerBattleStats {
        let player = self.timeline.state.borrow_agent(&name.to_string());
        let class_name = player
            .class_state
            .get_normalized_class()
            .map(|class| class.to_str().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let mut afflictions = player
            .flags
            .aff_iter()
            .map(|aff: FType| aff.to_name())
            .collect::<Vec<String>>();
        afflictions.sort();

        let mut balances = HashMap::new();
        balances.insert("balance".to_string(), player.get_balance(BType::Balance));
        balances.insert("equilibrium".to_string(), player.get_balance(BType::Equil));
        balances.insert("tree".to_string(), player.get_balance(BType::Tree));
        balances.insert("focus".to_string(), player.get_balance(BType::Focus));
        balances.insert("fitness".to_string(), player.get_balance(BType::Fitness));
        balances.insert(
            "class_cure_1".to_string(),
            player.get_balance(BType::ClassCure1),
        );
        balances.insert(
            "class_cure_2".to_string(),
            player.get_balance(BType::ClassCure2),
        );
        balances.insert(
            "secondary".to_string(),
            player.get_balance(BType::Secondary),
        );
        balances.insert("qeb".to_string(), player.get_qeb_balance());

        PlayerBattleStats {
            name: name.to_string(),
            class_name,
            vitals: json!({
                "health": player.get_stat(SType::Health),
                "health_percent": player.get_health_percent(),
                "max_health": player.get_max_stat(SType::Health),
                "mana": player.get_stat(SType::Mana),
                "mana_percent": player.get_mana_percent(),
                "max_mana": player.get_max_stat(SType::Mana),
                "sp": player.get_stat(SType::SP),
                "sp_percent": player.get_stat_percent(SType::SP),
                "max_sp": player.get_max_stat(SType::SP),
            }),
            balances: json!(balances),
            afflictions,
        }
    }
}

fn actions_for_class(class_name: &str) -> &'static [ActionDef] {
    match class_name {
        "Ascendril" => ASCENDRIL_ACTIONS,
        _ => ASCENDRIL_ACTIONS,
    }
}

fn passives_for_class(class_name: &str) -> &'static [PassiveDef] {
    match class_name {
        "Ascendril" => ASCENDRIL_PASSIVES,
        _ => ASCENDRIL_PASSIVES,
    }
}
