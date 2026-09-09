use topper_aetolia::classes::ascendril::*;
use topper_aetolia::observables::ActiveTransition;

use crate::models::ActionDef;

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

pub(crate) fn actions_for_class(class_name: &str) -> &'static [ActionDef] {
    match class_name {
        "Ascendril" => ASCENDRIL_ACTIONS,
        _ => ASCENDRIL_ACTIONS,
    }
}
