use crate::classes::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use crate::{targetted_action, untargetted_action};

targetted_action!(Refract, "refract {}");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Embed {
    pub caster: String,
    pub vibration: Vibration,
}

impl Embed {
    pub fn new(caster: String, vibration: Vibration) -> Self {
        Self { caster, vibration }
    }
}

impl ActiveTransition for Embed {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("embed {}", self.vibration))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tones {
    pub caster: String,
    pub target: String,
    pub vibration: Vibration,
}

impl Tones {
    pub fn from_target(
        aet_target: AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
        vibration: Vibration,
    ) -> Self {
        let caster = model.who_am_i();
        let target = aet_target.get_name(model, controller);
        Self {
            caster,
            target,
            vibration,
        }
    }
}

impl ActiveTransition for Tones {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("strike tone {} {}", self.vibration, self.target,))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parallax {
    pub caster: String,
    pub time: CType,
    pub spell: String,
    pub target: Option<String>,
    pub ab: Option<String>,
}

impl Parallax {
    pub fn new_no_target(caster: String, time: CType, spell: String) -> Self {
        Self {
            caster,
            time,
            spell,
            target: None,
            ab: None,
        }
    }

    pub fn new_with_target(caster: String, time: CType, spell: String, target: String) -> Self {
        Self {
            caster,
            time,
            spell,
            target: Some(target),
            ab: None,
        }
    }

    pub fn new_with_target_and_ab(
        caster: String,
        time: CType,
        spell: String,
        target: String,
        ab: String,
    ) -> Self {
        Self {
            caster,
            time,
            spell,
            target: Some(target),
            ab: Some(ab),
        }
    }
}

impl ActiveTransition for Parallax {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "astra parallax {} {} {} {}",
            self.time,
            self.spell,
            self.target.as_ref().unwrap_or(&"".to_string()),
            self.ab.as_ref().unwrap_or(&"".to_string())
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alteration {
    pub caster: String,
    pub target: String,
    pub source: FType,
    pub result: FType,
}

impl Alteration {
    pub fn from_target(
        aet_target: AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
        source: FType,
        result: FType,
    ) -> Self {
        let caster = model.who_am_i();
        let target = aet_target.get_name(model, controller);
        Self {
            caster,
            target,
            source,
            result,
        }
    }
}

impl ActiveTransition for Alteration {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!(
            "astra alteration {} {} to {}",
            self.target, self.source, self.result
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GleamInflict {
    pub caster: String,
    pub target: String,
    pub color: GleamColor,
}

impl GleamInflict {
    pub fn from_target(
        aet_target: AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
        color: GleamColor,
    ) -> Self {
        let caster = model.who_am_i();
        let target = aet_target.get_name(model, controller);
        Self {
            caster,
            target,
            color,
        }
    }
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

impl Reenactment {
    pub fn new(caster: String, regalia: Regalia) -> Self {
        Self { caster, regalia }
    }
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

impl Forfeit {
    pub fn new(caster: String, regalia: Regalia) -> Self {
        Self { caster, regalia }
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bolt {
    pub caster: String,
    pub target: String,
    pub limb: LType,
}

impl Bolt {
    pub fn from_target(
        aet_target: AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
        limb: LType,
    ) -> Self {
        let caster = model.who_am_i();
        let target = aet_target.get_name(model, controller);
        Self {
            caster,
            target,
            limb,
        }
    }
}

impl ActiveTransition for Bolt {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("bolt {} {}", self.target, self.limb.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VayuaAttack {
    pub caster: String,
    pub target: String,
    pub venom: String,
}

impl VayuaAttack {
    pub fn from_target(
        aet_target: AetTarget,
        model: &BehaviorModel,
        controller: &BehaviorController,
        venom: String,
    ) -> Self {
        let caster = model.who_am_i();
        let target = aet_target.get_name(model, controller);
        Self {
            caster,
            target,
            venom,
        }
    }
}

impl ActiveTransition for VayuaAttack {
    fn act(&self, _timeline: &AetTimeline) -> ActivateResult {
        Ok(format!("jab {} {}", self.target, self.venom))
    }
}

untargetted_action!(EjaKodosaMend, "speak mend me");
targetted_action!(EjaKodosaKill, "speak kill {}");
