use crate::{observables::ActiveTransition, targetted_action, untargetted_action};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlowAttack {
    pub attacks: Vec<ZealotComboAction>,
    pub target: String,
}

impl FlowAttack {
    pub fn new(attacks: Vec<ZealotComboAction>, target: &str) -> Self {
        FlowAttack {
            attacks,
            target: target.to_string(),
        }
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
