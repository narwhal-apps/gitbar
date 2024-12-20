use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    pub open_at_startup: bool,
    pub is_compact_mode: bool,
    pub fetch_interval: i32,
}
