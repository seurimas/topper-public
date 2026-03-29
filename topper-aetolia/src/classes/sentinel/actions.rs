use super::constants::*;
use crate::classes::*;
use crate::timeline::*;
use crate::types::*;
use crate::untargetted_action;

untargetted_action!(CallWisp, "call wisp", "fabricate lurker", "CallWisp", "CallWisp");
untargetted_action!(CallWeasel, "call weasel", "fabricate wardpeeler", "CallWeasel", "CallWeasel");
untargetted_action!(
    CallNightingale,
    "call nightingale",
    "fabricate lightdrinker",
    "CallNightingale",
    "CallNightingale"
);
untargetted_action!(CallRook, "call rook", "fabricate murder", "CallRook", "CallRook");
untargetted_action!(CallCoyote, "call coyote", "fabricate darkhound", "CallCoyote", "CallCoyote");
untargetted_action!(CallRaccoon, "call raccoon", "fabricate pilferer", "CallRaccoon", "CallRaccoon");
untargetted_action!(CallElk, "call elk", "fabricate monstrosity", "CallElk", "CallElk");
untargetted_action!(CallGyrfalcon, "call gyrfalcon", "fabricate throatripper", "CallGyrfalcon", "CallGyrfalcon");
untargetted_action!(CallRaloth, "call raloth", "fabricate brutaliser", "CallRaloth", "CallRaloth");
untargetted_action!(CallCrocodile, "call crocodile", "fabricate eviscerator", "CallCrocodile", "CallCrocodile");
untargetted_action!(CallIcewyrm, "call icewyrm", "fabricate rimestalker", "CallIcewyrm", "CallIcewyrm");
untargetted_action!(CallCockatrice, "call cockatrice", "fabricate terrifier", "CallCockatrice", "CallCockatrice");
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
    "HurlPyrolum"
);
targetted_action!(
    HurlCorsin,
    "resin hurl corsin at {}",
    "tokin splatter coagulating at {}",
    "Hurl",
    "HurlCorsin"
);
targetted_action!(
    HurlTrientia,
    "resin hurl trientia at {}",
    "tokin splatter hallucinatory at {}",
    "Hurl",
    "HurlTrientia"
);
targetted_action!(
    HurlHarimel,
    "resin hurl harimel at {}",
    "tokin splatter adhesive at {}",
    "Hurl",
    "HurlHarimel"
);
targetted_action!(
    HurlGlauxe,
    "resin hurl glauxe at {}",
    "tokin splatter choking at {}",
    "Hurl",
    "HurlGlauxe"
);
targetted_action!(
    HurlBadulem,
    "resin hurl badulem at {}",
    "tokin splatter septic at {}",
    "Hurl",
    "HurlBadulem"
);
targetted_action!(
    HurlLysirine,
    "resin hurl lysirine at {}",
    "tokin splatter paralytic at {}",
    "Hurl",
    "HurlLysirine"
);
targetted_action!(Combust, "resin combust {}", "toxin kindle {}", "Combust", "Combust");
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
    "PierceActionLeft"
);
targetted_action!(
    PierceActionRight,
    "dhuriv pierce {} right",
    "ringblade incise {} right",
    "Pierce",
    "PierceActionRight"
);
targetted_action!(
    SeverActionLeft,
    "dhuriv sever {} left",
    "ringblade dissever {} left",
    "Sever",
    "SeverActionLeft"
);
targetted_action!(
    SeverActionRight,
    "dhuriv sever {} right",
    "ringblade dissever {} right",
    "Sever",
    "SeverActionRight"
);

untargetted_action!(MightAction, "might", "grit", "Might", "MightAction");

targetted_action!(
    DualrazeAction,
    "dhuriv dualraze {}",
    "ringblade dualraze {}",
    "Dualraze",
    "DualrazeAction"
);
targetted_action!(
    SpinecutAction,
    "dhuriv spinecut {}",
    "ringblade terminate {}",
    "Spinecut",
    "SpinecutAction"
);

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
    format!("order loyal attack {};;stand;;stand;;{}", target, attack)
}
