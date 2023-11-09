use crate::classes::remove_through;
use crate::timeline::*;
use crate::types::*;

lazy_static! {
    static ref RAZE_ORDER: Vec<FType> = vec![
        FType::Reflection,
        FType::Shielded,
        FType::Rebounding,
        FType::Speed,
    ];
}

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    let first_person = combat_action.caster.eq(&agent_states.me);
    let hints = agent_states.get_player_hint(&combat_action.caster, &"CALLED_VENOMS".to_string());
    match combat_action.skill.as_ref() {
        // Knifeplay non-combo attacks.
        "Bloodscourge" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Bloodscourge],
                after,
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
        }
        "Fleshbane" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fleshbane],
                after,
            );
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
        }
        "Bloodscourged" => {
            let venom = combat_action.annotation.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                if venom.eq_ignore_ascii_case("end") {
                    me.predator_board.bloodscourge_end();
                } else {
                    me.predator_board.bloodscourge_hit();
                    apply_venom(me, &venom, false);
                }
            });
        }
        "Cirisosis" => {
            let venom = combat_action.annotation.clone();
            for_agent(agent_states, &combat_action.caster, &|me| {
                if venom.eq_ignore_ascii_case("end") {
                    me.predator_board.cirisosis_lost();
                } else {
                    me.predator_board.cirisosis_shock();
                }
            });
        }
        // Knifeplay combo attacks.
        "Jab" | "Lowhook" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, 5.5, true),
                after,
            );
        }
        "Spinslash" => {
            let limb = LType::from_name(&combat_action.annotation);
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (limb, 4.0, true),
                after,
            );
        }
        "Pinprick" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Epilepsy],
                after,
            );
        }
        "Lateral" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, 6.0, true),
                after,
            );
        }
        "Vertical" | "Crescentcut" | "Butterfly" => {
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
        }
        "Trip" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Feint" => {
            let limb = LType::from_name(&combat_action.annotation);
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_predator(&|class_state| {
                    class_state.feint();
                });
            });
            for_agent(agent_states, &combat_action.target, &move |target| {
                target.set_parrying(limb);
            });
        }
        "Flashkicked" => {
            let aff = FType::from_name(&combat_action.annotation);
            if let Some(aff) = aff {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.toggle_flag(aff, true);
                });
            }
        }
        "Raze" => {
            let annotation = combat_action.annotation.clone();
            for_agent(
                agent_states,
                &combat_action.target,
                &move |me: &mut AgentState| {
                    remove_through(
                        me,
                        match annotation.as_ref() {
                            "reflection" => FType::Reflection,
                            "shield" => FType::Shielded,
                            "rebounding" => FType::Rebounding,
                            "speed" => FType::Speed,
                            _ => FType::Speed,
                        },
                        &RAZE_ORDER.to_vec(),
                    )
                },
            );
        }
        "Gouge" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Disfigurement],
                after,
            );
        }
        // Predation attacks
        "Dartshot" | "Twinshot" => {
            apply_weapon_hits(
                agent_states,
                &combat_action.caster,
                &combat_action.target,
                after,
                first_person,
                &hints,
            );
            let mut check_cirisosis = false;
            for observation in after {
                if check_cirisosis {
                    if matches!(observation, AetObservation::DiscernedCure(_, _)) {
                        for_agent(agent_states, &combat_action.target, &|me| {
                            me.predator_board.cirisosis_start();
                        });
                    } else {
                        break;
                    }
                }
                if let AetObservation::Devenoms(devenomed) = observation {
                    if devenomed == "cirisosis" {
                        check_cirisosis = true;
                    }
                }
            }
        }
        "Pheromones" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Pacifism],
                after,
            );
        }
        "Mindnumb" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Impairment],
                after,
            );
        }
        _ => {}
    }
    Ok(())
}
