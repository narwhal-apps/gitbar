use crate::state::github::client::GitHubClient;
use crate::state::types::{AppState, ManagedState};
use log::info;

#[tauri::command]
pub async fn fetch_github_reviews(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
) -> Result<(), String> {
    info!("Ferching with graphql");
    // Extract the needed values and clone the client while holding the lock
    let (username, github_client) = {
        let state_guard = state.data.lock().unwrap();
        let mut client_guard = state.github_client.lock().unwrap();

        // Get auth info from state
        let auth = state_guard.auth.as_ref().ok_or("Not authenticated")?;
        let user = auth.user.as_ref().ok_or("No user found")?;
        let token = auth.token.as_ref().ok_or("No token found")?;

        // If client is not initialized, create a new one
        if client_guard.is_none() {
            *client_guard = Some(
                GitHubClient::create_client(token, auth.hostname.clone())
                    .map_err(|e| e.to_string())?,
            );
        }

        let client = client_guard.as_ref().unwrap();

        // Return the values we need
        (user.login.clone(), client.clone())
    }; // MutexGuards are dropped here

    // Now we can make the async call using the cloned client
    let reviews = github_client
        .get_all_relevant_prs_2(&username)
        .await
        .map_err(|e| e.to_string())?;

    info!("Fetched {} reviews", reviews.len());
    info!("Fetched {:?} reviews", reviews);

    state
        .update(&app_handle, |current_state| {
            current_state.issue_count = reviews.len() as i32;
            current_state.reviews = reviews;
        })
        .map_err(|e| e.to_string());

    Ok(())
}

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
    token: String,
    hostname: Option<String>,
) -> Result<AppState, String> {
    let client =
        GitHubClient::create_client(&token, hostname.clone()).map_err(|e| e.to_string())?;

    let user = client.get_user_info().await.map_err(|e| e.to_string())?;

    let reviews = client
        .get_all_relevant_prs_2(&user.login)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut github_client = state.github_client.lock().unwrap();
        *github_client = Some(client);

        state
            .update(&app_handle, |current_state| {
                current_state.issue_count = reviews.len() as i32;
                current_state.reviews = reviews;
                current_state.auth = Some(crate::state::types::AuthState {
                    token: Some(token),
                    user: Some(user),
                    hostname,
                });
            })
            .map_err(|e| e.to_string())?;
    }

    Ok(state.get())
}

#[tauri::command]
pub async fn logout(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
) -> Result<(), String> {
    // Clear GitHub client
    {
        let mut github_client = state.github_client.lock().unwrap();
        *github_client = None;
    }

    // Reset app state to initial values
    state
        .update(&app_handle, |current_state| {
            *current_state = AppState::default();
            // Add any other state fields that need to be reset
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}
