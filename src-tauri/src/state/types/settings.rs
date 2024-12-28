use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    #[serde(default)] // just use default for bool which is false
    pub open_at_startup: bool,
    #[serde(default)]
    pub is_compact_mode: bool,
    #[serde(default)]
    pub fetch_interval: i32,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            open_at_startup: false,
            is_compact_mode: false,
            fetch_interval: 30,
        }
    }
}
