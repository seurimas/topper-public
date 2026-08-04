use crate::bt::*;
use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    classes::group::*, observables::*, targetted_action, timeline::*, types::*, untargetted_action,
};

targetted_action!(Spark, "cast spark {}", categories: ("Elemancy", "Humourism"));
targetted_action!(AshenFeet, "cast ashenfeet {}", categories: ("Elemancy", "Humourism"));
untargetted_action!(FireburstCast, "cast fireburst", categories: ("Elemancy", "Humourism"));
targetted_action!(Fireburst, "fireburst {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Blazewhirl, "cast blazewhirl {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Conflagrate, "cast conflagrate {}", categories: ("Elemancy", "Humourism"));
untargetted_action!(Afterburn, "cast afterburn", categories: ("Elemancy", "Humourism"));
targetted_action!(Sunspot, "cast sunspot {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Pyroclast, "cast pyroclast {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Disintegrate, "cast disintegrate {}", categories: ("Elemancy", "Humourism"));
untargetted_action!(Coldsnap, "cast coldsnap", categories: ("Elemancy", "Humourism"));
targetted_action!(Drench, "cast drench {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Iceray, "cast iceray {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Glazeflow, "cast glazeflow {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Direfrost, "cast direfrost {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Icicle, "cast icicle {}", categories: ("Elemancy", "Humourism"));
untargetted_action!(Shatter, "cast shatter", categories: ("Elemancy", "Humourism"));
targetted_action!(Crystalise, "cast crystalise {}", categories: ("Elemancy", "Humourism"));
untargetted_action!(Winterheart, "cast winterheart", categories: ("Elemancy", "Humourism"));
targetted_action!(Windlance, "cast windlance {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Pressurize, "cast pressurize {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Arcbolt, "cast arcbolt {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Electrosphere, "cast electrosphere {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Thunderclap, "cast thunderclap {}", categories: ("Elemancy", "Humourism"));
targetted_action!(Feedback, "cast feedback {}", categories: ("Elemancy", "Humourism"));
targetted_action!(AeroblastFast, "cast aeroblast {} fast", categories: ("Elemancy", "Humourism"));
targetted_action!(AeroblastSlow, "cast aeroblast {} slow", categories: ("Elemancy", "Humourism"));
untargetted_action!(Stormwrath, "cast stormwrath", categories: ("Elemancy", "Humourism"));
untargetted_action!(Capacitance, "cast capacitance", categories: ("Elemancy", "Humourism"));

untargetted_action!(Fulcrum, "fulcrum construct", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(FulcrumExpand, "fulcrum expand", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(FulcrumCallback, "fulcrum callback", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(FulcrumInterfuse, "fulcrum interfuse", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(FulcrumPush, "fulcrum push", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(Schism, "fulcrum schism on", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(Imbalance, "fulcrum imbalance on", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(FulcrumRestore, "fulcrum restore", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(EnrichFire, "fulcrum enrich fire", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(EnrichWater, "fulcrum enrich water", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(EnrichAir, "fulcrum enrich air", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(Emberbrand, "fulcrum branding {} ember", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(Frostbrand, "fulcrum branding {} frost", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(Thunderbrand, "fulcrum branding {} thunder", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(CatalystEmber, "fulcrum catalyst {} ember", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(CatalystFrost, "fulcrum catalyst {} frost", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(CatalystThunder, "fulcrum catalyst {} thunder", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(Enrapture, "fulcrum enrapture {}", categories: ("Thaumaturgy", "Hematurgy"));
targetted_action!(FulcrumDetect, "fulcrum detect {}", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(Shift, "fulcrum shift", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(Degradation, "fulcrum degradation on", categories: ("Thaumaturgy", "Hematurgy"));
untargetted_action!(Spiritrift, "fulcrum spiritrift on", categories: ("Thaumaturgy", "Hematurgy"));

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Glyph {
    Portal,
    Destruction,
    Vainglory,
    Addling,
    Entrapment,
    Slumber,
    Arrogance,
    Leaching,
    Gravity,
    Clarity,
}

impl Glyph {
    pub fn to_skill(&self) -> String {
        match self {
            Glyph::Portal => "Portal Glyph",
            Glyph::Destruction => "Destruction Glyph",
            Glyph::Vainglory => "Vainglory Glyph",
            Glyph::Addling => "Addling Glyph",
            Glyph::Entrapment => "Entrapment Glyph",
            Glyph::Slumber => "Slumber Glyph",
            Glyph::Arrogance => "Arrogance Glyph",
            Glyph::Leaching => "Leaching Glyph",
            Glyph::Gravity => "Gravity Glyph",
            Glyph::Clarity => "Clarity Glyph",
        }
        .to_string()
    }

    pub fn name(&self) -> String {
        match self {
            Glyph::Portal => "PORTAL",
            Glyph::Destruction => "DESTRUCTION",
            Glyph::Vainglory => "VAINGLORY",
            Glyph::Addling => "ADDLING",
            Glyph::Entrapment => "ENTRAPMENT",
            Glyph::Slumber => "SLUMBER",
            Glyph::Arrogance => "ARROGANCE",
            Glyph::Leaching => "LEACHING",
            Glyph::Gravity => "GRAVITY",
            Glyph::Clarity => "CLARITY",
        }
        .to_string()
    }

    pub fn from_name(name: impl ToString) -> Self {
        match name.to_string().to_uppercase().as_str() {
            "PORTAL" => Glyph::Portal,
            "DESTRUCTION" => Glyph::Destruction,
            "VAINGLORY" => Glyph::Vainglory,
            "ADDLING" => Glyph::Addling,
            "ENTRAPMENT" => Glyph::Entrapment,
            "SLUMBER" => Glyph::Slumber,
            "ARROGANCE" => Glyph::Arrogance,
            "LEACHING" => Glyph::Leaching,
            "GRAVITY" => Glyph::Gravity,
            "CLARITY" => Glyph::Clarity,
            _ => Glyph::Portal, // Default to Portal if unknown, though ideally this should be handled more robustly
        }
    }
}

pub struct GlyphTraceAction {
    pub caster: String,
    pub traced: Glyph,
    pub aegis: bool,
    pub target: Option<String>,
}

impl GlyphTraceAction {
    pub fn new(caster: String, traced: Glyph) -> Self {
        GlyphTraceAction {
            caster,
            traced,
            aegis: false,
            target: None,
        }
    }

    pub fn with_aegis(mut self) -> Self {
        self.aegis = true;
        self
    }

    pub fn with_target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }
}

impl ActiveTransition for GlyphTraceAction {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let category = if timeline
            .state
            .borrow_agent(&self.caster)
            .class_state
            .is_mirrored()
        {
            "Esoterica"
        } else {
            "Arcanism"
        };
        let mut observations = vec![CombatAction::observation(
            &self.caster,
            category,
            &self.traced.to_skill(),
            &"",
            &self.target.clone().unwrap_or("".to_string()),
        )];
        ProbableEvent::certain(observations)
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        let root = match &self.target {
            Some(target) => format!("trace {} {}", self.traced.name(), target),
            None => format!("trace {}", self.traced.name()),
        };
        if self.aegis {
            Ok(format!("{} onto aegis", root))
        } else {
            Ok(root)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TwinnedArcbolt {
    pub caster: String,
    pub targets: (String, String),
}

impl TwinnedArcbolt {
    pub fn from_target_and_phenomenon(
        target: &crate::classes::AetTarget,
        model: &crate::classes::BehaviorModel,
        controller: &crate::classes::BehaviorController,
        id: i64,
    ) -> Self {
        let target_name = target.get_name(model, controller);
        TwinnedArcbolt {
            caster: model.who_am_i(),
            targets: (target_name, id.to_string()),
        }
    }
}

impl ActiveTransition for TwinnedArcbolt {
    fn simulate(&self, timeline: &AetTimeline) -> Vec<ProbableEvent> {
        let category = if timeline
            .state
            .borrow_agent(&self.caster)
            .class_state
            .is_mirrored()
        {
            "Humourism"
        } else {
            "Elemancy"
        };
        let mut observations = vec![
            CombatAction::observation(&self.caster, category, &"Arcbolt", &"", &self.targets.0),
            CombatAction::observation(&self.caster, category, &"Arcbolt", &"", &self.targets.1),
        ];
        ProbableEvent::certain(observations)
    }

    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "cast arcbolt {} {}",
            self.targets.0, self.targets.1
        ))
    }
}
