use regex::Regex;

use crate::curatives::remove_in_order;
use crate::curatives::RANDOM_CURES;
use crate::non_agent::AetTimelineRoomExt;
use crate::timeline::*;
use crate::types::*;

lazy_static! {
    static ref EMBED: Regex = Regex::new(r"embed (\w+)").unwrap();
}

const RAY_DAMAGE: CType = 20;
const STILLNESS_DAMAGE: CType = 13;

fn get_damage(before: &Vec<AetObservation>, base: CType) -> CType {
    let mut damage = base;
    if before.iter().any(|obs| {
        if let AetObservation::CombatAction(action) = obs {
            action.skill.eq_ignore_ascii_case("Parallax Release")
        } else {
            false
        }
    }) {
        damage *= 6;
        damage /= 10;
    }
    damage
}

pub fn handle_sent(command: &String, agent_states: &mut AetTimelineState) {
    if let Some(captures) = EMBED.captures(command) {
        let me = agent_states.me.clone();
        agent_states.add_player_hint(
            &me,
            &"embedding",
            captures
                .get(1)
                .unwrap()
                .as_str()
                .to_string()
                .to_ascii_lowercase(),
        );
    } else if command.contains("fvibes") {
        let me = agent_states.me.clone();
        agent_states.add_player_hint(&me, &"embedding", "focus".to_string());
    }
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Refraction" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.refract(combat_action.target.clone());
                });
            });
        }
        "Embed" => {
            println!(
                "Hint: {:?}",
                agent_states.get_player_hint(&combat_action.caster, &"embedding".to_string())
            );
            if let Some(embedding) = agent_states
                .get_player_hint(&combat_action.caster, &"embedding".to_string())
                .and_then(Vibration::from_name)
            {
                if embedding == Vibration::Focus {
                    // Focus is a special case that doesn't embed.
                    // Instead, we get all of the vibrations from ourselves!
                    let who = agent_states.borrow_agent(&combat_action.caster);
                    let room_id = agent_states.borrow_me().room_id;
                    let vibrations = who
                        .check_if_siderealist(&|me| me.vibration_states())
                        .unwrap_or(vec![]);
                    println!("Found {} vibrations to embed.", vibrations.len());
                    for vibration in vibrations {
                        agent_states.observe_vibration_not_in_room(
                            vibration.1,
                            vibration.0,
                            &combat_action.caster,
                        );
                        agent_states.observe_vibration_in_room(
                            room_id,
                            vibration.0,
                            &combat_action.caster,
                            vibration.2.activate(),
                        );
                        for_agent(agent_states, &combat_action.caster, &move |me| {
                            me.assume_siderealist(&|me| {
                                me.set_vibration(vibration.0, room_id, vibration.2.activate());
                            });
                        });
                    }
                    return Ok(());
                }
                let room_id = agent_states.borrow_me().room_id;
                agent_states.observe_vibration_in_room(
                    room_id,
                    embedding,
                    &combat_action.caster,
                    VibrationState::fresh(),
                );
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_siderealist(&|me| {
                        me.embed(embedding, room_id);
                    });
                });
            }
        }
        "palpitation" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.damage_stat(SType::Health, 5);
            });
        }
        "Tones Tremors" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.0), after);
            });
        }
        "creeps" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.caster, vec![aff], after);
            }
        }
        "Tones Creeps" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Loneliness, FType::Masochism],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
        }
        "Tones Oscillate" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Muddled],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
        }
        "disorientation" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
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
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
        }
        "Tones Stridulation" => {
            for_agent(agent_states, &combat_action.target, &move |me| {
                if me.is(FType::Deafness) {
                    me.toggle_flag(FType::Deafness, false);
                } else {
                    me.toggle_flag(FType::Sensitivity, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
        }
        "cavitation" => {
            if let Some(limb) = LType::try_from_name(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.caster,
                    (limb, 3.4, true),
                    after,
                );
            }
        }
        "dissension" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.damage_stat(SType::Health, 4);
            });
        }
        "Dissension Hit" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Dissonance],
                after,
            );
        }
        "Tones Dissension" => {
            attack_first_affliction(
                agent_states,
                &combat_action.target,
                vec![FType::Dissonance],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
        }
        "plague" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.caster, vec![aff], after);
            }
        }
        "Tones Plague" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
                });
            } else {
                let perspect = agent_states.get_perspective(combat_action);
                let observations = after.clone();
                for_agent_uncertain(
                    agent_states,
                    &combat_action.target,
                    &move |me: &mut AgentState| {
                        apply_or_infer_random_afflictions(
                            me,
                            &observations,
                            perspect,
                            Some((
                                1,
                                vec![
                                    FType::Confusion,
                                    FType::Weariness,
                                    FType::Superstition,
                                    FType::Vomiting,
                                    FType::Recklessness,
                                    FType::Epilepsy,
                                    FType::Paresis,
                                    FType::Anorexia,
                                ],
                            )),
                        )
                    },
                );
            }
        }
        "Tones Crystalforest" => {
            if combat_action.target.eq_ignore_ascii_case("") {
                return Ok(());
            }
            if let Some(defense) = FType::from_name(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.toggle_flag(defense, false);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.toggle_flag(FType::Shielded, false);
                });
            }
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
                me.assume_siderealist(&|me| {
                    me.shatter_crystalforest();
                });
            });
        }
        "lullaby" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
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
            for_agent(agent_states, &combat_action.caster, &move |me| {
                apply_or_infer_balance(me, (BType::Secondary, 3.5), after);
            });
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
            if attack_hit(after) {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.damage_stat(SType::Health, 7);
                });
            }
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
                if !attack_parried(after) {
                    for_agent(agent_states, &combat_action.target, &move |me| {
                        me.siderealist_board.irradiate(limb);
                    });
                }
            }
        }
        "Eja Kodosa Mend" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.mended();
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
                .iter()
                .find(|obs| matches!(obs, AetObservation::DiscernedAfflict(_)))
                .and_then(|obs| match obs {
                    AetObservation::DiscernedAfflict(aff) => Some(aff),
                    _ => None,
                })
                .and_then(FType::from_name)
            {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.toggle_flag(defence, false);
                });
            } else {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    println!("Could not find affliction to erode: {:?}", after);
                    me.toggle_flag(FType::Shielded, false);
                });
            }
        }
        "Ray" => {
            let perspective = agent_states.get_perspective(combat_action);
            let damage = get_damage(before, RAY_DAMAGE);
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.siderealist_board.ray();
                if perspective != Perspective::Target {
                    me.damage_stat(SType::Health, damage);
                }
            });
        }
        "Enigma" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.enigmaed();
                });
            });
        }
        "Glimmercrest Hit" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Echoes],
                after,
            );
        }
        "Embody" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.embodied();
                });
            });
        }
        "Sprite Hit" => {
            attack_afflictions(
                agent_states,
                &combat_action.caster,
                vec![FType::Phosphenes],
                after,
            );
        }
        "Dustring" => {
            if combat_action.annotation.eq_ignore_ascii_case("failure") {
                if agent_states.get_perspective(combat_action) != Perspective::Target {
                    for_agent(agent_states, &combat_action.target, &move |me| {
                        if me.get_stat(SType::Health) < 60 {
                            println!(
                                "Dustring failed, but we thought the health was: {}",
                                me.get_stat(SType::Health)
                            );
                            me.set_stat(SType::Health, 65);
                        }
                    });
                }
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
                me.siderealist_board
                    .asterism_hit(me.affs_in(RANDOM_CURES.as_ref()));
            });
        }
        "Asterism Hit" => {
            for_agent_uncertain(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_random_afflictions(
                        me,
                        after,
                        Perspective::Bystander,
                        Some((
                            1,
                            me.siderealist_board
                                .get_asterism_affs()
                                .iter()
                                .filter(|aff| !me.is(**aff))
                                .map(|aff| *aff)
                                .collect(),
                        )),
                    )
                },
            );
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
        "Gleam Inflict" => {
            if let Some(star) = GleamColor::from_annotation(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_siderealist(&|me| {
                        me.inflict(star);
                    });
                });
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![star.affliction()],
                    after,
                );
            }
        }
        "Chromaflare" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.chromaflare();
                });
            });
        }
        "Chromaflare Hit" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.damage_stat(SType::Health, 5);
            });
        }
        "Parallax" => {
            println!("Parallax: {:?}", after);
            if let Some(AetObservation::Proc(weave)) = after.get(1) {
                if !weave.skill.eq_ignore_ascii_case("parallax weave") {
                    return Ok(());
                }
                let time = weave.target.parse::<f32>().unwrap_or(0.0);
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.assume_siderealist(&|me| {
                        me.weave_parallax(
                            time,
                            weave.annotation.clone(),
                            combat_action.target.clone(),
                        );
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
        "Foresight" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.foresighted();
                });
                me.toggle_flag(FType::Foresight, true);
            });
        }
        "Centrum" => {
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_siderealist(&|me| {
                    me.centrumed();
                });
                me.toggle_flag(FType::Centrum, true);
            });
        }
        "Stillness" => {
            if combat_action.annotation.eq_ignore_ascii_case("hit") {
                let damage = get_damage(before, STILLNESS_DAMAGE);
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.observe_flag(FType::Echoes, true);
                    me.damage_stat(SType::Health, damage);
                });
                if agent_states.get_perspective(&combat_action) != Perspective::Attacker {
                    for_agent(agent_states, &combat_action.caster, &move |me| {
                        if me.affs_count(&vec![
                            FType::Stupidity,
                            FType::Confusion,
                            FType::Dementia,
                            FType::Hallucinations,
                        ]) >= 4
                        {
                            me.siderealist_board.moonlet_hit();
                        }
                    });
                    attack_first_affliction(
                        agent_states,
                        &combat_action.caster,
                        vec![
                            FType::Stupidity,
                            FType::Confusion,
                            FType::Dementia,
                            FType::Hallucinations,
                        ],
                        after,
                    );
                    attack_first_affliction(
                        agent_states,
                        &combat_action.caster,
                        vec![
                            FType::Stupidity,
                            FType::Confusion,
                            FType::Dementia,
                            FType::Hallucinations,
                        ],
                        after,
                    );
                }
            } else if combat_action.annotation.eq_ignore_ascii_case("failure") {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.observe_flag(FType::Echoes, false);
                });
            } else {
                // Nothing to do.
            }
        }
        "Alteration" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation[4..].to_string()) {
                for_agent(agent_states, &combat_action.target, &move |me| {
                    me.toggle_flag(aff, false);
                });
            }
        }
        "Alteration fail" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.observe_flag(aff, false);
                });
            }
        }
        "Syzygy" => {
            if combat_action.annotation.eq_ignore_ascii_case("failure") {
                for_agent(agent_states, &combat_action.caster, &move |me| {
                    me.siderealist_board.expire_oldest_anomaly();
                });
                return Ok(());
            }
            for_agent(agent_states, &combat_action.target, &move |me| {
                me.siderealist_board.syzygy_hit();
            });
        }
        _ => {}
    }
    Ok(())
}
