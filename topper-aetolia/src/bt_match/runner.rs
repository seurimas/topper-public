// ── New matching types ──────────────────────────────────────────────────────

use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, Mutex},
};

use behavior_bark::unpowered::UnpoweredFunction;
use topper_core::timeline::{BaseAgentState, BaseTimeline, db::DummyDatabaseModule};

use crate::{
    agent::{AgentState, FType},
    bt::{BehaviorController, BehaviorModel, get_tree},
    bt_match::{BtMatchConfig, Divergence, format_time},
    classes::AFFLICT_VENOMS,
    observables::ActionPlan,
    timeline::{AetObservation, AetTimeSlice, AetTimeline, simulation_slice},
};

/// What a single BT state-branch predicted.
pub struct BranchPlan {
    pub skills: Vec<String>,
    /// Resolved venom names (empty when no venoms were checked for this run).
    pub venoms: Vec<String>,
}

// ── MatchRunner ─────────────────────────────────────────────────────────────

type TreeArc = Arc<
    Mutex<
        Box<
            dyn UnpoweredFunction<Model = BehaviorModel, Controller = BehaviorController>
                + Sync
                + Send,
        >,
    >,
>;

/// Runs the BT-match comparison across time slices from a combat log.
pub struct MatchRunner {
    player_name: String,
    opponent_name: String,
    tree_name: String,
    config: BtMatchConfig,
    timeline: AetTimeline,
    tree_arc: TreeArc,
    pub match_count: usize,
}

impl MatchRunner {
    pub fn new(
        player_name: String,
        opponent_name: String,
        tree_name: String,
        config: BtMatchConfig,
    ) -> Self {
        let tree_arc = get_tree(&tree_name);
        Self {
            player_name,
            opponent_name,
            tree_name,
            config,
            timeline: AetTimeline::new(),
            tree_arc,
            match_count: 0,
        }
    }

    /// Process one time slice.
    ///
    /// Returns `Ok(())` if every action in the slice matched the BT.
    /// Returns `Err(Divergence)` on the first divergence.
    pub fn process_slice(&mut self, time_slice: &AetTimeSlice) -> Result<(), Divergence> {
        let obs = time_slice.observations.as_deref().unwrap_or(&[]);

        // 1. All non-ignored combat actions by the player in this slice.
        let observed_skills: Vec<String> = obs
            .iter()
            .filter_map(|o| match o {
                AetObservation::CombatAction(ca)
                    if ca.caster == self.player_name
                        && !self.config.ignore.iter().any(|s| s.eq_ignore_ascii_case(&ca.skill)) =>
                {
                    Some(ca.skill.clone())
                }
                _ => None,
            })
            .collect();

        if observed_skills.is_empty() {
            self.timeline
                .push_time_slice(time_slice.clone(), None as Option<&DummyDatabaseModule>)
                .ok();
            return Ok(());
        }

        // 2. Venoms actually delivered in this slice.
        //    Source A: Devenoms (first-person, viewer is attacker).
        //    Source B: Afflicted → AFFLICT_VENOMS reverse-map (viewer is defender).
        //    Deduplicate by venom name.
        let mut observed_venoms: Vec<String> = Vec::new();
        let mut seen_venoms: HashSet<String> = HashSet::new();
        for o in obs {
            let venom = match o {
                AetObservation::Devenoms(v) => Some(v.clone()),
                AetObservation::Afflicted(aff_name) => FType::from_str(aff_name)
                    .ok()
                    .and_then(|ft| AFFLICT_VENOMS.get(&ft))
                    .map(|v| v.to_string()),
                _ => None,
            };
            if let Some(v) = venom {
                let key = v.to_lowercase();
                if seen_venoms.insert(key) {
                    observed_venoms.push(v);
                }
            }
        }

        // 3. Dodge / miss skipping.
        let n_skippable = obs
            .iter()
            .filter(|o| matches!(o, AetObservation::Dodges(_) | AetObservation::Misses(_)))
            .count();

        // 4. Check BT branches (single run for the whole slice).
        let branches: Vec<AgentState> = self
            .timeline
            .state
            .get_agent(&self.player_name)
            .cloned()
            .unwrap_or_else(|| vec![AgentState::get_base_state()]);

        let (any_match, branch_plans) =
            self.check_branches(&observed_skills, &observed_venoms, n_skippable, &branches);

        if any_match {
            let skill_str = observed_skills.join(", ");
            if observed_venoms.is_empty() {
                println!(
                    "[{}] MATCH   {} -> {}",
                    format_time(time_slice.time),
                    self.player_name,
                    skill_str
                );
            } else {
                println!(
                    "[{}] MATCH   {} -> {} (venoms: {})",
                    format_time(time_slice.time),
                    self.player_name,
                    skill_str,
                    observed_venoms.join(", ")
                );
            }
            self.match_count += observed_skills.len();

            // Fire skill traps: if any planned skill has a trap, inject
            // those observations as a synthetic time slice.
            let planned_skills: Vec<String> = branch_plans
                .iter()
                .flat_map(|bp| bp.skills.iter())
                .cloned()
                .collect();
            let mut trap_obs: Vec<AetObservation> = Vec::new();
            for skill in &planned_skills {
                if let Some(obs) = self.config.skill_traps.get(skill) {
                    trap_obs.extend(obs.iter().cloned());
                }
            }
            if !trap_obs.is_empty() {
                let trap_slice = simulation_slice(trap_obs, time_slice.time);
                self.timeline
                    .push_time_slice(trap_slice, None as Option<&DummyDatabaseModule>)
                    .ok();
            }
        } else {
            let player_state = self.timeline.state.borrow_agent(&self.player_name).clone();
            let opponent_state = self
                .timeline
                .state
                .borrow_agent(&self.opponent_name)
                .clone();
            return Err(Divergence {
                time: time_slice.time,
                player_name: self.player_name.clone(),
                opponent_name: self.opponent_name.clone(),
                observed_skills,
                observed_venoms,
                branch_plans,
                matches_before: self.match_count,
                player_state,
                opponent_state,
            });
        }

        self.timeline
            .push_time_slice(time_slice.clone(), None as Option<&DummyDatabaseModule>)
            .ok();
        Ok(())
    }

