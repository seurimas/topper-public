use crate::classes::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use crate::{targetted_action, untargetted_action};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parallax {
    pub caster: String,
    pub time: CType,
    pub spell: String,
    pub target: Option<String>,
}

impl ActiveTransition for Parallax {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "astra parallax {} {} {}",
            self.time,
            self.spell,
            self.target.as_ref().unwrap_or(&"".to_string())
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GleamInflict {
    pub caster: String,
    pub target: String,
    pub color: GleamColor,
}

impl ActiveTransition for GleamInflict {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "astra inflict {} {}",
            self.target,
            self.color.name()
        ))
    }
}

untargetted_action!(Luminesce, "astra luminesce");
targetted_action!(Ray, "astra ray {}");
untargetted_action!(Blueshift, "astra blueshift");
targetted_action!(Erode, "astra erode {}");
untargetted_action!(Rotate, "astra rotate");
untargetted_action!(Enigma, "astra enigma");
targetted_action!(EnigmaAttack, "order glimmercrest attack {}");
targetted_action!(Dustring, "astra dustring {}");
untargetted_action!(Gleam, "astra gleam");
untargetted_action!(Eventide, "astra eventide");
targetted_action!(Asterism, "astra asterism {}");
untargetted_action!(Foresight, "astra foresight");
untargetted_action!(Embody, "astra embody");
targetted_action!(EmbodyAttack, "order sprite attack {}");
untargetted_action!(Centrum, "astra centrum");
targetted_action!(Moonlet, "astra moonlet {}");
untargetted_action!(Equinox, "astra equinox");
targetted_action!(Stillness, "astra stillness {}");
targetted_action!(Redshift, "astra redshift {}");
targetted_action!(Chromaflare, "astra chromaflare {}");
targetted_action!(Syzygy, "astra syzygy {}");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reenactment {
    pub caster: String,
    pub regalia: Regalia,
}

impl ActiveTransition for Reenactment {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("reenact {}", self.regalia.name()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Forfeit {
    pub caster: String,
    pub regalia: Regalia,
}

impl ActiveTransition for Forfeit {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("forfeit {}", self.regalia.regalia_type()))
    }
}

targetted_action!(Illgrasp, "illgrasp {}");
targetted_action!(BoltHead, "bolt {} head");
targetted_action!(BoltTorso, "bolt {} torso");
targetted_action!(BoltLeftArm, "bolt {} left arm");
targetted_action!(BoltRightArm, "bolt {} right arm");
targetted_action!(BoltLeftLeg, "bolt {} left leg");
targetted_action!(BoltRightLeg, "bolt {} right leg");
untargetted_action!(EjaKodosaMend, "speak mend me");
targetted_action!(EjaKodosaKill, "speak kill {}");
