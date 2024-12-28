use log::{error, info};
use std::sync::Mutex;
use tauri::{App, Manager};
use tauri_plugin_store::StoreExt;

pub mod commands;
pub mod github;
pub mod types;

pub use types::{AppState, ManagedState};

pub fn init_store(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing store...");

    // Create or get the store
    let store = app.store("store.json")?;
    info!("Store accessed successfully");

    // Initialize the state
    let state = match store.get("app_state") {
        Some(stored_value) => {
            info!("Found existing state, attempting to parse");
            match AppState::from_json(stored_value) {
                Ok(state) => state,
                Err(e) => {
                    error!(
                        "Failed to parse stored state, falling back to default: {}",
                        e
                    );
                    let default_state = AppState::default();
                    store.set("app_state".to_string(), default_state.to_json());
                    store.save()?;
                    default_state
                }
            }
        }
        None => {
            info!("No existing state found, creating default");
            let default_state = AppState::default();
            store.set("app_state".to_string(), default_state.to_json());
            store.save()?;
            default_state
        }
    };

    info!("Creating managed state");
    app.manage(ManagedState {
        data: Mutex::new(state),
        client: Mutex::new(None),
    });
    info!("State managed successfully");

    Ok(())
}