    /// Print a final summary line.
    pub fn finish(&self) {
        if self.match_count == 0 {
            println!("No actions found for '{}' in this log.", self.player_name);
        } else {
            println!("\nFULL MATCH — {} actions all matched.", self.match_count);
        }
    }

    /// Run the BT against every state branch; return whether any branch matched
    /// all observed skills and venoms.
    fn check_branches(
        &self,
        observed_skills: &[String],
        observed_venoms: &[String],
        n_skippable: usize,
        branches: &[AgentState],
    ) -> (bool, Vec<BranchPlan>) {
        let class_hint = self.tree_name.split('/').next().unwrap_or("");
        let mut tree_guard = self.tree_arc.lock().unwrap();
        let mut branch_plans: Vec<BranchPlan> = Vec::new();

        for branch in branches {
            let mut branch_tl = self.timeline.branch();
            branch_tl.state.me = self.player_name.clone();
            branch_tl
                .state
                .agent_states
                .insert(self.player_name.clone(), vec![branch.clone()]);

            let mut controller = BehaviorController::default();
            controller.plan = ActionPlan::new(&self.player_name);
            controller.target = Some(self.opponent_name.clone());
            match class_hint {
                "predator" => controller.init_predator(),
                "monk" => controller.init_monk(),
                "zealot" => controller.init_zealot(),
                "infiltrator" => controller.init_infiltrator(),
                _ => {}
            }

            tree_guard.reset(&branch_tl);
            tree_guard.resume_with(&branch_tl, &mut controller);

            let skills = controller.plan.get_skills();

            // Resolve planned venoms when the slice has venom activity.
            let venoms: Vec<String> = if !observed_venoms.is_empty() || n_skippable > 0 {
                let opp_state = branch_tl.state.borrow_agent(&self.opponent_name).clone();
                controller
                    .get_venoms_from_plan(observed_venoms.len() + n_skippable, &opp_state, &vec![])
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                vec![]
            };

            let skills_match = observed_skills
                .iter()
                .all(|s| skills.iter().any(|p| p.eq_ignore_ascii_case(s)));
            let venoms_match = observed_venoms
                .iter()
                .all(|v| venoms.iter().any(|p| p.eq_ignore_ascii_case(v)));

            branch_plans.push(BranchPlan { skills, venoms });

            if skills_match && venoms_match {
                return (true, branch_plans);
            }
        }

        (false, branch_plans)
    }
}
