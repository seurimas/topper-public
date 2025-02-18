use crate::non_agent::Appeals;
use crate::timeline::*;
use crate::types::*;

pub fn handle_combat_action(
    combat_action: &CombatAction,
    agent_states: &mut AetTimelineState,
    _before: &Vec<AetObservation>,
    after: &Vec<AetObservation>,
) -> Result<(), String> {
    if let Some(card) = Appeals::from_name(&combat_action.skill) {
        for_agent(
            agent_states,
            &combat_action.caster,
            &move |me: &mut AgentState| {
                me.persuasion_state.appeal(card);
            },
        );
    } else {
        match combat_action.skill.as_str() {
            "Rhetoric" => {
                for_agent(
                    agent_states,
                    &combat_action.caster,
                    &move |me: &mut AgentState| {
                        me.persuasion_state.rhetoric_start();
                    },
                );
            }
            "Cyclic" => {
                if let Some(cyclic) = Appeals::from_name(&combat_action.annotation) {
                    for_agent(
                        agent_states,
                        &combat_action.caster,
                        &move |me: &mut AgentState| {
                            me.persuasion_state.cyclic = Some(cyclic);
                        },
                    );
                }
            }
            _ => {}
        }
    }
    Ok(())
}
