use serde::{Deserialize, Serialize};

use crate::{
    agent::*, classes::zealot::constants::SWAGGER_LIMIT, observables::ActiveTransition,
    targetted_action, untargetted_action,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZealotComboAction {
    // Punches
    PummelLeft,
    PummelRight,
    Palmforce,
    Clawtwist,
    DislocateLeftArm,
    DislocateRightArm,
    DislocateLeftLeg,
    DislocateRightLeg,
    Twinpress,
    Direblow,
    // Kicks
    WanekickLeft,
    WanekickRight,
    Sunkick,
    Risekick,
    HeelrushHead,
    HeelrushTorso,
    HeelrushLeftArm,
    HeelrushRightArm,
    HeelrushLeftLeg,
    HeelrushRightLeg,
    Edgekick,
    Dropkick,
    // Chains
    Jawcrack,
    Uprise,
    Wristlash,
    Anklepin,
    Descent,
    Trammel,
}

impl ZealotComboAction {
    pub fn flow_str(&self) -> &str {
        match self {
            // Punches
            ZealotComboAction::PummelLeft => "pummel left",
            ZealotComboAction::PummelRight => "pummel right",
            ZealotComboAction::Clawtwist => "clawtwist",
            ZealotComboAction::Twinpress => "twinpress",
            ZealotComboAction::Direblow => "direblow",
            ZealotComboAction::DislocateLeftArm => "dislocate left arm",
            ZealotComboAction::DislocateRightArm => "dislocate right arm",
            ZealotComboAction::DislocateLeftLeg => "dislocate left leg",
            ZealotComboAction::DislocateRightLeg => "dislocate right leg",
            ZealotComboAction::Palmforce => "palmforce strike",
            // Kicks
            ZealotComboAction::WanekickLeft => "wanekick left",
            ZealotComboAction::WanekickRight => "wanekick right",
            ZealotComboAction::Sunkick => "sunkick",
            ZealotComboAction::Risekick => "risekick",
            ZealotComboAction::HeelrushHead => "heelrush head",
            ZealotComboAction::HeelrushTorso => "heelrush torso",
            ZealotComboAction::HeelrushLeftArm => "heelrush left arm",
            ZealotComboAction::HeelrushRightArm => "heelrush right arm",
            ZealotComboAction::HeelrushLeftLeg => "heelrush left leg",
            ZealotComboAction::HeelrushRightLeg => "heelrush right leg",
            ZealotComboAction::Edgekick => "edgekick",
            ZealotComboAction::Dropkick => "dropkick",
            // Chains
            ZealotComboAction::Jawcrack => "jawcrack",
            ZealotComboAction::Uprise => "uprise",
            ZealotComboAction::Wristlash => "wristlash",
            ZealotComboAction::Anklepin => "anklepin",
            ZealotComboAction::Descent => "descent",
            ZealotComboAction::Trammel => "trammel",
        }
    }

    pub fn can_start_combo(&self) -> bool {
        match self {
            ZealotComboAction::Palmforce => false,
            ZealotComboAction::HeelrushHead
            | ZealotComboAction::HeelrushTorso
            | ZealotComboAction::HeelrushLeftArm
            | ZealotComboAction::HeelrushRightArm
            | ZealotComboAction::HeelrushLeftLeg
            | ZealotComboAction::HeelrushRightLeg => false,
            _ => true,
        }
    }

    pub fn check_action(&self, me: &AgentState, target: &AgentState) -> bool {
        match self {
            ZealotComboAction::DislocateLeftArm
            | ZealotComboAction::DislocateRightArm
            | ZealotComboAction::DislocateLeftLeg
            | ZealotComboAction::DislocateRightLeg => {
                if !me.arms_free() {
                    return false;
                }
            }
            ZealotComboAction::WanekickLeft
            | ZealotComboAction::WanekickRight
            | ZealotComboAction::Edgekick
            | ZealotComboAction::Sunkick => {
                if !me.legs_free() {
                    return false;
                }
            }
            _ => {}
        }
        match self {
            ZealotComboAction::PummelLeft
                | ZealotComboAction::PummelRight
                | ZealotComboAction::Palmforce
                | ZealotComboAction::Clawtwist
                | ZealotComboAction::DislocateLeftArm
                | ZealotComboAction::DislocateRightArm
                | ZealotComboAction::DislocateLeftLeg
                | ZealotComboAction::DislocateRightLeg
                | ZealotComboAction::Twinpress
                | ZealotComboAction::Direblow
                 => {
                if me.get_count(FType::SappedStrength) >= SWAGGER_LIMIT {
                    return false;
                }
            }
            ZealotComboAction::Risekick => {
                if !me.is(FType::Fallen) {
                    return false;
                }
            }
            ZealotComboAction::Dropkick => {
                if !target.is(FType::Fallen)
                    || !target.get_limb_state(LType::LeftArmDamage).amputated
                    || !target.get_limb_state(LType::RightArmDamage).amputated
                {
                    return false;
                }
            }
            ZealotComboAction::Anklepin
            | ZealotComboAction::Jawcrack
            | ZealotComboAction::Descent
            | ZealotComboAction::Wristlash
            | ZealotComboAction::Trammel
            // | ZealotComboAction::Rive
            // | ZealotComboAction::Whipburst
            | ZealotComboAction::Uprise  => {
                if me.get_balance(BType::Secondary) < 2. {
                    return false;
                }
            }
            _ => {}
        };
        match self {
            ZealotComboAction::DislocateLeftArm => {
                let limb_state = target.get_limb_state(LType::LeftArmDamage);
                if limb_state.broken || limb_state.is_dislocated {
                    return false;
                }
            }
            ZealotComboAction::DislocateRightArm => {
                let limb_state = target.get_limb_state(LType::RightArmDamage);
                if limb_state.broken || limb_state.is_dislocated {
                    return false;
                }
            }
            ZealotComboAction::DislocateLeftLeg => {
                let limb_state = target.get_limb_state(LType::LeftLegDamage);
                if limb_state.broken || limb_state.is_dislocated {
                    return false;
                }
            }
            ZealotComboAction::DislocateRightLeg => {
                let limb_state = target.get_limb_state(LType::RightLegDamage);
                if limb_state.broken || limb_state.is_dislocated {
                    return false;
                }
            }
            _ => {}
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowAttack {
    pub attacks: Vec<ZealotComboAction>,
    pub target: String,
}

impl FlowAttack {
    pub fn new(attacks: Vec<ZealotComboAction>, target: String) -> Self {
        FlowAttack { attacks, target }
    }

    pub fn to_string(&self) -> String {
        let attack_strs: Vec<&str> = self.attacks.iter().map(|a| a.flow_str()).collect();
        format!("flow {} {}", self.target, attack_strs.join(" "))
    }
}

impl ActiveTransition for FlowAttack {
    fn act(&self, _timline: &crate::timeline::AetTimeline) -> crate::observables::ActivateResult {
        Ok(self.to_string())
    }
}

untargetted_action!(Wrath, "wrath");
untargetted_action!(Swagger, "swagger");
untargetted_action!(Firefist, "enact firefist");
untargetted_action!(RespirationHold, "respiration hold");
untargetted_action!(PsiRecover, "psi recover");
untargetted_action!(Zenith, "enact zenith");
untargetted_action!(Pyromania, "enact pyromania");
untargetted_action!(PsiTorrent, "psi torrent");

targetted_action!(Cinderkin, "order hellcat attack {}");
targetted_action!(Immolation, "enact immolation {}");
targetted_action!(PsiDisableAeon, "psi disable {} tarot aeon");
targetted_action!(PsiDull, "psi dull {}");
targetted_action!(PsiShock, "psi shock {}");
targetted_action!(Pendulum, "enact pendulum {}");
targetted_action!(PendulumReverse, "enact pendulum {} reverse");
targetted_action!(Scorch, "enact scorch {}");
targetted_action!(Heatspear, "enact heatspear {}");
targetted_action!(Quicken, "enact quicken {}");
targetted_action!(Infernal, "enact infernal {}");
targetted_action!(HacklesWhipburst, "hackles {} whipburst");
targetted_action!(HacklesJawcrack, "hackles {} jawcrack");
targetted_action!(HacklesUprise, "hackles {} uprise");
targetted_action!(HacklesWristlash, "hackles {} wristlash");
targetted_action!(HacklesAnklepin, "hackles {} anklepin");
targetted_action!(HacklesDescent, "hackles {} descent");
targetted_action!(HacklesTrammel, "hackles {} trammel");
