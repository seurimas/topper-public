use super::constants::*;
use crate::classes::group::with_call_venom_double;
use crate::classes::group::with_call_venom_single;
use crate::classes::*;
use crate::timeline::*;
use crate::types::*;
use crate::untargetted_action;

untargetted_action!(
    CallWisp,
    "call wisp",
    "fabricate lurker",
    "CallWisp",
    "CallWisp"
);
untargetted_action!(
    CallWeasel,
    "call weasel",
    "fabricate wardpeeler",
    "CallWeasel",
    "CallWeasel"
);
untargetted_action!(
    CallNightingale,
    "call nightingale",
    "fabricate lightdrinker",
    "CallNightingale",
    "CallNightingale"
);
untargetted_action!(
    CallRook,
    "call rook",
    "fabricate murder",
    "CallRook",
    "CallRook"
);
untargetted_action!(
    CallCoyote,
    "call coyote",
    "fabricate darkhound",
    "CallCoyote",
    "CallCoyote"
);
untargetted_action!(
    CallRaccoon,
    "call raccoon",
    "fabricate pilferer",
    "CallRaccoon",
    "CallRaccoon"
);
untargetted_action!(
    CallElk,
    "call elk",
    "fabricate monstrosity",
    "CallElk",
    "CallElk"
);
untargetted_action!(
    CallGyrfalcon,
    "call gyrfalcon",
    "fabricate throatripper",
    "CallGyrfalcon",
    "CallGyrfalcon"
);
untargetted_action!(
    CallRaloth,
    "call raloth",
    "fabricate brutaliser",
    "CallRaloth",
    "CallRaloth"
);
untargetted_action!(
    CallCrocodile,
    "call crocodile",
    "fabricate eviscerator",
    "CallCrocodile",
    "CallCrocodile"
);
untargetted_action!(
    CallIcewyrm,
    "call icewyrm",
    "fabricate rimestalker",
    "CallIcewyrm",
    "CallIcewyrm"
);
untargetted_action!(
    CallCockatrice,
    "call cockatrice",
    "fabricate terrifier",
    "CallCockatrice",
    "CallCockatrice"
);
targetted_action!(
    Icebreath,
    "order icewyrm icebreath {}",
    "order rimestalker verglas {}",
    "Icebreath",
    "Icebreath"
);

targetted_action!(
    HurlPyrolum,
    "resin hurl pyrolum at {}",
    "tokin splatter flammable at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlCorsin,
    "resin hurl corsin at {}",
    "tokin splatter coagulating at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlTrientia,
    "resin hurl trientia at {}",
    "tokin splatter hallucinatory at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlHarimel,
    "resin hurl harimel at {}",
    "tokin splatter adhesive at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlGlauxe,
    "resin hurl glauxe at {}",
    "tokin splatter choking at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlBadulem,
    "resin hurl badulem at {}",
    "tokin splatter septic at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    HurlLysirine,
    "resin hurl lysirine at {}",
    "tokin splatter paralytic at {}",
    "Hurl",
    "Splatter"
);
targetted_action!(
    Combust,
    "resin combust {}",
    "toxin kindle {}",
    "Combust",
    "Kindle"
);
untargetted_action!(Alacrity, "alacrity", "efficiency", "Alacrity", "Alacrity");

targetted_action!(
    RalothTrample,
    "order raloth trample {}",
    "order brutaliser rampage {}",
    "RalothTrample",
    "RalothTrample"
);

targetted_action!(
    PierceActionLeft,
    "dhuriv pierce {} left",
    "ringblade incise {} left",
    "Pierce",
    "Incise"
);
targetted_action!(
    PierceActionRight,
    "dhuriv pierce {} right",
    "ringblade incise {} right",
    "Pierce",
    "Incise"
);
targetted_action!(
    SeverActionLeft,
    "dhuriv sever {} left",
    "ringblade dissever {} left",
    "Sever",
    "Dissever"
);
targetted_action!(
    SeverActionRight,
    "dhuriv sever {} right",
    "ringblade dissever {} right",
    "Sever",
    "Dissever"
);

