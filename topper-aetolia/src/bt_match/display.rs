use crate::agent::{AgentState, BType};

/// Format centiseconds as `M:SS.cc`.
pub fn format_time(centis: i32) -> String {
    let secs = centis / 100;
    let mins = secs / 60;
    format!("{}:{:02}.{:02}", mins, secs % 60, centis % 100)
}

pub fn parse_time(s: &str) -> Option<i32> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let mins = parts[0].parse::<i32>().ok()?;
    let secs_parts: Vec<&str> = parts[1].split('.').collect();
    if secs_parts.len() != 2 {
        return None;
    }
    let secs = secs_parts[0].parse::<i32>().ok()?;
    let centis = secs_parts[1].parse::<i32>().ok()?;
    Some(mins * 60 * 100 + secs * 100 + centis)
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
