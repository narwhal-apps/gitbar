use crate::state::types::Author;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct User {
    pub login: String,
    pub name: Option<String>,
    pub id: i32,
    pub avatar_url: String,
    pub html_url: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct AuthState {
    pub token: Option<String>,
    pub hostname: Option<String>,
    pub user: Option<Author>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, TS)]
#[ts(export, export_to = "index.ts")]
pub struct AuthTokenOptions {
    pub hostname: String,
    pub token: String,
}
