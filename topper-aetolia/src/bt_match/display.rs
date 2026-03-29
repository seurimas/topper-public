use crate::agent::{AgentState, BType};

/// Format centiseconds as `M:SS.cc`.
pub fn format_time(centis: i32) -> String {
    let secs = centis / 100;
    let mins = secs / 60;
    format!("{}:{:02}.{:02}", mins, secs % 60, centis % 100)
}

/// Print balance, equilibrium, and affliction summary for one agent.
pub fn print_agent_state(label: &str, state: &AgentState) {
    let affs: Vec<String> = state.flags.aff_iter().map(|f| format!("{}", f)).collect();
    let bal = state.balanced(BType::Balance);
    let eq = state.balanced(BType::Equil);
    println!(
        "  {} | balance={} equil={} | affs=[{}]",
        label,
        bal,
        eq,
        if affs.is_empty() {
            "none".to_string()
        } else {
            affs.join(", ")
        }
    );
}
