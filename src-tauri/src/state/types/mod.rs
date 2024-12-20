use serde_json::json;
use tauri::Emitter;
use tauri_plugin_store::StoreExt;

mod app_state;
mod auth;
mod github;
mod settings;

pub use app_state::{AppState, ManagedState, StateChangePayload, StateField};
pub use auth::AuthState;
pub use github::{Author, Review, UserId};

impl AppState {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "auth": self.auth,
            "settings": self.settings,
            "github": self.github,
            "issueCount": self.issue_count,
            "reviews": self.reviews,
            "availableOrgs": self.available_orgs,
            "theme": self.theme,
        })
    }

    pub fn from_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

// Add these helper functions
impl ManagedState {
    pub fn update<F>(
        &self,
        app: &tauri::AppHandle,
        updater: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut AppState),
    {
        let mut state = self.state.lock().unwrap();
        let old_state = state.clone(); // Clone the state before update

        updater(&mut state);

        // Determine which fields changed
        let changed_fields = self.detect_changes(&old_state, &state);

        // Create the payload
        let payload = StateChangePayload {
            new_state: state.clone(),
            changed_fields,
        };

        app.emit("state-changed", payload)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Save to store
        let json_value = json!(state.to_json());
        let store = app.store("store.json")?;
        store.set("app_state".to_string(), json_value);
        store.save()?;

        #[cfg(target_os = "macos")]
        {
            let tray = app.tray_by_id("gitbar-tray").unwrap();
            let mut rev_count = state.issue_count.to_string();
            if state.issue_count > 0 {
                rev_count.insert_str(0, " ");
            }
            tray.set_title(Some(rev_count)).unwrap();
        }

        Ok(())
    }

    // Helper method to detect which fields changed
    fn detect_changes(&self, old_state: &AppState, new_state: &AppState) -> Vec<StateField> {
        let mut changed_fields = Vec::new();

        if old_state.auth.as_ref() != new_state.auth.as_ref() {
            changed_fields.push(StateField::Auth);
        }
        if old_state.settings.as_ref() != new_state.settings.as_ref() {
            changed_fields.push(StateField::Settings);
        }
        if old_state.github.as_ref() != new_state.github.as_ref() {
            changed_fields.push(StateField::Github);
        }
        if old_state.issue_count != new_state.issue_count {
            changed_fields.push(StateField::IssueCount);
        }
        if old_state.reviews != new_state.reviews {
            changed_fields.push(StateField::Reviews);
        }
        if old_state.available_orgs != new_state.available_orgs {
            changed_fields.push(StateField::AvailableOrgs);
        }
        if old_state.theme != new_state.theme {
            changed_fields.push(StateField::Theme);
        }

        changed_fields
    }

    pub fn get(&self) -> AppState {
        self.state.lock().unwrap().clone()
    }
}
