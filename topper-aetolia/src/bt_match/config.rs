use std::collections::HashMap;

use crate::timeline::AetObservation;

/// Config loaded from `bt_match.json` (or a custom path).
#[derive(serde::Deserialize, Default)]
pub struct BtMatchConfig {
    /// Skills to skip when comparing against the BT.
    #[serde(default)]
    pub ignore: Vec<String>,
    /// Skill traps: when the BT produces a skill matching a key, inject the
    /// associated observations as a synthetic time slice.
    #[serde(default)]
    pub skill_traps: HashMap<String, Vec<AetObservation>>,
}
