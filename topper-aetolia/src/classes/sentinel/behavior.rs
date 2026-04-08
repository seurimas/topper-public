use behavior_bark::unpowered::*;
use serde::*;

use crate::{
    agent::sentinel::{Resin, SentinelBeast},
    bt::*,
    classes::{AFFLICT_VENOMS, VENOM_AFFLICTS, VenomPlan, get_venoms_from_plan, group::*},
    non_agent::AetTimelineRoomExt,
    observables::{ActiveTransition, PlainAction},
    types::*,
};

use super::{actions::*, constants::*};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SentinelBehavior {
    // Beast summoning
    CallBeast(SentinelBeast),
    CallBeasts(Vec<SentinelBeast>),
    // Resin
    Hurl(AetTarget, Resin),
    Combust(AetTarget),
    // Standalone first/second strikes
    SentinelFirstStrike(AetTarget, FirstStrikeSpec),
    SentinelSecondStrike(AetTarget, SecondStrikeSpec),
    // Weapon combos (uses venom plan from controller)
    SentinelCombo(AetTarget),
    SentinelComboFull(AetTarget, FirstStrikeSpec, SecondStrikeSpec),
    // Special weapon attacks
    SentinelDualraze(AetTarget),
    Spinecut(AetTarget),
    WhirlStart(AetTarget),
    WhirlContinue(AetTarget),
    SentinelPierce(AetTarget, String),
    SentinelSever(AetTarget, String),
    SentinelThroatcrush(AetTarget),
    // Raloth trample
    RalothTrample(AetTarget),
    // Self-buffs / class cures
    Alacrity,
    SentinelMight,
}

/// Serializable first-strike specification for ComboFull.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum FirstStrikeSpec {
    Slash,
    Ambush,
    Blind,
    Twirl,
    Strike,
    Crosscut,
    WeakenArms,
    WeakenLegs,
    Reave,
    Trip,
    Slam,
    DauntCoyote,
    DauntRaloth,
    DauntCrocodile,
    DauntCockatrice,
    Icebreath,
    Combust,
}

/// Serializable second-strike specification for ComboFull.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SecondStrikeSpec {
    Stab,
    Slice,
    Thrust,
    Flourish,
    Disarm,
    Gouge,
    Heartbreaker,
    Slit,
}

fn resolve_first_strike(
    spec: &FirstStrikeSpec,
    controller: &BehaviorController,
    target: &AgentState,
) -> FirstStrike {
    let get_venom = |target: &AgentState| -> &'static str {
        controller
            .get_venoms_from_plan(1, target, &vec![])
            .first()
            .copied()
            .unwrap_or("curare")
    };
    match spec {
        FirstStrikeSpec::Slash => FirstStrike::Slash(get_venom(target)),
        FirstStrikeSpec::Ambush => FirstStrike::Ambush(get_venom(target)),
        FirstStrikeSpec::Blind => FirstStrike::Blind,
        FirstStrikeSpec::Twirl => FirstStrike::Twirl,
        FirstStrikeSpec::Strike => FirstStrike::Strike,
        FirstStrikeSpec::Crosscut => FirstStrike::Crosscut,
        FirstStrikeSpec::WeakenArms => FirstStrike::WeakenArms,
        FirstStrikeSpec::WeakenLegs => FirstStrike::WeakenLegs,
        FirstStrikeSpec::Reave => FirstStrike::Reave,
        FirstStrikeSpec::Trip => FirstStrike::Trip,
        FirstStrikeSpec::Slam => FirstStrike::Slam,
        FirstStrikeSpec::DauntCoyote => FirstStrike::DauntCoyote,
        FirstStrikeSpec::DauntRaloth => FirstStrike::DauntRaloth,
        FirstStrikeSpec::DauntCrocodile => FirstStrike::DauntCrocodile,
        FirstStrikeSpec::DauntCockatrice => FirstStrike::DauntCockatrice,
        FirstStrikeSpec::Icebreath => FirstStrike::Icebreath,
        FirstStrikeSpec::Combust => FirstStrike::Combust,
    }
}

