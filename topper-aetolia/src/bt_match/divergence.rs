use std::fmt;

use crate::{
    agent::{AgentState, BType},
    bt_match::{BranchPlan, format_time},
};

fn write_agent_state(f: &mut fmt::Formatter<'_>, name: &str, state: &AgentState) -> fmt::Result {
    let affs: Vec<String> = state.flags.aff_iter().map(|fl| format!("{}", fl)).collect();
    let affs_str = if affs.is_empty() {
        "none".to_string()
    } else {
        affs.join(", ")
    };
    writeln!(
        f,
        "  {} | bal={} eq={} pill={} salve={} smoke={} | affs=[{}]",
        name,
        state.balanced(BType::Balance),
        state.balanced(BType::Equil),
        state.balanced(BType::Pill),
        state.balanced(BType::Salve),
        state.balanced(BType::Smoke),
        affs_str,
    )
}

/// A structured description of the first divergence found in a log.
pub struct Divergence {
    pub time: i32,
    pub player_name: String,
    pub opponent_name: String,
    /// All non-ignored CombatAction skills from the diverging slice.
    pub observed_skills: Vec<String>,
    /// Venoms actually delivered (`Devenoms` + `Afflicted`→`AFFLICT_VENOMS`).
    pub observed_venoms: Vec<String>,
    /// One entry per branch evaluated, in order.
    pub branch_plans: Vec<BranchPlan>,
    pub matches_before: usize,
    pub player_state: AgentState,
    pub opponent_state: AgentState,
}

impl fmt::Display for Divergence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header
        writeln!(f, "\n[{}] DIVERGENCE", format_time(self.time))?;

        // Observed line
        let observed_skill_str = self.observed_skills.join(", ");
        if self.observed_venoms.is_empty() {
            writeln!(
                f,
                "  Observed:    {} -> {}",
                self.player_name, observed_skill_str
            )?;
        } else {
            writeln!(
                f,
                "  Observed:    {} -> {} (venoms: {})",
                self.player_name,
                observed_skill_str,
                self.observed_venoms.join(", ")
            )?;
        }

        // Per-branch plans
        for (i, branch) in self.branch_plans.iter().enumerate() {
            writeln!(f, "  branch {} plan: {}", i + 1, branch.qeb_inputs);
            if branch.skills.is_empty() {
                writeln!(f, "  branch {}: (no action)", i + 1)?;
                continue;
            }

            let missing_skills: Vec<&String> = self
                .observed_skills
                .iter()
                .filter(|s| !branch.skills.iter().any(|p| p.eq_ignore_ascii_case(s)))
                .collect();

            let mut line = format!("  branch {}: skills=[{}]", i + 1, branch.skills.join(", "));
            if !missing_skills.is_empty() {
                line.push_str(&format!(
                    " — missing: [{}]",
                    missing_skills
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }

            if !branch.venoms.is_empty() || !self.observed_venoms.is_empty() {
                let missing_venoms: Vec<&String> = self
                    .observed_venoms
                    .iter()
                    .filter(|v| !branch.venoms.iter().any(|p| p.eq_ignore_ascii_case(v)))
                    .collect();
                line.push_str(&format!("; venoms=[{}]", branch.venoms.join(", ")));
                if !missing_venoms.is_empty() {
                    line.push_str(&format!(
                        " — missing: [{}]",
                        missing_venoms
                            .iter()
                            .map(|v| v.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }

            writeln!(f, "{}", line)?;
        }

        writeln!(f)?;

        // Agent states
        write_agent_state(f, &self.player_name, &self.player_state)?;
        write_agent_state(f, &self.opponent_name, &self.opponent_state)?;

        writeln!(
            f,
            "\n{} actions matched before first divergence.",
            self.matches_before
        )
    }
}
