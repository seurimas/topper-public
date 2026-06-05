use crate::classes::Class;
use crate::curatives::MENTAL_AFFLICTIONS;
use crate::curatives::RANDOM_CURES;
use crate::non_agent::AetTimelineDenizenExt;
use crate::non_agent::AetTimelineRoomExt;
use crate::timeline::*;
use crate::types::*;
use regex::Regex;

use super::phenomenon_in_room;

const SPARK_DAMAGE_PERCENT: CType = 5;
const FIREBURST_DAMAGE_PERCENT: CType = 6;
const CONFLAGRATE_DAMAGE_PERCENT: CType = 7;
const SUNPOT_DAMAGE_PERCENT: CType = 18;
const PYROCLAST_DAMAGE_PERCENT: CType = 20;
const DISINTEGRATE_DAMAGE_PERCENT: CType = 40;

const ICERAY_DAMAGE_PERCENT: CType = 10;
const ICICLE_DAMAGE_PERCENT: CType = 15;
const ICE_SHARD_DAMAGE_PERCENT: CType = 1;
const DIREFROST_DAMAGE_PERCENT: CType = 7;
const DIREFROST_PROC_DAMAGE_PERCENT: CType = 10;
// Mana drains
const DRENCH_BASE_PERCENT: CType = 7;
const DRENCH_SHIVERING_PERCENT: CType = 14;
const DRENCH_FRIGID_PERCENT: CType = 28;
const DRENCH_FROZEN_PERCENT: CType = 40;
const CRYSTALISE_DAMAGE_PERCENTS: [CType; 5] = [19, 43, 67, 91, 115];

const WINDLANCE_DAMAGE_PERCENT: CType = 4;
const ARCBOLT_DAMAGE_PERCENT: CType = 14;
const THUNDERCLAP_DAMAGE_PERCENT: CType = 7;
const AEROBLAST_DAMAGE_PERCENT: CType = 10;
const AEROBLAST_STUN_DAMAGE_PERCENT: CType = 18;