untargetted_action!(MightAction, "might", "grit", "Might", "Grit");

targetted_action!(
    DualrazeAction,
    "dhuriv dualraze {}",
    "ringblade twinraze {}",
    "Dualraze",
    "Twinraze"
);
targetted_action!(
    SpinecutAction,
    "dhuriv spinecut {}",
    "ringblade terminate {}",
    "Spinecut",
    "Terminate"
);
targetted_action!(
    ThroatcrushAction,
    "dhuriv throatcrush {}",
    "ringblade stifle {}",
    "Throatcrush",
    "Stifle"
);

pub struct WhirlAction {
    pub caster: String,
    pub target: String,
    pub venom: String,
    pub follow_up: bool,
}

impl WhirlAction {
    pub fn new(caster: String, target: String, venom: String, follow_up: bool) -> Self {
        WhirlAction {
            caster,
            target,
            venom,
            follow_up,
        }
    }
}

impl ActiveTransition for WhirlAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        let mirrored = timeline
            .state
            .borrow_agent(&self.caster)
            .class_state
            .is_mirrored();
        if self.follow_up {
            Ok(with_call_venom_single(
                timeline,
                &self.target,
                &self.venom,
                None,
                format!("envenom left with {}", self.venom),
            ))
        } else if !mirrored {
            Ok(with_call_venom_single(
                timeline,
                &self.target,
                &self.venom,
                None,
                format!("dhuriv whirl {} {}", self.target, self.venom),
            ))
        } else {
            Ok(with_call_venom_single(
                timeline,
                &self.target,
                &self.venom,
                None,
                format!("ringblade pirouette {} {}", self.target, self.venom),
            ))
        }
    }
    fn skill_names(&self) -> Vec<String> {
        vec!["Whirl".to_string(), "Pirouette".to_string()]
    }
}

pub struct FirstStrikeAction {
    pub caster: String,
    pub target: String,
    pub first_strike: FirstStrike,
}

impl FirstStrikeAction {
    pub fn new(caster: String, target: String, first_strike: FirstStrike) -> Self {
        FirstStrikeAction {
            caster,
            target,
            first_strike,
        }
    }
}

impl ActiveTransition for FirstStrikeAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        let mirrored = timeline
            .state
            .borrow_agent(&self.caster)
            .class_state
            .is_mirrored();
        if !self.first_strike.venom().is_empty() {
            Ok(with_call_venom_single(
                timeline,
                &self.target,
                self.first_strike.venom(),
                None,
                format!(
                    "order loyal attack {};;stand;;stand;;{}",
                    self.target,
                    self.first_strike.full_str(&self.target, mirrored),
                ),
            ))
        } else {
            Ok(format!(
                "order loyal attack {};;stand;;stand;;{}",
                self.target,
                self.first_strike.full_str(&self.target, mirrored)
            ))
        }
    }
    fn skill_names(&self) -> Vec<String> {
        let spirit = match &self.first_strike {
            FirstStrike::Slash(_) => "Slash",
            FirstStrike::Ambush(_) => "Ambush",
            FirstStrike::Blind => "Blind",
            FirstStrike::Twirl => "Twirl",
            FirstStrike::Strike => "Strike",
            FirstStrike::Crosscut => "Crosscut",
            FirstStrike::WeakenArms | FirstStrike::WeakenLegs => "Weaken",
            FirstStrike::Reave => "Reave",
            FirstStrike::Trip => "Trip",
            FirstStrike::Slam => "Slam",
            FirstStrike::DauntCoyote
            | FirstStrike::DauntRaloth
            | FirstStrike::DauntCrocodile
            | FirstStrike::DauntCockatrice => "Daunt",
            FirstStrike::Icebreath => "Icebreath",
            FirstStrike::Combust => "Combust",
        };
        let shadow = match &self.first_strike {
            FirstStrike::Slash(_) => "Contrive",
            FirstStrike::Ambush(_) => "Waylay",
            FirstStrike::Blind => "Ploy",
            FirstStrike::Twirl => "Ruse",
            FirstStrike::Strike => "Gambit",
            FirstStrike::Crosscut => "Phlebotomise",
            FirstStrike::WeakenArms | FirstStrike::WeakenLegs => "Impair",
            FirstStrike::Reave => "Shave",
            FirstStrike::Trip => "Gambol",
            FirstStrike::Slam => "Perplex",
            FirstStrike::DauntCoyote
            | FirstStrike::DauntRaloth
            | FirstStrike::DauntCrocodile
            | FirstStrike::DauntCockatrice => "Accost",
            FirstStrike::Icebreath => "Verglas",
            FirstStrike::Combust => "Kindle",
        };
        let mut names = vec![spirit.to_string()];
        if spirit != shadow {
            names.push(shadow.to_string());
        }
        names
    }
}