fn resolve_second_strike(
    spec: &SecondStrikeSpec,
    controller: &BehaviorController,
    target: &AgentState,
) -> SecondStrike {
    let get_venom = |target: &AgentState| -> &'static str {
        controller
            .get_venoms_from_plan(1, target, &vec![])
            .first()
            .copied()
            .unwrap_or("curare")
    };
    match spec {
        SecondStrikeSpec::Stab => SecondStrike::Stab(get_venom(target)),
        SecondStrikeSpec::Slice => SecondStrike::Slice(get_venom(target)),
        SecondStrikeSpec::Thrust => SecondStrike::Thrust(get_venom(target)),
        SecondStrikeSpec::Flourish => SecondStrike::Flourish(get_venom(target)),
        SecondStrikeSpec::Disarm => SecondStrike::Disarm,
        SecondStrikeSpec::Gouge => SecondStrike::Gouge,
        SecondStrikeSpec::Heartbreaker => SecondStrike::Heartbreaker,
        SecondStrikeSpec::Slit => SecondStrike::Slit,
    }
}

fn beast_call_action(beast: &SentinelBeast) -> Box<dyn ActiveTransition> {
    match beast {
        SentinelBeast::Wisp => Box::new(CallWisp::new(String::new())),
        SentinelBeast::Weasel => Box::new(CallWeasel::new(String::new())),
        SentinelBeast::Nightingale => Box::new(CallNightingale::new(String::new())),
        SentinelBeast::Rook => Box::new(CallRook::new(String::new())),
        SentinelBeast::Coyote => Box::new(CallCoyote::new(String::new())),
        SentinelBeast::Raccoon => Box::new(CallRaccoon::new(String::new())),
        SentinelBeast::Elk => Box::new(CallElk::new(String::new())),
        SentinelBeast::Gyrfalcon => Box::new(CallGyrfalcon::new(String::new())),
        SentinelBeast::Raloth => Box::new(CallRaloth::new(String::new())),
        SentinelBeast::Crocodile => Box::new(CallCrocodile::new(String::new())),
        SentinelBeast::Icewyrm => Box::new(CallIcewyrm::new(String::new())),
        SentinelBeast::Cockatrice => Box::new(CallCockatrice::new(String::new())),
    }
}

fn hurl_action(target_name: String, caster: String, resin: &Resin) -> Box<dyn ActiveTransition> {
    match resin {
        Resin::Pyrolum => Box::new(HurlPyrolum::new(caster, target_name)),
        Resin::Corsin => Box::new(HurlCorsin::new(caster, target_name)),
        Resin::Trientia => Box::new(HurlTrientia::new(caster, target_name)),
        Resin::Harimel => Box::new(HurlHarimel::new(caster, target_name)),
        Resin::Glauxe => Box::new(HurlGlauxe::new(caster, target_name)),
        Resin::Badulem => Box::new(HurlBadulem::new(caster, target_name)),
        Resin::Lysirine => Box::new(HurlLysirine::new(caster, target_name)),
    }
}

