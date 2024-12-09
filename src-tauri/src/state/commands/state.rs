use crate::state::types::{AppState, ManagedState};

#[tauri::command]
pub async fn update_state(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
    updated_state: serde_json::Value,
) -> Result<(), String> {
    println!("Update state command received");

    match AppState::from_json(updated_state.clone()) {
        Ok(new_state) => {
            println!("Successfully parsed new state");
            state
                .update(&app_handle, |current_state| {
                    *current_state = new_state;
                    println!("State updated successfully");
                })
                .map_err(|e| {
                    println!("Failed to update state: {:?}", e);
                    e.to_string()
                })
        }
        Err(e) => {
            println!("Failed to parse state: {:?}", e);
            println!("Received state structure: {:#?}", updated_state);
            Err(format!("Failed to parse state: {}", e))
        }
    }
}

#[tauri::command]
pub async fn get_state(
    state: tauri::State<'_, ManagedState>,
    window: tauri::Window,
) -> Result<AppState, String> {
    // Return the current state immediately
    Ok(state.get())
}

#[tauri::command]
pub async fn update_partial_state(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
    field: String,
    value: serde_json::Value,
) -> Result<(), String> {
    state
        .update(&app_handle, |current_state| match field.as_str() {
            "auth" => {
                if let Ok(auth) = serde_json::from_value(value.clone()) {
                    current_state.auth = auth;
                }
            }
            "settings" => {
                if let Ok(settings) = serde_json::from_value(value.clone()) {
                    current_state.settings = settings;
                }
            }
            "github" => {
                if let Ok(github) = serde_json::from_value(value.clone()) {
                    current_state.github = github;
                }
            }
            "issueCount" => {
                if let Ok(count) = serde_json::from_value(value.clone()) {
                    current_state.issue_count = count;
                }
            }
            "reviews" => {
                if let Ok(reviews) = serde_json::from_value(value.clone()) {
                    current_state.reviews = reviews;
                }
            }
            "availableOrgs" => {
                if let Ok(orgs) = serde_json::from_value(value.clone()) {
                    current_state.available_orgs = orgs;
                }
            }
            "theme" => {
                if let Ok(theme) = serde_json::from_value(value.clone()) {
                    current_state.theme = theme;
                }
            }
            _ => println!("Unknown field: {}", field),
        })
        .map_err(|e| e.to_string())
}