pub struct SecondStrikeAction {
    pub caster: String,
    pub target: String,
    pub second_strike: SecondStrike,
}

impl SecondStrikeAction {
    pub fn new(caster: String, target: String, second_strike: SecondStrike) -> Self {
        SecondStrikeAction {
            caster,
            target,
            second_strike,
        }
    }
}

impl ActiveTransition for SecondStrikeAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        let mirrored = timeline
            .state
            .borrow_agent(&self.caster)
            .class_state
            .is_mirrored();
        if !self.second_strike.venom().is_empty() {
            Ok(with_call_venom_single(
                timeline,
                &self.target,
                self.second_strike.venom(),
                None,
                self.second_strike.full_str(&self.target, mirrored),
            ))
        } else {
            Ok(self.second_strike.full_str(&self.target, mirrored))
        }
    }
    fn skill_names(&self) -> Vec<String> {
        let spirit = match &self.second_strike {
            SecondStrike::Stab(_) => "Stab",
            SecondStrike::Slice(_) => "Slice",
            SecondStrike::Thrust(_) => "Thrust",
            SecondStrike::Flourish(_) => "Flourish",
            SecondStrike::Disarm => "Disarm",
            SecondStrike::Gouge => "Gouge",
            SecondStrike::Heartbreaker => "Heartbreaker",
            SecondStrike::Slit => "Slit",
        };
        let shadow = match &self.second_strike {
            SecondStrike::Stab(_) => "Beguile",
            SecondStrike::Slice(_) => "Wile",
            SecondStrike::Thrust(_) => "Inveigle",
            SecondStrike::Flourish(_) => "Brandish",
            SecondStrike::Disarm => "Conciliate",
            SecondStrike::Gouge => "Muddle",
            SecondStrike::Heartbreaker => "Desolate",
            SecondStrike::Slit => "Razor",
        };
        let mut names = vec![spirit.to_string()];
        if spirit != shadow {
            names.push(shadow.to_string());
        }
        names
    }
}

pub struct ComboAction {
    pub caster: String,
    pub target: String,
    pub first_strike: FirstStrike,
    pub second_strike: SecondStrike,
}

impl ComboAction {
    pub fn new(
        caster: String,
        target: String,
        first_strike: FirstStrike,
        second_strike: SecondStrike,
    ) -> Self {
        ComboAction {
            caster,
            target,
            first_strike,
            second_strike,
        }
    }
}