lazy_static! {
    static ref AEROBLAST_FAST: Regex = Regex::new(r"(?i)^cast aeroblast \w+ (fast|slow)$").unwrap();
    static ref ICICLE_SENT: Regex = Regex::new(r"(?i)^cast icicle (\w+)$").unwrap();
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    if let Some(caps) = AEROBLAST_FAST.captures(command) {
        let me = agent_states.me.clone();
        let speed = caps.get(1).unwrap().as_str().to_ascii_lowercase();
        agent_states.add_player_hint(&me, &"AEROBLAST_SPEED".to_string(), speed);
    } else if let Some(caps) = ICICLE_SENT.captures(command) {
        let me = agent_states.me.clone();
        let limb = caps.get(1).unwrap().as_str().to_ascii_lowercase();
        agent_states.add_player_hint(&me, &"ICICLE_TARGET".to_string(), limb);
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        // Afflicts with blisters + limp_veins
        "Spark" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Blisters, FType::Impairment],
                after,
            );
            for_agent(agent_states, &combat_action.target, &|me| {
                me.damage_stat_percent(SType::Health, SPARK_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                });
            });
        }
        // Afflicts with ashenfeet
        "Ashenfeet" => {
            if combat_action.annotation.eq("proc") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.toggle_flag(FType::AshenFeet, false);
                    me.set_flag(FType::LeftLegCrippled, true);
                    me.set_flag(FType::RightLegCrippled, true);
                    me.set_flag(FType::Fallen, true);
                });
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::AshenFeet],
                    after,
                );
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Fire);
                    });
                });
            }
        }
        // Gives me 4 stacks of fireburst, or hits for ablaze
        "Fireburst" => {
            if combat_action.target.is_empty() {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fireburst_fill();
                        ascendril.cast_spell(Element::Fire);
                    })
                });
            } else {
                let observations = after.clone();
                let arm_balance = match combat_action.annotation.as_str() {
                    "left" => Some(BType::LeftHandBalance),
                    "right" => Some(BType::RightHandBalance),
                    _ => None,
                };
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fireburst_decrement();
                    });
                    if let Some(arm_balance) = arm_balance {
                        apply_or_infer_balance(me, (arm_balance, 2.5), &observations);
                    }
                });
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.tick_flag_up(FType::Ablaze);
                    me.damage_stat_percent(SType::Health, FIREBURST_DAMAGE_PERCENT);
                });
            }
        }
        // Gives the target a Blazwhirl phenomenon
        "Blazewhirl" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                    ascendril.try_claim(PhenomenaKind::Blazewhirl);
                });
            });
        }
        // With ablaze, gives emberbrand.
        "Conflagrate" => {
            let ablaze = after.iter().any(|obs| match obs {
                AetObservation::CombatAction(action) => action.annotation.eq("ablaze"),
                _ => false,
            });
            if combat_action.annotation.eq("ablaze") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Ablaze, true);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Courage, false);
                    me.damage_stat_percent(SType::Health, CONFLAGRATE_DAMAGE_PERCENT);
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Fire);
                    });
                });
            }
        }
        "Emberbranded" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                if me.is(FType::Clumsiness) {
                    me.set_flag(FType::Weariness, true);
                } else {
                    me.set_flag(FType::Clumsiness, true);
                }
            });
        }
        // Gives afterburn after a short wait
        "Afterburn" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.raise_afterburn();
                    ascendril.cast_spell(Element::Fire);
                });
            });
        }
        // Gives sunspot after a short wait.
        "Sunspot" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.ascendril_board.sunspot();
                me.damage_stat_percent(SType::Health, SUNPOT_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                });
            });
        }
        // Conditional attack, dealing head trauma and more
        "Pyroclast" => {
            let blazewhirl = phenomenon_in_room(agent_states, PhenomenaKind::Blazewhirl);
            if attack_hit(after) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (LType::HeadDamage, 22., true),
                    after,
                );
                if blazewhirl {
                    attack_afflictions(
                        agent_states,
                        &combat_action.target,
                        vec![FType::Stun],
                        after,
                    );
                }
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.damage_stat_percent(SType::Health, PYROCLAST_DAMAGE_PERCENT);
                    if !me.is(FType::Arcane) {
                        me.damage_stat_percent(SType::Health, PYROCLAST_DAMAGE_PERCENT);
                    }
                });
            }

            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                    ascendril.use_up_resonance();
                });
            });
        }
        // Strip audit defenses
        "Disintegrate" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Arcane, false);
                me.damage_stat_percent(SType::Health, DISINTEGRATE_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                    ascendril.use_up_resonance();
                });
            });
            if combat_action.annotation.eq("charge") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_channel(ChannelType::Disintegrate, 4.);
                });
            }
        }
        // Freezes twice
        "Coldsnap" => {
            if combat_action.annotation.eq("proc") {
                attack_first_affliction(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Shivering, FType::Frigid, FType::Frozen],
                    after,
                );
                attack_first_affliction(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Shivering, FType::Frigid, FType::Frozen],
                    after,
                );
            } else {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Water);
                    });
                });
            }
        }
        "Frostbrand" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.observe_flag(FType::Frostbrand, true);
                me.observe_flag(FType::Direfrost, false);
            });
            if combat_action.annotation.eq("hypothermia") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Hypothermia, true);
                    me.observe_flag(FType::Frozen, true);
                    me.observe_flag(FType::Shivering, true);
                    me.observe_flag(FType::Frigid, true);
                });
            } else {
                attack_first_affliction(
                    agent_states,
                    &combat_action.caster,
                    vec![FType::Shivering, FType::Frigid, FType::Frozen],
                    after,
                );
            }
        }
        /**
        If shivering, knock of balance.
        */
        "Iceray" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Shivering) {
                    me.set_balance(BType::Balance, 0.5);
                }
                me.damage_stat_percent(SType::Health, ICERAY_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        // Creates glazeflow.
        "Glazeflow" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                    ascendril.try_claim(PhenomenaKind::Glazeflow);
                });
            });
        }
        // Drains mana based on freeze tier.
        "Drench" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                let drain = if me.is(FType::Frozen) {
                    DRENCH_FROZEN_PERCENT
                } else if me.is(FType::Frigid) {
                    DRENCH_FRIGID_PERCENT
                } else if me.is(FType::Shivering) {
                    DRENCH_SHIVERING_PERCENT
                } else {
                    DRENCH_BASE_PERCENT
                };
                me.damage_stat_percent(SType::Mana, drain);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        // With shivering, gives direfrost.
        "Direfrost" => {
            if combat_action.annotation.eq("direfrosted") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Direfrost, true);
                    me.set_flag(FType::Mindfog, true);
                    me.damage_stat_percent(SType::Health, DIREFROST_PROC_DAMAGE_PERCENT);
                });
            } else if combat_action.annotation.eq("frostbranded") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Frostbrand, true);
                    me.set_flag(FType::Direfrost, false);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.damage_stat_percent(SType::Health, DIREFROST_DAMAGE_PERCENT);
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Water);
                    });
                });
            }
        }
        // Gives 3 icicles.
        "Icicle" => {
            if combat_action.annotation.eq("hit") {
                if let Some(AetObservation::Parry(who, _what)) = after.get(1) {
                } else {
                    attack_limb_damage(
                        agent_states,
                        &combat_action.caster,
                        (LType::TorsoDamage, 10.0, true),
                        after,
                    );
                    for_agent(agent_states, &combat_action.caster, &|me| {
                        me.ascendril_board.icicles_hit();
                        me.damage_stat_percent(SType::Health, ICICLE_DAMAGE_PERCENT);
                    });
                }
            } else if combat_action.annotation.eq("already_icicles") {
                let target = agent_states
                    .get_player_hint(&combat_action.caster, &"ICICLE_TARGET".to_string());
                if let Some(target) = target {
                    for_agent(agent_states, &target, &|me| {
                        me.ascendril_board.an_icicle();
                    });
                }
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.ascendril_board.icicles_spawn();
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Water);
                    });
                });
            }
        }
        // Shatters icicles.
        "Shatter" => {
            if let Some(limb) = LType::try_from_name(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.ascendril_board.shatter_down();
                });
                if let Some(AetObservation::Parry(who, _what)) = after.get(1) {
                } else {
                    attack_limb_damage(
                        agent_states,
                        &combat_action.caster,
                        (limb, 4.0, true),
                        after,
                    );
                    for_agent(agent_states, &combat_action.caster, &|me| {
                        me.damage_stat_percent(SType::Health, ICE_SHARD_DAMAGE_PERCENT);
                    });
                }
            } else if combat_action.annotation.eq("no_icicles") {
                let target = agent_states
                    .get_player_hint(&combat_action.caster, &"ICICLE_TARGET".to_string());
                if let Some(target) = target {
                    for_agent(agent_states, &target, &|me| {
                        me.ascendril_board.clear_icicles();
                    });
                }
            } else {
                agent_states.for_all_agents(&|me| {
                    me.ascendril_board.shatter();
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Water);
                    });
                });
            }
        }
        // If no leivitation, give ice_encased. If shivering, give hobbled. If glazeflow in room, give frozen_feet.
        "Crystalise" => {
            let glazeflow_in_room = phenomenon_in_room(agent_states, PhenomenaKind::Glazeflow);
            for_agent(agent_states, &combat_action.target, &move |me| {
                let mut conditions: usize = 0;
                if !me.is(FType::Speed) {
                    me.set_flag(FType::IceEncased, true);
                    conditions += 1;
                }
                if me.is(FType::Fallen) {
                    conditions += 1;
                }
                if me.is(FType::Frigid) {
                    me.set_flag(FType::Hobbled, true);
                    conditions += 1;
                } else if me.is(FType::Shivering) {
                    me.set_flag(FType::Hobbled, true);
                }
                if glazeflow_in_room {
                    me.set_flag(FType::FrozenFeet, true);
                    conditions += 1;
                }
                me.damage_stat_percent(SType::Mana, CRYSTALISE_DAMAGE_PERCENTS[conditions]);
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                    ascendril.use_up_resonance();
                });
            });
        }
        // Summons a winterheart, uses up resonance.
        "Winterheart" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                // TODO
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                    ascendril.use_up_resonance();
                });
            });
            if combat_action.annotation.eq("charge") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_channel(ChannelType::Winterheart, 4.);
                });
            }
        }
        // Gives fallen or strips shield.
        "Windlance" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Shielded) {
                    me.set_flag(FType::Shielded, false);
                } else {
                    me.set_flag(FType::Fallen, true);
                }
                me.damage_stat_percent(SType::Health, WINDLANCE_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Gives vertigo and confusion.
        "Pressurize" => {
            let laxity = after.iter().any(|obs| match obs {
                AetObservation::CombatAction(action) => action.annotation.eq("laxity"),
                _ => false,
            });
            if combat_action.annotation.eq("laxity") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Laxity, true);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Vertigo, true);
                    me.set_flag(FType::Confusion, true);
                    if laxity {
                        me.set_flag(FType::Laxity, true);
                    }
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Air);
                        if laxity {
                            ascendril.cast_spell(Element::Air);
                        }
                    });
                });
            }
        }
        // Gives paresis or turns into paralysis.
        "Arcbolt" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Paresis) {
                    me.set_flag(FType::Paralysis, true);
                } else {
                    me.set_flag(FType::Paresis, true);
                }
                me.damage_stat_percent(SType::Health, ARCBOLT_DAMAGE_PERCENT);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Spawns electrosphere.
        "Electrosphere" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                    ascendril.try_claim(PhenomenaKind::Electrosphere);
                });
            });
        }
        // Gives dizziness and stupidity. If they have both and another mental, give thunderbrand.
        "Thunderclap" => {
            if combat_action.annotation.eq("branded") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Thunderbrand, true);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Dizziness, true);
                    me.set_flag(FType::Stupidity, true);
                    if !me.is(FType::Courage) {
                        me.set_flag(FType::Confusion, true);
                    }
                    me.damage_stat_percent(SType::Health, THUNDERCLAP_DAMAGE_PERCENT);
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Air);
                    });
                });
            }
        }
        // Knocks unconcious.
        "Feedback" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Unconsciousness, true);
                me.set_stat_percent(SType::Mana, 100);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                    ascendril.feedback_used();
                });
            });
        }
        "Aeroblast" => {
            if combat_action.annotation.eq("hit") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Fallen, true);
                    if me.is(FType::Stupidity) && me.is(FType::Confusion) {
                        me.set_flag(FType::Dazed, true);
                    }
                    if me.is(FType::TorsoBroken) {
                        me.set_flag(FType::Speed, false);
                    }
                    me.ascendril_board.aeroblast_hit();
                    me.damage_stat_percent(SType::Health, AEROBLAST_DAMAGE_PERCENT);
                });
            } else if combat_action.annotation.eq("stun") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_flag(FType::Stun, true);
                    me.ascendril_board.aeroblast_stun_hit();
                    me.damage_stat_percent(SType::Health, AEROBLAST_STUN_DAMAGE_PERCENT);
                });
            } else if combat_action.annotation.eq("willstun") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.observe_flag(FType::Dizziness, true);
                    me.observe_flag(FType::Vertigo, true);
                    me.ascendril_board.aeroblast_stun();
                });
            } else {
                let fast = agent_states
                    .get_player_hint(&combat_action.caster, &"AEROBLAST_SPEED".to_string())
                    .map(|s| s == "fast")
                    .unwrap_or(false);
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.ascendril_board.aeroblast(fast);
                });
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.cast_spell(Element::Air);
                        ascendril.use_up_resonance();
                    });
                });
            }
        }
        // Summons a stormwrath here.
        "Stormwrath" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                //
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                    ascendril.use_up_resonance();
                });
            });
            if combat_action.annotation.eq("charge") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_channel(ChannelType::Stormwrath, 4.);
                });
            }
        }
        // Constructs a fulcrum.
        "Construct" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.fulcrum_construct();
                });
            });
        }
        // Expands a fulcrum.
        "Expand" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                let room = me.room_id;
                me.assume_ascendril(&|ascendril| {
                    ascendril.fulcrum_expand(room);
                });
            });
        }
        // Contracts a fulcrum.
        "Interfuse" | "Callback" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.fulcrum_contract();
                });
            });
        }
        "Push" => {
            if combat_action.annotation.eq("") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fulcrum_push_start(combat_action.target.clone());
                    });
                });
            } else {
                agent_states.for_all_agents(&|me| {
                    if me.get_normalized_class() == Some(Class::Ascendril) {
                        me.assume_ascendril(&|ascendril| {
                            ascendril.fulcrum_push_end();
                        });
                    }
                });
            }
        }
        "Echoes" => {
            if combat_action.annotation.eq("off") {
                return Ok(());
            } else if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.echoes_on();
                    });
                });
            }
        }
        // Gives an affliction based on annotation.
        "Schism" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(aff, true);
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.schism_on();
                    });
                });
            } else if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.schism_on();
                    });
                });
            } else if combat_action.annotation.eq("hit") {
                let observations = after.clone();
                for_agent_uncertain_closure(
                    agent_states,
                    &combat_action.caster,
                    Box::new(move |you| {
                        apply_or_infer_random_afflictions(
                            you,
                            &observations,
                            Perspective::Target,
                            Some((
                                1,
                                vec![
                                    FType::Misery,
                                    FType::Recklessness,
                                    FType::Masochism,
                                    FType::Stupidity,
                                    FType::Impatience,
                                ],
                            )),
                        )
                    }),
                );
            }
        }
        // Strips Arcane.
        "Imbalance" => {
            if combat_action.annotation.eq("hit") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Arcane, false);
                });
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.imbalance_on();
                    });
                });
            } else if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.imbalance_on();
                    });
                });
            }
        }
        // Cures a random affliction.
        "Restore" => {
            let observations = after.clone();
            let perspective = agent_states.get_perspective(&combat_action);
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_strike_random_cure(
                        me,
                        &observations,
                        perspective,
                        (1, RANDOM_CURES.to_vec()),
                    );
                    let mut duration = 20.0;
                    if me.is(FType::Laxity) {
                        duration += 2.0;
                    }
                    apply_or_infer_balance(me, (BType::ClassCure1, duration), &observations);
                },
            );
        }
        // Changes resonance.
        "Enrich" => {
            let element = match combat_action.annotation.as_str() {
                "fire" => Element::Fire,
                "water" => Element::Water,
                "air" => Element::Air,
                "spirit" => Element::Spirit,
                _ => Element::Fire,
            };
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.enrich(element);
                });
            });
        }
        "Branding" => {
            let brand = match combat_action.annotation.as_str() {
                "ember" => FType::Emberbrand,
                "frost" => FType::Frostbrand,
                "thunder" => FType::Thunderbrand,
                _ => {
                    return Ok(());
                }
            };
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.use_up_resonance();
                });
            });
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(brand, true);
            });
        }
        // Inferno still applies secondary balance on start.
        "Inferno" => {
            if combat_action.annotation.eq("start") {
                let observations = after.clone();
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    apply_or_infer_balance(me, (BType::Secondary, 8.0), &observations);
                });
            }
        }
        // Maelstrom still applies secondary balance on start.
        "Maelstrom" => {
            if combat_action.annotation.eq("start") {
                let observations = after.clone();
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    apply_or_infer_balance(me, (BType::Secondary, 8.0), &observations);
                });
            }
        }
        // Typhoon still applies secondary balance on start.
        "Typhoon" => {
            if combat_action.annotation.eq("start") {
                let observations = after.clone();
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    apply_or_infer_balance(me, (BType::Secondary, 8.0), &observations);
                });
            }
        }
        // Raises a conductive capacitance shield.
        "Capacitance" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.raise_capacitance();
                });
            });
        }
        "Flare" => {
            let observations = after.clone();
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 5.0), &observations);
            });
        }
        // Consumes brand on target; if target has Etherflux, enrich the caster.
        "Catalyst" => {
            let element = match combat_action.annotation.as_str() {
                "ember" => Element::Fire,
                "frost" => Element::Water,
                "thunder" => Element::Air,
                _ => return Ok(()),
            };
            let brand = match element {
                Element::Fire => FType::Emberbrand,
                Element::Water => FType::Frostbrand,
                Element::Air => FType::Thunderbrand,
                _ => return Ok(()),
            };
            let has_etherflux = agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::Etherflux);
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.set_flag(brand, false);
            });
            if has_etherflux {
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.enrich(element);
                    });
                });
            }
        }
        // Records that the caster used shift.
        "Shift" => {
            if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.use_shift();
                    });
                });
            }
        }
        // Records that the caster toggled degradation on.
        "Degradation" => {
            if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.degradation_on();
                    });
                });
            }
        }
        // Records that the caster toggled spiritrift on.
        "Spiritrift" => {
            if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.spiritrift_on();
                    });
                });
            }
        }
        // Enrapture: destroy fulcrum on success, consume resonance on failure.
        "Enrapture" => {
            if combat_action.annotation.eq("success") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fulcrum_destroy();
                    });
                });
            } else if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.use_up_resonance();
                    });
                });
            } else {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.channel_state
                        .channel_seconds(ChannelType::Enrapture, 10.);
                });
            }
        }
        // Annotation is health, target is mana.
        "Detect" => {
            let health = combat_action.annotation.parse::<i32>().ok();
            let mana = combat_action.target.parse::<i32>().ok();
            let time = agent_states.time;
            if let (Some(health), Some(mana)) = (health, mana) {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.set_seen_stat(SType::Health, health, time);
                    me.set_seen_stat(SType::Mana, mana, time);
                });
            }
        }
        "NoResonance" => {
            let element = match combat_action.annotation.as_str() {
                "fire" => Element::Fire,
                "water" => Element::Water,
                "air" => Element::Air,
                "spirit" => Element::Spirit,
                _ => return Ok(()),
            };
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    if ascendril.resonance_active(&element) {
                        ascendril.use_up_resonance();
                    }
                });
            });
        }
        _ => {}
    }
    Ok(())
}
