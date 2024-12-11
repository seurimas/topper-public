use crate::curatives::remove_in_order;
use crate::curatives::RANDOM_CURES;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Tones Tremors" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Creeps" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            }
        }
        "Tones Creeps" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Loneliness, FType::Masochism],
                after,
            );
        }
        "Tones Oscillate" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Muddled],
                after,
            );
        }
        "Disorientation" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness],
                after,
            );
        }
        "Tones Disorientation" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Epilepsy, FType::Berserking],
                after,
            );
        }
        "Tones Stridulation" => {
            for_agent(agent_states, &combat_action.target, &move |me| {
                if me.is(FType::Deafness) {
                    me.toggle_flag(FType::Deafness, false);
                } else {
                    me.toggle_flag(FType::Sensitivity, true);
                }
            });
        }
        "Dissension" => {
            if combat_action.annotation.eq_ignore_ascii_case("hit") {
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Dissonance],
                    after,
                );
            }
        }
        "Tones Dissension" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Dissonance],
                after,
            );
        }
        "Plague" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            }
        }
        "Tones Plague" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            }
        }
        "Lullaby" => {
            for_agent(agent_states, &combat_action.target, &move |me| {
                if me.is(FType::Insomnia) {
                    me.toggle_flag(FType::Insomnia, false);
                } else if !me.is(FType::Asleep) {
                    me.toggle_flag(FType::Asleep, true);
                } else if me.is(FType::Instawake) {
                    me.toggle_flag(FType::Instawake, false);
                }
            });
        }
        "Tones Lullaby" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Asleep, FType::Hypersomnia],
                after,
            );
        }
        "Vayua Attack" => {
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
        }
        "Ontesme Illgrasp" => {
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.set_count(FType::Illgrasp, 4);
            });
        }
        "Averroes Bolt" => {
            if let Some(limb) = LType::try_from_name(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, 15., true),
                    after,
                );
            }
        }
        "Eja Kodosa Mend" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.use_mend();
                });
                for limb in vec![
                    LType::LeftArmDamage,
                    LType::RightArmDamage,
                    LType::LeftLegDamage,
                    LType::RightLegDamage,
                ] {
                    if !me.limb_damage.broken(limb) && me.limb_damage.crippled(limb) {
                        me.limb_damage.set_limb_crippled(limb, false);
                        break;
                    }
                }
            });
        }
        "Erode" => {
            if let Some(defence) = after
                .get(0)
                .and_then(|obs| match obs {
                    AetObservation::DiscernedAfflict(aff) => Some(aff),
                    _ => None,
                })
                .and_then(FType::from_name)
            {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.toggle_flag(defence, false);
                });
            }
        }
        "Ray" => {
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.siderealist_board.ray();
            });
        }
        "Glimmercrest Hit" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Echoes],
                after,
            );
        }
        "Sprite Hit" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Phosphenes],
                after,
            );
        }
        "Dustring" => {
            if combat_action.annotation.eq_ignore_ascii_case("failure") {
                return Ok(());
            }
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.siderealist_board.dustring_hit();
            });
        }
        "Asterism" => {
            if combat_action.annotation.eq_ignore_ascii_case("failure") {
                return Ok(());
            }
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.siderealist_board.asterism_hit();
            });
        }
        "Moonlet" => {
            if combat_action.annotation.eq_ignore_ascii_case("start") {
                let mut duration = 4.;
                let target = agent_states.borrow_agent(&combat_action.target);
                for limb in vec![
                    LType::LeftArmDamage,
                    LType::RightArmDamage,
                    LType::LeftLegDamage,
                    LType::RightLegDamage,
                ] {
                    if !target.limb_damage.broken(limb) {
                        continue;
                    }
                    duration -= 0.5;
                }
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.channel_state
                        .channel_seconds(ChannelType::Moonlet, duration);
                });
            } else if combat_action.annotation.eq_ignore_ascii_case("end") {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.siderealist_board.moonlet_hit();
                });
            }
        }
        "Gleam" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.use_gleam();
                });
            });
        }
        "Parallax" => {
            if let Some(AetObservation::Proc(weave)) = after.get(0) {
                if !weave.skill.eq_ignore_ascii_case("parallax weave") {
                    return Ok(());
                }
                let time = weave.target.parse::<f32>().unwrap_or(0.0);
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_siderealist(&|me| {
                        me.weave_parallax(time, weave.target.clone(), combat_action.target.clone());
                    });
                });
            }
        }
        "Parallax Release" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.release_parallax();
                });
            });
        }
        "Eventide" => {
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
                    apply_or_infer_balance(me, (BType::ClassCure1, 18.0), &observations);
                },
            );
        }
        "Equinox" => {
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
                    apply_or_infer_balance(me, (BType::ClassCure2, 20.0), &observations);
                },
            );
        }
        "Stillness" => {
            if combat_action.annotation.eq_ignore_ascii_case("hit") {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.observe_flag(FType::Echoes, true);
                });
            } else if combat_action.annotation.eq_ignore_ascii_case("failure") {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.observe_flag(FType::Echoes, false);
                });
            } else {
                // Nothing to do.
            }
        }
        _ => {}
    }
    Ok(())
}
