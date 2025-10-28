use crate::{agent::*, classes::zealot::constants::*, timeline::*};

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    match combat_action.skill.as_ref() {
        "Welts" => {
            let limb = match combat_action.annotation.as_ref() {
                "head" => LType::HeadDamage,
                "torso" => LType::TorsoDamage,
                "left arm" => LType::LeftArmDamage,
                "right arm" => LType::RightArmDamage,
                "left leg" => LType::LeftLegDamage,
                "right leg" => LType::RightLegDamage,
                _ => LType::SIZE, // I don't want to panic
            };
            for_agent(agent_states, &combat_action.caster, &move |you| {
                you.limb_damage.welt(limb);
            });
        }
        "Hellcat" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                if you.is(FType::Ablaze) {
                    you.tick_flag_up(FType::Ablaze);
                }
            });
        }
        "WeltHit" => {
            let limb = match combat_action.annotation.as_ref() {
                "head" => LType::HeadDamage,
                "torso" => LType::TorsoDamage,
                "left arm" => LType::LeftArmDamage,
                "right arm" => LType::RightArmDamage,
                "left leg" => LType::LeftLegDamage,
                "right leg" => LType::RightLegDamage,
                _ => LType::SIZE, // I don't want to panic
            };
            attack_limb_damage(
                agent_states,
                &combat_action.caster,
                (limb, 6.5, true),
                after,
            );
        }
        "Dislocated" => {
            let (limb, dislocation) = match combat_action.annotation.as_ref() {
                "left arm" => (LType::LeftArmDamage, FType::LeftArmDislocated),
                "right arm" => (LType::RightArmDamage, FType::RightArmDislocated),
                "left leg" => (LType::LeftLegDamage, FType::LeftLegDislocated),
                "right leg" => (LType::RightLegDamage, FType::RightLegDislocated),
                _ => (LType::SIZE, FType::SIZE), // I don't want to panic
            };
            for_agent(agent_states, &combat_action.caster, &move |you| {
                let limb_state = you.get_limb_state(limb);
                let damage_change = 33.34 - limb_state.damage;
                you.limb_damage.set_limb_broken(limb, true);
                you.toggle_flag(dislocation, false);
            });
        }
        "InfernalSeal" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                you.observe_flag(FType::Ablaze, true);
                you.toggle_flag(FType::InfernalSeal, true);
            });
        }
        "Zenith" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                you.assume_zealot(|zealot| zealot.zenith.initiate());
            });
        }
        "Pyromania" => match combat_action.annotation.as_ref() {
            "" => {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.assume_zealot(|zealot| zealot.pyromania.activate(2000));
                });
            }
            "hit" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        if me.is(FType::Ablaze) {
                            me.tick_flag_up(FType::Ablaze);
                        }
                    },
                );
            }
            "fall" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.toggle_flag(FType::Fallen, true);
                    },
                );
            }
            "shield" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &|me: &mut AgentState| {
                        me.toggle_flag(FType::Shielded, false);
                    },
                );
            }
            _ => {}
        },
        "Heelrush" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_ONE_DAMAGE, true),
                    after,
                );
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.set_channel_with_limb(ChannelType::Heelrush, 3.25, limb);
                    },
                );
            }
        }
        "Heelrush Two" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_TWO_DAMAGE, true),
                    after,
                );
            }
        }
        "Heelrush Three" => {
            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, HEELRUSH_THREE_DAMAGE, true),
                    after,
                );
            }
        }
        "Direblow" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    me.set_channel(ChannelType::Direblow, 2.0);
                },
            );
        }
        "Direblow Weak" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, DIREBLOW_WEAK_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Lightwound],
                after,
            );
        }
        "Direblow Strong" => {
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, DIREBLOW_STRONG_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Lightwound, FType::Deepwound],
                after,
            );
        }
        "Risekick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.65),
                after,
            );
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, RISEKICK_DAMAGE, true),
                after,
            );
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.toggle_flag(FType::Fallen, false);
                },
            );
        }
        "Pummel" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.65),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, PUMMEL_DAMAGE, true),
                    after,
                );
            }
        }
        "Wanekick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            if let Ok(limb) = get_limb_damage(&combat_action.annotation) {
                attack_limb_damage(
                    agent_states,
                    &combat_action.target,
                    (limb, WANEKICK_DAMAGE, true),
                    after,
                );
            }
        }
        "Clawtwist" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::TorsoDamage, CLAWTWIST_DAMAGE, true),
                after,
            );
        }
        "Sunkick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::HeadDamage, SUNKICK_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Dizziness, FType::Stupidity],
                after,
            );
        }
        "Edgekick" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );
            attack_limb_damage(
                agent_states,
                &combat_action.target,
                (LType::HeadDamage, EDGEKICK_DAMAGE, true),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::CrippledThroat, FType::Stuttering],
                after,
            );
        }
        "Palmforce" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Fallen],
                after,
            );
        }
        "Twinpress" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.25),
                after,
            );

            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::MuscleSpasms, FType::Stiffness],
                after,
            );
        }
        "Dislocate" => {
            apply_combo_balance(
                agent_states,
                &combat_action.caster,
                (BType::Balance, 3.5),
                after,
            );

            let aff = match combat_action.annotation.as_ref() {
                "left arm" => Some(FType::LeftArmDislocated),
                "right arm" => Some(FType::RightArmDislocated),
                "left leg" => Some(FType::LeftLegDislocated),
                "right leg" => Some(FType::RightLegDislocated),
                _ => None,
            };
            if let Some(aff) = aff {
                attack_afflictions(agent_states, &combat_action.target, vec![aff], after);
            }
        }
        "Anklepin" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::SoreAnkle],
                after,
            );
            for_agent(agent_states, &combat_action.target, &|you| {
                you.limb_damage.dewelt(LType::LeftLegDamage);
                you.limb_damage.dewelt(LType::RightLegDamage);
            });
        }
        "Trammel" => {
            let amputation = match combat_action.annotation.as_ref() {
                "left arm" => FType::LeftArmAmputated,
                "right arm" => FType::RightArmAmputated,
                "left leg" => FType::LeftLegAmputated,
                "right leg" => FType::RightLegAmputated,
                _ => FType::SIZE, // I don't want to panic
            };
            attack_afflictions(agent_states, &combat_action.target, vec![amputation], after);
        }
        "Wristlash" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::SoreWrist],
                after,
            );
            for_agent(agent_states, &combat_action.target, &|you| {
                you.limb_damage.dewelt(LType::LeftArmDamage);
                you.limb_damage.dewelt(LType::RightArmDamage);
            });
        }
        "Descent" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Backstrain],
                after,
            );
            for_agent(agent_states, &combat_action.target, &|you| {
                you.limb_damage.dewelt(LType::TorsoDamage);
            });
        }
        "Uprise" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Whiplash],
                after,
            );
            for_agent(agent_states, &combat_action.target, &|you| {
                you.limb_damage.dewelt(LType::HeadDamage);
            });
        }
        "Jawcrack" => {
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::WateryEyes],
                after,
            );
        }
        "Rejection" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_flag(FType::Rebounding, true);
                },
            );
        }
        "Pendulum" => {
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                    me.set_balance(BType::pendulum(), 20.0);
                },
            );
            let annotation = combat_action.annotation.clone();
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.rotate_limbs(annotation == "anti-clockwise");
                },
            );
        }
        "Whipburst" => {
            for_agent(agent_states, &combat_action.target, &|you| {
                if you.is(FType::Ablaze) {
                    you.tick_flag_up(FType::Ablaze);
                }
            });
        }
        "Quicken" => {
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                },
            );
            for_agent(agent_states, &combat_action.target, &|you| {
                you.tick_flag_up(FType::Ablaze);
                you.tick_flag_up(FType::Ablaze);
                you.tick_flag_up(FType::Ablaze);
            });
        }
        "Infernal" => {
            if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.limb_damage.set_limb_broken(LType::TorsoDamage, false);
                });
            } else {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                    },
                );
                let observations = after.clone();
                for_agent(agent_states, &combat_action.target, &|you| {
                    you.set_flag(FType::InfernalSeal, true);
                });
            }
        }
        "InfernalShroud" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                you.set_flag(FType::Shielded, false);
            });
        }
        "InfernalShrouded" => {
            for_agent(agent_states, &combat_action.caster, &|you| {
                you.toggle_flag(FType::InfernalSeal, false);
                you.toggle_flag(FType::InfernalShroud, true);
            });
        }
        "Scorch" => {
            let observations = after.clone();
            for_agent(
                agent_states,
                &combat_action.caster,
                &move |me: &mut AgentState| {
                    apply_or_infer_balance(me, (BType::Equil, 2.0), &observations);
                },
            );
            attack_afflictions(
                agent_states,
                &combat_action.target,
                vec![FType::Ablaze],
                after,
            );
        }
        "Heatspear" => {
            if combat_action.annotation.eq("failure") {
                for_agent(agent_states, &combat_action.caster, &|you| {
                    you.observe_flag(FType::Ablaze, false);
                });
            } else {
                let observations = after.clone();
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        apply_or_infer_balance(me, (BType::Equil, 3.0), &observations);
                    },
                );
                attack_afflictions(
                    agent_states,
                    &combat_action.target,
                    vec![FType::Ablaze, FType::Heatspear],
                    after,
                );
            }
        }
        "Firefist" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::firefist(), 80.0);
                },
            );
        }
        "Wrath" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::wrath(), 30.0);
                },
            );
        }
        "Dull" => {
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    me.set_flag(FType::Indifference, true);
                },
            );
        }
        "Immolation" => {
            if combat_action.annotation.eq("failure") {
                for_agent(
                    agent_states,
                    &combat_action.target,
                    &|me: &mut AgentState| {
                        me.observe_flag(FType::Ablaze, false);
                    },
                );
            }
        }
        "Recover" => {
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
        "Hackles" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::Secondary, 6.5);
                },
            );
        }
        "Disable" => {
            for_agent(
                agent_states,
                &combat_action.caster,
                &|me: &mut AgentState| {
                    me.set_balance(BType::ClassAttack4, 90.0);
                },
            );
            for_agent(
                agent_states,
                &combat_action.target,
                &|me: &mut AgentState| {
                    // me.set_balance(BType::Disabled, 12.0);
                },
            );
        }
        _ => {}
    }
    Ok(())
}
