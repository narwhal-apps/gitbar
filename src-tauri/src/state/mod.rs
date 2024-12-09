use std::sync::Mutex;
use tauri::{App, Manager};
use tauri_plugin_store::StoreExt;

pub mod commands;
pub mod github;
pub mod types;

pub use types::{AppState, ManagedState};

pub fn init_store(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing store...");

    // Create or get the store
    let store = app.store("store.json")?;
    println!("Store accessed successfully");

    // Initialize the state
    let state = match store.get("app_state") {
        Some(stored_value) => {
            println!("Found existing state, attempting to parse");
            AppState::from_json(stored_value)?
        }
        None => {
            println!("No existing state found, creating default");
            let default_state = AppState::default();
            store.set("app_state".to_string(), default_state.to_json());
            store.save()?;
            default_state
        }
    };

    println!("Creating managed state");
    app.manage(ManagedState {
        state: Mutex::new(state),
        github_client: Mutex::new(None),
    });
    println!("State managed successfully");

    Ok(())
}
