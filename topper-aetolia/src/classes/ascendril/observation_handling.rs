use crate::curatives::MENTAL_AFFLICTIONS;
use crate::curatives::RANDOM_CURES;
use crate::non_agent::AetTimelineDenizenExt;
use crate::non_agent::AetTimelineRoomExt;
use crate::timeline::*;
use crate::types::*;

use super::phenomenon_in_room;

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
                vec![FType::Blisters, FType::LimpVeins],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                });
            });
        }
        // Afflicts with ashenfeet
        "Ashenfeet" => {
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
        // Gives me 4 stacks of fireburst, or hits for ablaze
        "Fireburst" => {
            if combat_action.target.is_empty() {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fireburst_fill();
                    })
                });
            } else {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.fireburst_decrement();
                        ascendril.cast_spell(Element::Fire);
                    });
                });
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Ablaze, true);
                });
            }
        }
        // Gives the target a Blazwhirl phenomenon
        "Blazewhirl" => {
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                    ascendril.try_claim(PhenomenaState::Blazewhirl);
                });
            });
        }
        // With ablaze, gives emberbrand.
        "Conflagrate" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Ablaze) {
                    me.set_flag(FType::Emberbrand, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                });
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
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Fire);
                });
            });
        }
        // Conditional attack, dealing head trauma and more
        "Pyroclast" => {
            let blazewhirl = phenomenon_in_room(
                agent_states,
                agent_states.get_room_id(),
                PhenomenaState::Blazewhirl,
            );
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
            }

            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.use_up_resonance();
                });
            });
        }
        // Strip audit defenses
        "Disintegrate" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Arcane, false);
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.use_up_resonance();
                });
            });
        }
        // Freezes twice
        "Coldsnap" => {
            attack_first_affliction(
                agent_states,
                // &combat_action.target,
                &"Gherond".to_string(),
                vec![FType::Shivering, FType::Frigid, FType::Frozen],
                after,
            );
            attack_first_affliction(
                agent_states,
                // &combat_action.target,
                &"Gherond".to_string(),
                vec![FType::Shivering, FType::Frigid, FType::Frozen],
                after,
            );
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        /**
        If shivering, knock of balance. If frigid, strip levitation. If frozen, give disrupted.
        */
        "Iceray" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Shivering) {
                    me.set_balance(BType::Balance, 0.5);
                }
                if me.is(FType::Frigid) {
                    me.set_flag(FType::Levitation, false);
                }
                if me.is(FType::Frozen) {
                    me.set_flag(FType::Disrupted, true);
                }
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
                    ascendril.try_claim(PhenomenaState::Glazeflow);
                });
            });
        }
        // With shivering, gives direfrost.
        "Direfrost" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Shivering) {
                    me.set_flag(FType::Direfrost, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        // Gives 3 icicles.
        "Icicle" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.ascendril_board.icicles_spawn();
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        // Shatters icicles.
        "Shatter" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.ascendril_board.shatter();
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Water);
                });
            });
        }
        // If no leivitation, give ice_encased. If shivering, give hobbled. If glazeflow in room, give frozen_feet.
        "Crystalize" => {
            let glazeflow_in_room = phenomenon_in_room(
                agent_states,
                agent_states.get_room_id(),
                PhenomenaState::Glazeflow,
            );
            for_agent(agent_states, &combat_action.target, &|me| {
                if !me.is(FType::Levitation) {
                    me.set_flag(FType::IceEncased, true);
                }
                if me.is(FType::Shivering) {
                    me.set_flag(FType::Hobbled, true);
                }
                if glazeflow_in_room {
                    me.set_flag(FType::FrozenFeet, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
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
                    ascendril.use_up_resonance();
                });
            });
        }
        // Destroys shield or gives fallen.
        "Windlance" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Shielded) {
                    me.set_flag(FType::Shielded, false);
                } else {
                    me.set_flag(FType::Fallen, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Gives vertigo and confusion. If they have thunderbrand, also cause recklessness.
        "Pressurize" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Vertigo, true);
                me.set_flag(FType::Confusion, true);
                if me.is(FType::Thunderbrand) {
                    me.set_flag(FType::Recklessness, true);
                }
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Gives paresis or turns into paralysis.
        "Arcbolt" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Paresis) {
                    me.set_flag(FType::Paralysis, true);
                } else {
                    me.set_flag(FType::Paresis, true);
                }
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
                    ascendril.try_claim(PhenomenaState::Electrosphere);
                });
            });
        }
        // Gives dizziness and stupidity. If they have both and another mental, give thunderbrand.
        "Thunderclap" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                if me.is(FType::Dizziness)
                    && me.is(FType::Stupidity)
                    && me.affs_count(&MENTAL_AFFLICTIONS.to_vec()) >= 3
                {
                    me.set_flag(FType::Thunderbrand, true);
                }
                me.set_flag(FType::Dizziness, true);
                me.set_flag(FType::Stupidity, true);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Knocks unconcious.
        "Feedback" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                me.set_flag(FType::Unconscious, true);
            });
            for_agent(agent_states, &combat_action.caster, &|me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.cast_spell(Element::Air);
                });
            });
        }
        // Gives vomiting and fallen. Aeroblasts.
        "Aeroblast" => {
            if combat_action.annotation.eq("hit") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Vomiting, true);
                    me.set_flag(FType::Fallen, true);
                    me.ascendril_board.aeroblast_hit();
                });
            } else {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.ascendril_board.aeroblast();
                });
            }

            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.use_up_resonance();
                });
            });
        }
        // Summons a stormwrath here.
        "Stormwrath" => {
            for_agent(agent_states, &combat_action.target, &|me| {
                //
            });
            for_agent(agent_states, &combat_action.caster, &move |me| {
                me.assume_ascendril(&|ascendril| {
                    ascendril.use_up_resonance();
                });
            });
        }
        // Constructs a fulcrum.
        "Fulcrum" => {
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
        // Gives an affliction based on annotation.
        "Schism" => {
            if let Some(aff) = FType::from_name(&combat_action.annotation) {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(aff, true);
                });
            } else if combat_action.annotation.eq("on") {
                for_agent(agent_states, &combat_action.caster, &|me| {
                    me.assume_ascendril(&|ascendril| {
                        ascendril.schism_on();
                    });
                });
            }
        }
        // Strips Arcane.
        "Imbalance" => {
            if combat_action.annotation.eq("hit") {
                for_agent(agent_states, &combat_action.target, &|me| {
                    me.set_flag(FType::Arcane, false);
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
                    apply_or_infer_balance(me, (BType::ClassCure1, 20.0), &observations);
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
        _ => {}
    }
    Ok(())
}
