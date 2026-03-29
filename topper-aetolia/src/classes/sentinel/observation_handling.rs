use super::constants::*;
use crate::classes::*;
use crate::observables::*;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Might" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    let mut duration = 20.0;
                    if me.is(FType::Laxity) {
                        duration += 2.0;
                    }
                    me.set_balance(BType::ClassCure1, duration);
                },
            );
        }
        "Slash" | "Stab" | "Slice" | "Thrust" | "Ambush" | "Flourish" => {
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
        }
        "Pierce" | "Sever" => {
            let mut target = &combat_action.target;
            let mut limb_hit = None;
            let mut limb_damaged = false;
            for observation in after {
                match observation {
                    AetObservation::Damaged(_who, limb) => {
                        limb_hit = get_limb_damage(limb).ok();
                        limb_damaged = true;
                    }
                    AetObservation::Connects(limb) => {
                        limb_hit = get_limb_damage(limb).ok();
                        limb_damaged = false;
                    }
                    AetObservation::Rebounds => {
                        target = &combat_action.caster;
                    }
                    AetObservation::CombatAction(action) => {
                        if action != combat_action {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(limb_hit) = limb_hit {
                for_agent(agent_states, target, &move |you: &mut AgentState| {
                    if limb_damaged {
                        you.set_limb_damage(limb_hit, DAMAGED_VALUE, true);
                        you.limb_damage.set_limb_broken(limb_hit, true);
                    } else {
                        you.set_flag(limb_hit.crippled().unwrap(), true);
                    }
                });
            } else {
                println!("No limb hit...");
            }
        }
        "Dualraze" => {
            let razed = combat_action.annotation.clone();
            for_agent(
                agent_states,
                &combat_action.target,
                &move |mut you: &mut AgentState| {
                    remove_through(
                        you,
                        match razed.as_ref() {
                            "rebounding" => FType::Rebounding,
                            "shield" => FType::Shielded,
                            _ => FType::Speed,
                        },
                        &DUALRAZE_ORDER.to_vec(),
                    );
                },
            );
        }
        "Reave" => {
            let razed = combat_action.annotation.clone();
            for_agent(
                agent_states,
                &combat_action.target,
                &move |mut you: &mut AgentState| {
                    remove_through(
                        you,
                        match razed.as_ref() {
                            "shielded" => FType::Shielded,
                            _ => FType::Rebounding,
                        },
                        &REAVE_ORDER.to_vec(),
                    );
                },
            );
            if let Some(def_flag) = FType::from_name(&combat_action.annotation) {
                attack_strip(agent_states, &combat_action.caster, vec![def_flag], after);
            }
        }
        "Twirl" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Confusion],
                after,
            );
        }
        "Throatcrush" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::DestroyedThroat],
                after,
            );
        }
        "Lysirine" => match combat_action.annotation.as_ref() {
            "hot" => {
                // Hot burn: Paresis+Impatience+Confusion, but each upgrades if the target
                // already has that affliction (Paresis→Arrhythmia, Impatience→Hallucinations,
                // Confusion→Stupidity).
                let caster = combat_action.caster.clone();
                let affs = {
                    let you = agent_states.borrow_agent(&caster);
                    let a1 = if you.is(FType::Paresis) {
                        FType::Arrhythmia
                    } else {
                        FType::Paresis
                    };
                    let a2 = if you.is(FType::Impatience) {
                        FType::Hallucinations
                    } else {
                        FType::Impatience
                    };
                    let a3 = if you.is(FType::Confusion) {
                        FType::Stupidity
                    } else {
                        FType::Confusion
                    };
                    vec![a1, a2, a3]
                };
                attack_afflictions(agent_states, &caster, affs, after);
                for_agent(agent_states, &caster, &|you: &mut AgentState| {
                    you.resin_state.hot_burn();
                });
            }
            _ => {}
        },
        "Crosscut" => {
            if agent_states
                .borrow_agent(&combat_action.target)
                .is(FType::Impairment)
            {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Addiction],
                    after,
                );
            } else {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Impairment],
                    after,
                );
            }
        }
        "Weaken" => {
            // Annotation is "left arm", "right arm", "left leg", or "right leg".
            // Arms: drop parry + 278 limb damage. Legs: lethargy + 278 limb damage.
            let rebounds = after.iter().any(|obs| matches!(obs, AetObservation::Rebounds));
            if !rebounds {
                let annotation = combat_action.annotation.clone();
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &move |you: &mut AgentState| match annotation.as_ref() {
                        "left arm" => {
                            you.limb_damage.adjust_limb(LType::LeftArmDamage, 278);
                            you.clear_parrying();
                        }
                        "right arm" => {
                            you.limb_damage.adjust_limb(LType::RightArmDamage, 278);
                            you.clear_parrying();
                        }
                        "left leg" => {
                            you.limb_damage.adjust_limb(LType::LeftLegDamage, 278);
                        }
                        "right leg" => {
                            you.limb_damage.adjust_limb(LType::RightLegDamage, 278);
                        }
                        _ => {}
                    },
                );
                match combat_action.annotation.as_ref() {
                    "left leg" | "right leg" => {
                        attack_afflictions(
                            agent_states,
                            &combat_action.target,
                            vec![FType::Lethargy],
                            after,
                        );
                    }
                    _ => {}
                }
            }
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
        }
        "Trip" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Balance, 2.25);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Slam" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Balance, 2.25);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Epilepsy, FType::Laxity],
                after,
            );
        }
        "Gouge" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Balance, 2.25);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Impatience],
                after,
            );
        }
        "Heartbreaker" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Balance, 2.25);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Arrhythmia],
                after,
            );
        }
        "Slit" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Balance, 2.25);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::CrippledThroat],
                after,
            );
        }
        "Trientia" => match combat_action.annotation.as_ref() {
            "hot" => {
                let caster = combat_action.caster.clone();
                for_agent(agent_states, &caster, &|you: &mut AgentState| {
                    you.resin_state.hot_burn();
                });
            }
            "cold" => {
                // Cold burn (9 cycles): if the target has dizziness, the fumes cause faintness.
                let caster = combat_action.caster.clone();
                let has_dizziness = agent_states.borrow_agent(&caster).is(FType::Dizziness);
                if has_dizziness {
                    attack_afflictions(
                        agent_states,
                        &caster,
                        vec![FType::Faintness],
                        after,
                    );
                }
                for_agent(agent_states, &caster, &|you: &mut AgentState| {
                    you.resin_state.cold_burn();
                });
            }
            _ => {}
        },
        // State tracking for remaining resins. Hot burns clear the resin state; cold burns
        // advance the tick counter. Specific cold/hot effects (damage, allergy severity,
        // coagulation, etc.) are not modelled here.
        "Pyrolum" | "Corsin" | "Harimel" | "Glauxe" | "Badulem" => {
            match combat_action.annotation.as_ref() {
                "hot" => {
                    let caster = combat_action.caster.clone();
                    for_agent(agent_states, &caster, &|you: &mut AgentState| {
                        you.resin_state.hot_burn();
                    });
                }
                "cold" => {
                    let caster = combat_action.caster.clone();
                    for_agent(agent_states, &caster, &|you: &mut AgentState| {
                        you.resin_state.cold_burn();
                    });
                }
                _ => {}
            }
        }
        "Whirl" => {
            // Both the initial hit and the delayed second hit arrive under this skill.
            // The second hit fires while off balance and requires no equilibrium.
            let observations = after.clone();
            let first_person = combat_action.caster.eq(&agent_states.me);
            let hints =
                agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
        }
        "Combust" => {
            // Ignite the resin currently on the target. Also opens a Flourish/Brandish window
            // (handled by the planner — no state flag needed here).
            for_agent(
                agent_states,
                &combat_action.target,
                &|you: &mut AgentState| {
                    you.resin_state.ignite();
                },
            );
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Balance, 2.65), &observations);
                },
            );
        }
        "Alacrity" => {
            // Grants three uses of reduced balance cost for the next shots/splatter/lays.
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.assume_sentinel(|s| s.alacrity = 3);
                    me.set_balance(BType::Equil, 2.25);
                },
            );
        }
        // Thrust (Sentinel) / Inveigle (Executor): when the target has Arrhythmia, this
        // should also knock their pill balance. TODO: model pill balance knock duration
        // and wire it up here once confirmed.
        // Passive actions
        "Gyrfalcon" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Disfigurement],
                after,
            );
        }
        "Elk" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Fallen],
                after,
            );
        }
        "Weasel" => {
            if let Some(def_flag) = FType::from_name(&combat_action.annotation) {
                attack_strip(agent_states, &combat_action.caster, vec![def_flag], after);
            }
        }
        "Cockatrice" | "Crocodile" | "Raloth" => {
            if let Some(aff_flag) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.caster, vec![aff_flag], after);
            }
        }
        "Daunt" => match combat_action.annotation.as_ref() {
            "coyote" | "darkhound" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.set_balance(BType::Equil, 2.25);
                    },
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Claustrophobia],
                    after,
                );
            }
            "raloth" | "brutaliser" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.set_balance(BType::Equil, 2.25);
                    },
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Agoraphobia],
                    after,
                );
            }
            "crocodile" | "eviscerator" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.set_balance(BType::Equil, 2.25);
                    },
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Loneliness],
                    after,
                );
            }
            "cockatrice" | "terrifier" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.set_balance(BType::Equil, 2.25);
                    },
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Mania],
                    after,
                );
            }
            _ => {}
        },
        "Icebreath" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Equil, 2.25);
                },
            );
            attack_strip_or_afflict(
                agent_states,
                &combat_action.target,
                vec![FType::Insulation, FType::Shivering, FType::Frozen],
                after,
            );
        }
        "Icewyrm" => {
            attack_strip_or_afflict(
                agent_states,
                &combat_action.caster,
                vec![FType::Insulation, FType::Shivering, FType::Frozen],
                after,
            );
        }
        _ => {}
    }
    Ok(())
}
