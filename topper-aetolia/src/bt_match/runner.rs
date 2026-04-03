// ── New matching types ──────────────────────────────────────────────────────

use std::{
    collections::HashSet,
    str::FromStr,
    sync::{Arc, Mutex},
};

use behavior_bark::unpowered::UnpoweredFunction;
use topper_core::timeline::{BaseAgentState, BaseTimeline, db::DummyDatabaseModule};

use crate::{
    agent::{AgentState, BType, CType, FType},
    bt::{BehaviorController, BehaviorModel, DEBUG_TREES, get_tree},
    bt_match::{BtMatchConfig, Divergence, format_time},
    classes::{AFFLICT_VENOMS, get_controller},
    observables::ActionPlan,
    timeline::{AetObservation, AetTimeSlice, AetTimeline, CombatAction, simulation_slice},
};

/// What a single BT state-branch predicted.
#[derive(Debug, Clone)]
pub struct BranchPlan {
    pub skills: Vec<String>,
    /// Resolved venom names (empty when no venoms were checked for this run).
    pub venoms: Vec<String>,
    /// The resolved QEB input string from the controller.
    pub qeb_inputs: String,
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
    verbose: bool,
    timeline: AetTimeline,
    tree_arc: TreeArc,
    pub match_count: usize,
    skips: Vec<i32>,
}

impl MatchRunner {
    pub fn new(
        player_name: String,
        opponent_name: String,
        tree_name: String,
        config: BtMatchConfig,
        verbose: bool,
        skips: Vec<i32>,
    ) -> Self {
        let tree_arc = get_tree(&tree_name);
        Self {
            player_name,
            opponent_name,
            tree_name,
            config,
            verbose,
            timeline: AetTimeline::new(),
            tree_arc,
            match_count: 0,
            skips,
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
                        && !self
                            .config
                            .ignore
                            .iter()
                            .any(|s| s.eq_ignore_ascii_case(&ca.skill)) =>
                {
                    Some(ca.skill.clone())
                }
                _ => None,
            })
            .collect();

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

        let timeline_fixes: Vec<(&str, Box<dyn Fn(&mut AgentState)>)> = vec![(
            "balance corrected",
            Box::new(|me: &mut AgentState| {
                me.balances[BType::Balance as usize].reset();
                me.balances[BType::Equil as usize].reset();
            }),
        )];

        let (any_match, branch_plans, fix_label) = self.check_and_fix(
            &observed_skills,
            &observed_venoms,
            n_skippable,
            &timeline_fixes,
            time_slice.time,
        );