impl ActiveTransition for ComboAction {
    fn simulate(&self, _timeline: &AetTimeline) -> Vec<ProbableEvent> {
        Vec::new()
    }
    fn act(&self, timeline: &AetTimeline) -> ActivateResult {
        Ok(get_combo_action(
            &timeline,
            &self.caster,
            &self.target,
            &self.first_strike,
            &self.second_strike,
        ))
    }
    fn skill_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        // Spirit (Sentinel) first strike names
        let spirit_first = match &self.first_strike {
            FirstStrike::Slash(_) => "Slash",
            FirstStrike::Ambush(_) => "Ambush",
            FirstStrike::Blind => "Blind",
            FirstStrike::Twirl => "Twirl",
            FirstStrike::Strike => "Strike",
            FirstStrike::Crosscut => "Crosscut",
            FirstStrike::WeakenArms | FirstStrike::WeakenLegs => "Weaken",
            FirstStrike::Reave => "Reave",
            FirstStrike::Trip => "Trip",
            FirstStrike::Slam => "Slam",
            FirstStrike::DauntCoyote
            | FirstStrike::DauntRaloth
            | FirstStrike::DauntCrocodile
            | FirstStrike::DauntCockatrice => "Daunt",
            FirstStrike::Icebreath => "Icebreath",
            FirstStrike::Combust => "Combust",
        };
        // Shadow (Executor) first strike names
        let shadow_first = match &self.first_strike {
            FirstStrike::Slash(_) => "Contrive",
            FirstStrike::Ambush(_) => "Waylay",
            FirstStrike::Blind => "Ploy",
            FirstStrike::Twirl => "Ruse",
            FirstStrike::Strike => "Gambit",
            FirstStrike::Crosscut => "Phlebotomise",
            FirstStrike::WeakenArms | FirstStrike::WeakenLegs => "Impair",
            FirstStrike::Reave => "Shave",
            FirstStrike::Trip => "Gambol",
            FirstStrike::Slam => "Perplex",
            FirstStrike::DauntCoyote
            | FirstStrike::DauntRaloth
            | FirstStrike::DauntCrocodile
            | FirstStrike::DauntCockatrice => "Accost",
            FirstStrike::Icebreath => "Verglas",
            FirstStrike::Combust => "Kindle",
        };
        names.push(spirit_first.to_string());
        if spirit_first != shadow_first {
            names.push(shadow_first.to_string());
        }
        // Spirit (Sentinel) second strike names
        let spirit_second = match &self.second_strike {
            SecondStrike::Stab(_) => "Stab",
            SecondStrike::Slice(_) => "Slice",
            SecondStrike::Thrust(_) => "Thrust",
            SecondStrike::Flourish(_) => "Flourish",
            SecondStrike::Disarm => "Disarm",
            SecondStrike::Gouge => "Gouge",
            SecondStrike::Heartbreaker => "Heartbreaker",
            SecondStrike::Slit => "Slit",
        };
        // Shadow (Executor) second strike names
        let shadow_second = match &self.second_strike {
            SecondStrike::Stab(_) => "Beguile",
            SecondStrike::Slice(_) => "Wile",
            SecondStrike::Thrust(_) => "Inveigle",
            SecondStrike::Flourish(_) => "Brandish",
            SecondStrike::Disarm => "Conciliate",
            SecondStrike::Gouge => "Muddle",
            SecondStrike::Heartbreaker => "Desolate",
            SecondStrike::Slit => "Razor",
        };
        names.push(spirit_second.to_string());
        if spirit_second != shadow_second {
            names.push(shadow_second.to_string());
        }
        names
    }
}

fn get_combo_action(
    timeline: &AetTimeline,
    caster: &String,
    target: &String,
    first_strike: &FirstStrike,
    second_strike: &SecondStrike,
) -> String {
    let mirrored = timeline
        .state
        .borrow_agent(caster)
        .class_state
        .is_mirrored();
    let attack = if first_strike.flourish() {
        format!(
            "{};;{}",
            first_strike.full_str(target, mirrored),
            second_strike.full_str(target, mirrored)
        )
    } else if mirrored {
        format!(
            "ringblade dance {} {} {} {} {}",
            target,
            first_strike.combo_str(mirrored),
            second_strike.combo_str(mirrored),
            first_strike.venom(),
            second_strike.venom(),
        )
    } else {
        format!(
            "dhuriv combo {} {} {} {} {}",
            target,
            first_strike.combo_str(mirrored),
            second_strike.combo_str(mirrored),
            first_strike.venom(),
            second_strike.venom(),
        )
    };
    let with_loyals = format!("order loyal attack {};;stand;;{}", target, attack);
    let v1 = first_strike.venom();
    let v2 = second_strike.venom();
    if !v1.is_empty() && !v2.is_empty() {
        with_call_venom_double(timeline, target, v1, v2, None, with_loyals)
    } else if !v1.is_empty() {
        with_call_venom_single(timeline, target, v1, None, with_loyals)
    } else if !v2.is_empty() {
        with_call_venom_single(timeline, target, v2, None, with_loyals)
    } else {
        with_loyals
    }
}
