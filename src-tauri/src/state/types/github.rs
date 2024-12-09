use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct GithubSettings {
    pub archive: bool,
    #[serde(rename = "type")]
    pub request_type: String, // Change from enum to String temporarily
    pub state: String, // Change from enum to String temporarily
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
pub struct Review {
    pub repository: String,
    pub author: String,
    pub author_object: Author,
    // pub created_at: DateTime<Utc>,
    pub created_at: String,
    pub number: u64, // Change from String to i32
    pub url: String,
    pub title: String,
    // pub merged: bool,
    pub closed: bool,
    pub is_draft: bool,
    pub review_decision: String, // Change from enum to String temporarily
    pub total_comments_count: u32,
    // pub labels: Labels, // Change from Vec<Label> to Labels struct
    // pub status_check_rollup: StatusCheckRollup,
    // pub is_read_by_viewer: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct Author {
    pub login: String,
    pub id: UserId,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub patch_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
