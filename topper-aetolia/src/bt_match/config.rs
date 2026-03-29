/// Config loaded from `bt_match.json` (or a custom path).
#[derive(serde::Deserialize, Default)]
pub struct BtMatchConfig {
    /// Skills to skip when comparing against the BT.
    #[serde(default)]
    pub ignore: Vec<String>,
}
