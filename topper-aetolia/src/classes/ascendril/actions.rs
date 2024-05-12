use crate::bt::*;
use serde::*;
use topper_bt::unpowered::*;

use crate::{
    classes::group::*, observables::*, targetted_action, timeline::*, types::*, untargetted_action,
};

targetted_action!(Spark, "cast spark {}");
targetted_action!(AshenFeet, "cast ashenfeet {}");
untargetted_action!(FireburstCast, "cast fireburst");
targetted_action!(Fireburst, "fireburst {}");
targetted_action!(Blazewhirl, "cast blazewhirl {}");
// targetted_action!(Conflagrate, "cast conflagrate {}");
untargetted_action!(Afterburn, "cast afterburn");
targetted_action!(Sunspot, "cast sunspot {}");
targetted_action!(Pyroclast, "cast pyroclast {}");
targetted_action!(Disintegrate, "cast disintegrate {}");
untargetted_action!(Coldsnap, "cast coldsnap");
targetted_action!(Drench, "cast drench {}");
targetted_action!(Iceray, "cast iceray {}");
targetted_action!(Glazeflow, "cast glazeflow {}");
// targetted_action!(Direfrost, "cast direfrost {}");
targetted_action!(Icicle, "cast icicle {}");
untargetted_action!(Shatter, "cast shatter");
targetted_action!(Crystalise, "cast crystalise {}");
untargetted_action!(Winterheart, "cast winterheart");
targetted_action!(Windlance, "cast windlance {}");
targetted_action!(Pressurize, "cast pressurize {}");
targetted_action!(Arcbolt, "cast arcbolt {}");
targetted_action!(Electrosphere, "cast electrosphere {}");
targetted_action!(Thunderclap, "cast thunderclap {}");
targetted_action!(Feedback, "cast feedback {}");
targetted_action!(Aeroblast, "cast aeroblast {}");
untargetted_action!(Stormwrath, "cast stormwrath");

untargetted_action!(Fulcrum, "fulcrum construct");
untargetted_action!(FulcrumExpand, "fulcrum expand");
untargetted_action!(Schism, "fulcrum schism on");
untargetted_action!(Imbalance, "fulcrum imbalance on");
untargetted_action!(Restore, "restore");
untargetted_action!(EnrichFire, "fulcrum enrich fire");
untargetted_action!(EnrichWater, "fulcrum enrich water");
untargetted_action!(EnrichAir, "fulcrum enrich air");
untargetted_action!(GlimpseFire, "fulcrum glimpse fire");
untargetted_action!(GlimpseWater, "fulcrum glimpse water");
untargetted_action!(GlimpseAir, "fulcrum glimpse air");
targetted_action!(Flare, "fulcrum flare {}");
targetted_action!(Emberbrand, "fulcrum branding {} emberbrand");
targetted_action!(Frostbrand, "fulcrum branding {} frostbrand");
targetted_action!(Thunderbrand, "fulcrum branding {} thunderbrand");