impl UnpoweredFunction for SentinelBehavior {
    type Model = BehaviorModel;
    type Controller = BehaviorController;

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
    ) -> UnpoweredFunctionState {
        let me = model.state.borrow_me();
        match self {
            // ── Beast summoning ──────────────────────────────────────────
            SentinelBehavior::CallBeast(beast) => {
                call_beasts_behavior(model, controller, &me, &vec![*beast])
            }
            SentinelBehavior::CallBeasts(wanted) => {
                call_beasts_behavior(model, controller, &me, wanted)
            }

            // ── Resin ────────────────────────────────────────────────────
            SentinelBehavior::Hurl(target, resin) => {
                if let Some(you) = target.get_target(model, controller) {
                    if you.resin_state.burning.is_active() {
                        return UnpoweredFunctionState::Failed;
                    } else if you.resin_state.hot.is_some() && you.resin_state.cold.is_some() {
                        // Don't throw resin at targets already affected by both resins
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(hurl_action(
                        target.get_name(model, controller),
                        model.who_am_i(),
                        resin,
                    ));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::Combust(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    let venom = controller
                        .get_venoms_from_plan(1, target_agent, &vec![])
                        .first()
                        .copied()
                        .unwrap_or("curare");
                    controller.plan.add_to_qeb(Box::new(ComboAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        FirstStrike::Combust,
                        SecondStrike::Flourish(venom),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Standalone first strike ──────────────────────────────────
            SentinelBehavior::SentinelFirstStrike(target, first_spec) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    let first_strike = resolve_first_strike(first_spec, controller, target_agent);
                    let afflictions = first_strike.afflictions();
                    if !afflictions.is_empty() && afflictions.iter().all(|a| target_agent.is(*a)) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(FirstStrikeAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        first_strike,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Standalone second strike ─────────────────────────────────
            SentinelBehavior::SentinelSecondStrike(target, second_spec) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    let me = model.state.borrow_me();
                    let second_strike =
                        resolve_second_strike(second_spec, controller, target_agent);
                    if second_strike.is_flourish()
                        && !me
                            .check_if_sentinel(&|s| s.has_first_strike(true))
                            .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    } else if let Some(affliction) = second_strike.affliction() {
                        if target_agent.is(affliction) {
                            return UnpoweredFunctionState::Failed;
                        }
                    }
                    controller
                        .plan
                        .add_to_plain(Box::new(SecondStrikeAction::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                            second_strike,
                        )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Weapon combo (auto-planned) ──────────────────────────────
            SentinelBehavior::SentinelCombo(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    let stack = controller.aff_priorities.clone();
                    // Pick first strike
                    let first_strike = stack
                        .as_ref()
                        .and_then(|s| get_first_strike_from_plan(s, 1, target_agent, &vec![]).pop())
                        .unwrap_or(FirstStrike::Slash("curare"));
                    // If target has rebounding and first strike doesn't ignore it, use Reave
                    let first_strike = if target_agent.is(FType::Rebounding)
                        && !first_strike.ignores_rebounding()
                    {
                        FirstStrike::Reave
                    } else {
                        println!(
                            "Chose first strike {:?} against target with rebounding {:?}",
                            first_strike,
                            target_agent.is(FType::Rebounding)
                        );
                        first_strike
                    };
                    // Assume first strike hits for second-strike planning
                    let mut assumed = target_agent.clone();
                    if let Some(affs) = FIRST_STRIKE_AFFS.get(&first_strike) {
                        for aff in affs {
                            assumed.set_flag(*aff, true);
                        }
                    }
                    // Pick second strike
                    let second_strike = if first_strike.flourish() {
                        // Flourish uses venom-based approach
                        stack
                            .as_ref()
                            .and_then(|s| {
                                get_venoms_from_plan(s, 1, &assumed, &vec![])
                                    .first()
                                    .and_then(|v| Some(SecondStrike::Flourish(v)))
                            })
                            .unwrap_or(SecondStrike::Stab("curare"))
                    } else {
                        stack
                            .as_ref()
                            .and_then(|s| {
                                get_second_strike_from_plan(s, 1, &assumed, &vec![]).pop()
                            })
                            .unwrap_or(SecondStrike::Stab("curare"))
                    };
                    controller.plan.add_to_qeb(Box::new(ComboAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        first_strike,
                        second_strike,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Weapon combo (explicit strikes) ──────────────────────────
            SentinelBehavior::SentinelComboFull(target, first_spec, second_spec) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    let first_strike = resolve_first_strike(first_spec, controller, target_agent);
                    let second_strike =
                        resolve_second_strike(second_spec, controller, target_agent);

                    let first_afflictions = first_strike.afflictions();
                    if !first_afflictions.is_empty()
                        && first_afflictions.iter().all(|a| target_agent.is(*a))
                    {
                        return UnpoweredFunctionState::Failed;
                    }

                    let second_affliction = second_strike.affliction();
                    if let Some(affliction) = second_affliction {
                        // If second strike has an affliction, check it against the target before the first strike hits
                        if target_agent.is(affliction) {
                            return UnpoweredFunctionState::Failed;
                        }
                    }

                    controller.plan.add_to_qeb(Box::new(ComboAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        first_strike,
                        second_strike,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Special weapon attacks ───────────────────────────────────
            SentinelBehavior::SentinelDualraze(target) => {
                if let Some(_target_agent) = target.get_target(model, controller) {
                    controller.plan.add_to_qeb(Box::new(DualrazeAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::Spinecut(target) => {
                if let Some(_target_agent) = target.get_target(model, controller) {
                    controller.plan.add_to_qeb(Box::new(SpinecutAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::WhirlStart(target) => {
                if let Some(you) = target.get_target(model, controller) {
                    let venom = controller
                        .get_venoms_from_plan(1, you, &vec![])
                        .first()
                        .copied()
                        .unwrap_or("curare");
                    controller.plan.add_to_qeb(Box::new(WhirlAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        venom.to_string(),
                        false,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::WhirlContinue(target) => {
                if let Some(you) = target.get_target(model, controller) {
                    let me = model.state.borrow_me();
                    if !me.check_if_sentinel(&|s| s.whirl_coming()).unwrap_or(false) {
                        return UnpoweredFunctionState::Failed;
                    }
                    let venom = controller
                        .get_venoms_from_plan(1, you, &vec![])
                        .first()
                        .copied()
                        .unwrap_or("curare");
                    controller.plan.add_to_plain(Box::new(WhirlAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                        venom.to_string(),
                        true,
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::SentinelPierce(target, side) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if target_agent.is(FType::Rebounding)
                        || target_agent.is(FType::Shielded)
                        || target_agent.can_parry()
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    let action: Box<dyn ActiveTransition> = match side.as_str() {
                        "left" => Box::new(PierceActionLeft::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                        )),
                        "right" => Box::new(PierceActionRight::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                        )),
                        _ => return UnpoweredFunctionState::Failed,
                    };
                    controller.plan.add_to_qeb(action);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::SentinelSever(target, side) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if target_agent.is(FType::Rebounding)
                        || target_agent.is(FType::Shielded)
                        || target_agent.can_parry()
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    let action: Box<dyn ActiveTransition> = match side.as_str() {
                        "left" => Box::new(SeverActionLeft::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                        )),
                        "right" => Box::new(SeverActionRight::new(
                            model.who_am_i(),
                            target.get_name(model, controller),
                        )),
                        _ => return UnpoweredFunctionState::Failed,
                    };
                    controller.plan.add_to_qeb(action);
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }
            SentinelBehavior::SentinelThroatcrush(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if target_agent.is(FType::Rebounding)
                        || target_agent.is(FType::Shielded)
                        || target_agent.is(FType::DestroyedThroat)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(ThroatcrushAction::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Raloth trample ───────────────────────────────────────────
            SentinelBehavior::RalothTrample(target) => {
                if let Some(target_agent) = target.get_target(model, controller) {
                    if !me
                        .check_if_sentinel(&|s| s.has_beast(SentinelBeast::Raloth))
                        .unwrap_or(false)
                    {
                        return UnpoweredFunctionState::Failed;
                    }
                    if !target_agent.is(FType::Fallen) {
                        return UnpoweredFunctionState::Failed;
                    }
                    controller.plan.add_to_qeb(Box::new(RalothTrample::new(
                        model.who_am_i(),
                        target.get_name(model, controller),
                    )));
                    UnpoweredFunctionState::Complete
                } else {
                    UnpoweredFunctionState::Failed
                }
            }

            // ── Self-buffs / class cures ─────────────────────────────────
            SentinelBehavior::Alacrity => {
                if me.check_if_sentinel(&|s| s.alacrity > 0).unwrap_or(false) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(Alacrity::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
            SentinelBehavior::SentinelMight => {
                if !me.balanced(BType::ClassCure1) {
                    return UnpoweredFunctionState::Failed;
                }
                controller
                    .plan
                    .add_to_qeb(Box::new(MightAction::new(model.who_am_i())));
                UnpoweredFunctionState::Complete
            }
        }
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // Nothing to reset
    }
}

const MAX_COMPANIONS: usize = 4;

fn beast_dismiss_name(beast: &SentinelBeast, mirrored: bool) -> &'static str {
    match (beast, mirrored) {
        (SentinelBeast::Wisp, false) => "wisp",
        (SentinelBeast::Wisp, true) => "lurker",
        (SentinelBeast::Weasel, false) => "weasel",
        (SentinelBeast::Weasel, true) => "wardpeeler",
        (SentinelBeast::Nightingale, false) => "nightingale",
        (SentinelBeast::Nightingale, true) => "lightdrinker",
        (SentinelBeast::Rook, false) => "rook",
        (SentinelBeast::Rook, true) => "murder",
        (SentinelBeast::Coyote, false) => "coyote",
        (SentinelBeast::Coyote, true) => "darkhound",
        (SentinelBeast::Raccoon, false) => "raccoon",
        (SentinelBeast::Raccoon, true) => "pilferer",
        (SentinelBeast::Elk, false) => "elk",
        (SentinelBeast::Elk, true) => "monstrosity",
        (SentinelBeast::Gyrfalcon, false) => "gyrfalcon",
        (SentinelBeast::Gyrfalcon, true) => "throatripper",
        (SentinelBeast::Raloth, false) => "raloth",
        (SentinelBeast::Raloth, true) => "brutaliser",
        (SentinelBeast::Crocodile, false) => "crocodile",
        (SentinelBeast::Crocodile, true) => "eviscerator",
        (SentinelBeast::Icewyrm, false) => "icewyrm",
        (SentinelBeast::Icewyrm, true) => "rimestalker",
        (SentinelBeast::Cockatrice, false) => "cockatrice",
        (SentinelBeast::Cockatrice, true) => "terrifier",
    }
}

fn call_beasts_behavior(
    model: &BehaviorModel,
    controller: &mut BehaviorController,
    me: &AgentState,
    wanted: &[SentinelBeast],
) -> UnpoweredFunctionState {
    let sentinel_state = me.check_if_sentinel(&|s| s.clone());
    let sentinel_state = match sentinel_state {
        Some(s) => s,
        None => return UnpoweredFunctionState::Failed,
    };

    // Check if all wanted beasts are already summoned
    if wanted.iter().all(|b| sentinel_state.has_beast(*b)) {
        return UnpoweredFunctionState::Failed;
    }

    let caster = model.who_am_i();
    let mirrored = model.state.borrow_agent(&caster).class_state.is_mirrored();

    // Find beasts that are summoned but not wanted
    let unwanted: Vec<SentinelBeast> = sentinel_state
        .beasts
        .iter()
        .filter(|b| !wanted.contains(b))
        .copied()
        .collect();

    // Find beasts that are wanted but not yet summoned
    let needed: Vec<&SentinelBeast> = wanted
        .iter()
        .filter(|b| !sentinel_state.has_beast(**b))
        .collect();

    let available_slots = MAX_COMPANIONS - sentinel_state.beast_count();

    if available_slots == 0 && !needed.is_empty() {
        // No room — dismiss an unwanted beast
        if let Some(to_dismiss) = unwanted.first() {
            println!(
                "Dismissing {:?} to summon {:?}",
                to_dismiss,
                needed.first().unwrap()
            );
            let name = beast_dismiss_name(to_dismiss, mirrored);
            controller
                .plan
                .add_to_qeb(Box::new(PlainAction::new(format!("dismiss {}", name))));
            return UnpoweredFunctionState::Complete;
        }
        // All 4 slots are wanted beasts, nothing to do
        return UnpoweredFunctionState::Failed;
    }

    // Summon the next needed beast
    if let Some(beast) = needed.first() {
        controller.plan.add_to_qeb(beast_call_action(beast));
        return UnpoweredFunctionState::Complete;
    }

    UnpoweredFunctionState::Failed
}
