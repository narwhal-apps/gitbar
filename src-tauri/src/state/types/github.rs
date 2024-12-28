use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct GithubSettings {
    #[serde(default)]
    pub archive: bool,
    #[serde(default)]
    #[serde(rename = "type")]
    pub request_type: String, // Change from enum to String temporarily
    #[serde(default)]
    pub state: String, // Change from enum to String temporarily
}

impl Default for GithubSettings {
    fn default() -> Self {
        Self {
            archive: false,
            request_type: "review-requested".to_string(),
            state: "open".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct Organization {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct UserId(pub u64);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct Labels(pub Vec<Label>);

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct StatusCheckRollup {
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub title: String,
    pub url: String,
    pub number: i64,
    pub repository: String,
    pub author: Author,
    pub created_at: String,
    pub closed: bool,
    pub is_draft: bool,
    pub review_decision: String,
    pub total_comments_count: i64,
    pub is_read_by_viewer: bool,
    pub labels: Labels,
    pub status_check_rollup: StatusCheckRollup,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub login: String,
    pub avatar_url: String,
    pub url: String,
}