        if any_match && !observed_skills.is_empty() {
            let skill_str = observed_skills.join(", ");
            let suffix = fix_label.map(|l| format!(" ({})", l)).unwrap_or_default();
            if observed_venoms.is_empty() {
                println!(
                    "[{}] MATCH   {} -> {}{}",
                    format_time(time_slice.time),
                    self.player_name,
                    skill_str,
                    suffix
                );
            } else {
                println!(
                    "[{}] MATCH   {} -> {} (venoms: {}){}",
                    format_time(time_slice.time),
                    self.player_name,
                    skill_str,
                    observed_venoms.join(", "),
                    suffix
                );
            }
            self.match_count += observed_skills.len();
        } else if self.skips.contains(&time_slice.time) {
            println!(
                "[{}] SKIPPED {} -> {} (venoms: {})",
                format_time(time_slice.time),
                self.player_name,
                observed_skills.join(", "),
                observed_venoms.join(", ")
            );
        } else if !observed_skills.is_empty() {
            if self.verbose {
                unsafe {
                    DEBUG_TREES = true;
                }
                let _ = self.check_and_fix(
                    &observed_skills,
                    &observed_venoms,
                    n_skippable,
                    &timeline_fixes,
                    time_slice.time,
                );
                unsafe {
                    DEBUG_TREES = false;
                }
            }
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

    /// Create a copy of the timeline with `me` set to the player being analyzed.
    fn player_timeline(&self) -> AetTimeline {
        let mut tl = self.timeline.clone();
        tl.state.me = self.player_name.clone();
        tl
    }

    /// Run the BT against every state branch; return whether any branch matched
    /// all observed skills and venoms.
    fn check_tree(
        &self,
        observed_skills: &[String],
        observed_venoms: &[String],
        n_skippable: usize,
    ) -> (bool, Vec<BranchPlan>) {
        let class_hint = self.tree_name.split('/').next().unwrap_or("");
        let strategy_hint = self.tree_name.split('/').nth(1).unwrap_or("");
        let mut tree_guard = self.tree_arc.lock().unwrap();
        let mut branch_plans: Vec<BranchPlan> = Vec::new();

        let mut controller = get_controller(
            match class_hint {
                "predator" => "predator",
                "monk" => "monk",
                "zealot" => "zealot",
                "infiltrator" => "infiltrator",
                "sentinel" => "sentinel",
                _ => "",
            },
            &self.player_name,
            &self.opponent_name,
            &self.timeline,
            &strategy_hint.to_string(),
            None as Option<&DummyDatabaseModule>,
        );
        match class_hint {
            "predator" => controller.init_predator(),
            "monk" => controller.init_monk(),
            "zealot" => controller.init_zealot(),
            "infiltrator" => controller.init_infiltrator(),
            _ => {}
        }

        let player_tl = self.player_timeline();
        tree_guard.reset(&player_tl);
        tree_guard.resume_with(&player_tl, &mut controller);

        let skills = controller.plan.get_skills();

        // Resolve planned venoms when the slice has venom activity.
        let venoms: Vec<String> = if !observed_venoms.is_empty() || n_skippable > 0 {
            let opp_state = player_tl.state.borrow_agent(&self.opponent_name).clone();
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

        let qeb_inputs = controller.plan.get_inputs(&self.timeline);
        branch_plans.push(BranchPlan {
            skills,
            venoms,
            qeb_inputs,
        });

        if skills_match && venoms_match {
            return (true, branch_plans);
        }

        (false, branch_plans)
    }

    /// Try check_branches with original state, then retry with each timeline
    /// fix applied until one matches or all are exhausted. After each attempt,
    /// fire any skill traps from the planned skills and retry if traps fired.
    fn check_and_fix<'a>(
        &mut self,
        observed_skills: &[String],
        observed_venoms: &[String],
        n_skippable: usize,
        timeline_fixes: &'a [(&'a str, Box<dyn Fn(&mut AgentState)>)],
        time: CType,
    ) -> (bool, Vec<BranchPlan>, Option<&'a str>) {
        if observed_skills.is_empty() {
            return (true, vec![], None);
        }
        let mut plans: Vec<BranchPlan> = Vec::new();
        // First try without any fixes.
        let (matched, base_plans) = self.check_tree(observed_skills, observed_venoms, n_skippable);
        plans.extend(base_plans.clone());
        let traps_fired = self.fire_skill_traps(&plans, time);
        if matched {
            return (true, plans, None);
        }
        if traps_fired {
            println!("Traps fired");
            let (matched, trap_plans, _) = self.check_and_fix(
                observed_skills,
                observed_venoms,
                n_skippable,
                timeline_fixes,
                time,
            );
            plans.extend(trap_plans.clone());
            if matched {
                return (true, plans, None);
            }
        }

        // Try each fix in order.
        for (label, fix) in timeline_fixes {
            let original_timeline = self.timeline.clone();
            let mut fixed_timeline = self.timeline.branch();
            fixed_timeline.state.for_all_agents(fix);
            self.timeline = fixed_timeline;
            let (matched, fix_plans) =
                self.check_tree(observed_skills, observed_venoms, n_skippable);
            plans.extend(fix_plans.clone());
            self.timeline = original_timeline;
            let traps_fired = self.fire_skill_traps(&plans, time);
            if matched {
                return (true, fix_plans, Some(label));
            }
            if traps_fired {
                println!("Traps fired on fixed timeline");
                let (matched, trap_fix_plans, _) = self.check_and_fix(
                    observed_skills,
                    observed_venoms,
                    n_skippable,
                    timeline_fixes,
                    time,
                );
                plans.extend(trap_fix_plans.clone());
                if matched {
                    return (true, trap_fix_plans, Some(label));
                }
            }
        }

        // Nothing matched — return the original (unfixed) plans.
        (false, plans, None)
    }

    /// Inject skill trap observations into the timeline for any planned skills
    /// that have traps configured. Returns true if any traps were fired.
    fn fire_skill_traps(&mut self, plans: &[BranchPlan], time: CType) -> bool {
        let planned_skills: Vec<String> = plans
            .iter()
            .flat_map(|bp| bp.skills.iter())
            .cloned()
            .collect();
        let mut trap_obs: Vec<AetObservation> = Vec::new();
        for skill in &planned_skills {
            if let Some(obs) = self.config.skill_traps.get(skill) {
                trap_obs.extend(obs.iter().map(|o| self.resolve_sentinels(o)));
            }
        }
        if !trap_obs.is_empty() {
            let trap_slice = simulation_slice(trap_obs, time);
            self.timeline
                .push_time_slice(trap_slice, None as Option<&DummyDatabaseModule>)
                .ok();
            true
        } else {
            false
        }
    }

    fn resolve_sentinel_str(&self, s: &str) -> String {
        match s {
            "LOG_ME" => self.player_name.clone(),
            "LOG_YOU" => self.opponent_name.clone(),
            _ => s.to_string(),
        }
    }

    fn resolve_sentinels(&self, obs: &AetObservation) -> AetObservation {
        match obs {
            AetObservation::CombatAction(ca) => {
                AetObservation::CombatAction(self.resolve_combat_action(ca))
            }
            AetObservation::Proc(ca) => AetObservation::Proc(self.resolve_combat_action(ca)),
            other => other.clone(),
        }
    }

    fn resolve_combat_action(&self, ca: &CombatAction) -> CombatAction {
        CombatAction {
            caster: self.resolve_sentinel_str(&ca.caster),
            target: self.resolve_sentinel_str(&ca.target),
            category: ca.category.clone(),
            skill: ca.skill.clone(),
            annotation: ca.annotation.clone(),
        }
    }
}
