use super::*;
use crate::alpha_beta::ActionPlanner;
use crate::bt::DEBUG_TREES;
use crate::classes::group::call_venom;
use crate::classes::group::call_venoms;
use crate::classes::group::should_call_venoms;
use crate::classes::*;
use crate::curatives::get_cure_depth;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;
use crate::untargetted_action;
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MonkComboAction {
    pub combo: MonkCombo,
    pub target: String,
}

impl ActiveTransition for MonkComboAction {
    fn act(&self, timline: &AetTimeline) -> ActivateResult {
        match &self.combo {
            MonkCombo::Standard(_, attacks) => Ok(format!(
                "combo {} {} {} {}",
                self.target,
                attacks[0].param_str(),
                attacks[1].param_str(),
                attacks[2].param_str()
            )),
            MonkCombo::Cobra(attacks) => Ok(format!(
                "combo {} {} {}",
                self.target,
                attacks[0].param_str(),
                attacks[1].param_str()
            )),
            MonkCombo::ChangeStance(stance, attacks) => Ok(format!(
                "combo {} {} {} {}",
                self.target,
                stance.param_str(),
                attacks[0].param_str(),
                attacks[1].param_str()
            )),
        }
    }
}

untargetted_action!(Weathering, "weathering");
untargetted_action!(Vitality, "vitality");
untargetted_action!(Sturdiness, "stand firm");
untargetted_action!(Toughness, "toughness");
untargetted_action!(Deaf, "deaf");
untargetted_action!(Hearing, "hearing");
untargetted_action!(Sight, "sight");
untargetted_action!(Blind, "blind");
untargetted_action!(Resistance, "resistance");
untargetted_action!(ProjectilesOn, "projectiles on");
untargetted_action!(ProjectilesOff, "projectiles off");
untargetted_action!(Constitution, "constitution");
untargetted_action!(SplitMind, "split mind");
untargetted_action!(JoinMind, "join mind");
untargetted_action!(ConsciousnessOn, "consciousness on");
untargetted_action!(ConsciousnessOff, "consciousness off");
untargetted_action!(Immunity, "immunity");
untargetted_action!(Boosting, "boost regeneration");
targetted_action!(TransmuteHealth, "transmute health {}");
targetted_action!(TransmuteMana, "transmute mana {}");
untargetted_action!(Numbness, "numb");
untargetted_action!(KaiTrance, "kai trance");
untargetted_action!(BreakTrance, "break trance");
targetted_action!(KaiChoke, "kai choke {}");
targetted_action!(NurtureAny, "kai nurture {}");
targetted_action!(NurtureLeftArm, "kai nurture {} left arm");
targetted_action!(NurtureRightArm, "kai nurture {} right arm");
targetted_action!(NurtureLeftLeg, "kai nurture {} left leg");
targetted_action!(NurtureRightLeg, "kai nurture {} right leg");
targetted_action!(NurtureHead, "kai nurture {} head");
targetted_action!(NurtureTorso, "kai nurture {} torso");
targetted_action!(KaiCripple, "kai cripple {}");
targetted_action!(KaiStrike, "kai strike {}");
untargetted_action!(KaiHeal, "kai heal");
targetted_action!(KaiBanish, "kai banish {}");
targetted_action!(KaiEnfeeble, "kai enfeeble {}");
untargetted_action!(Deliverance, "kai deliverance");

untargetted_action!(Bodyblock, "bdb");
untargetted_action!(BodyblockOff, "unblock body");
untargetted_action!(Evadeblock, "evb");
untargetted_action!(EvadeblockOff, "unblock evade");
// targetted_action!(Slam, "slt {}");
// targetted_action!(WrenchAny, "wrt {}");
// targetted_action!(WrenchLeftArm, "wrt {} left arm");
// targetted_action!(WrenchRightArm, "wrt {} right arm");
// targetted_action!(WrenchLeftLeg, "wrt {} left leg");
// targetted_action!(WrenchRightLeg, "wrt {} right leg");
untargetted_action!(Kipup, "kipup");
untargetted_action!(Armblock, "asb");
untargetted_action!(ArmblockOff, "unblock arms");
untargetted_action!(Legblock, "lsb");
untargetted_action!(LegblockOff, "unblock legs");
untargetted_action!(Tripblock, "trb");
untargetted_action!(TripblockOff, "unblock trip");
untargetted_action!(Pinchblock, "pnb");
untargetted_action!(PinchblockOff, "unblock pinch");
targetted_action!(Backbreaker, "bbt {}");

targetted_action!(MindLock, "mind lock {}");
targetted_action!(MindFear, "mind fear {}");
targetted_action!(MindHallucinate, "mind hallucinate {}");
targetted_action!(MindParalyse, "mind paralyse {}");
targetted_action!(MindConfuse, "mind confuse {}");
targetted_action!(MindSuffuse, "mind suffuse {}");
targetted_action!(MindDrain, "mind drain {}");
targetted_action!(MindDivine, "mind divine {}");
targetted_action!(MindRecklessness, "mind recklessness {}");
targetted_action!(MindDisrupt, "mind disrupt {}");
targetted_action!(MindEpilepsy, "mind epilepsy {}");
targetted_action!(MindPacify, "mind pacify {}");
targetted_action!(MindStupidity, "mind stupidity {}");
targetted_action!(MindAnorexia, "mind anorexia {}");
targetted_action!(MindAmnesia, "mind amnesia {}");
targetted_action!(MindDeadening, "mind deadening {}");
targetted_action!(MindStrip, "mind strip {}");
targetted_action!(MindCrush, "mind crush {}");
targetted_action!(MindBatter, "mind batter {}");
targetted_action!(MindCleanse, "mind cleanse {}");
targetted_action!(MindPush, "mind push {}");
