use super::constants::*;
use crate::classes::*;
use crate::timeline::*;
use crate::types::*;
use crate::untargetted_action;

untargetted_action!(CallWisp, "call wisp", "fabricate lurker");
untargetted_action!(CallWeasel, "call weasel", "fabricate wardpeeler");
untargetted_action!(
    CallNightingale,
    "call nightingale",
    "fabricate lightdrinker"
);
untargetted_action!(CallRook, "call rook", "fabricate murder");
untargetted_action!(CallCoyote, "call coyote", "fabricate darkhound");
untargetted_action!(CallRaccoon, "call raccoon", "fabricate pilferer");
untargetted_action!(CallElk, "call elk", "fabricate monstrosity");
untargetted_action!(CallGyrfalcon, "call gyrfalcon", "fabricate throatripper");
untargetted_action!(CallRaloth, "call raloth", "fabricate brutaliser");
untargetted_action!(CallCrocodile, "call crocodile", "fabricate eviscerator");
untargetted_action!(CallIcewyrm, "call icewyrm", "fabricate rimestalker");
untargetted_action!(CallCockatrice, "call cockatrice", "fabricate terrifier");
targetted_action!(
    Icebreath,
    "order icewyrm icebreath {}",
    "order rimestalker verglas {}"
);

targetted_action!(
    HurlPyrolum,
    "resin hurl pyrolum at {}",
    "tokin splatter flammable at {}"
);
targetted_action!(
    HurlCorsin,
    "resin hurl corsin at {}",
    "tokin splatter coagulating at {}"
);
targetted_action!(
    HurlTrientia,
    "resin hurl trientia at {}",
    "tokin splatter hallucinatory at {}"
);
targetted_action!(
    HurlHarimel,
    "resin hurl harimel at {}",
    "tokin splatter adhesive at {}"
);
targetted_action!(
    HurlGlauxe,
    "resin hurl glauxe at {}",
    "tokin splatter choking at {}"
);
targetted_action!(
    HurlBadulem,
    "resin hurl badulem at {}",
    "tokin splatter septic at {}"
);
targetted_action!(
    HurlLysirine,
    "resin hurl lysirine at {}",
    "tokin splatter paralytic at {}"
);
targetted_action!(Combust, "resin combust {}", "toxin kindle {}");
untargetted_action!(Alacrity, "alacrity", "efficiency");

targetted_action!(
    RalothTrample,
    "order raloth trample {}",
    "order brutaliser rampage {}"
);

targetted_action!(
    PierceActionLeft,
    "dhuriv pierce {} left",
    "ringblade incise {} left"
);
targetted_action!(
    PierceActionRight,
    "dhuriv pierce {} right",
    "ringblade incise {} right"
);
targetted_action!(
    SeverActionLeft,
    "dhuriv sever {} left",
    "ringblade dissever {} left"
);
targetted_action!(
    SeverActionRight,
    "dhuriv sever {} right",
    "ringblade dissever {} right"
);

untargetted_action!(MightAction, "might", "grit");

targetted_action!(
    DualrazeAction,
    "dhuriv dualraze {}",
    "ringblade dualraze {}"
);
targetted_action!(
    SpinecutAction,
    "dhuriv spinecut {}",
    "ringblade terminate {}"
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
