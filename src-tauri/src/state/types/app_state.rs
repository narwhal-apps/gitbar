use super::{
    auth::AuthState,
    github::{GithubSettings, Organization, Review},
    settings::SettingsState,
};
use crate::state::github::client::GitHubClient;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub auth: Option<AuthState>,
    pub settings: Option<SettingsState>,
    pub github: Option<GithubSettings>,
    pub issue_count: i32,
    pub reviews: Vec<Review>,
    pub available_orgs: Vec<Organization>,
    pub theme: Theme,
}

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub enum StateField {
    Auth,
    Settings,
    Github,
    IssueCount,
    Reviews,
    AvailableOrgs,
    Theme,
}

#[derive(Clone, Serialize, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct StateChangePayload {
    pub new_state: AppState,
    pub changed_fields: Vec<StateField>,
}

// Wrap AppState in a Mutex
pub struct ManagedState {
    pub state: Mutex<AppState>,
    pub github_client: Mutex<Option<GitHubClient>>,
}
