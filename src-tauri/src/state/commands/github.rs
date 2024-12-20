use crate::state::github::client::GitHubClient;
use crate::state::github::client::{get_all_relevant_prs, get_user_info};
use crate::state::types::ManagedState;

#[tauri::command]
pub async fn fetch_github_reviews(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
) -> Result<(), String> {
    // Extract the needed values and clone the client while holding the lock
    let (username, github_client) = {
        let state_guard = state.state.lock().unwrap();
        let client_guard = state.github_client.lock().unwrap();

        // Get auth info from state
        let auth = state_guard.auth.as_ref().ok_or("Not authenticated")?;
        let user = auth.user.as_ref().ok_or("No user found")?;
        let client = client_guard
            .as_ref()
            .ok_or("GitHub client not initialized")?;

        // Return the values we need
        (user.login.clone(), client.clone())
    }; // MutexGuards are dropped here

    // Now we can make the async call using the cloned client
    let reviews = github_client
        .get_all_relevant_prs(&username)
        .await
        .map_err(|e| e.to_string())?;

    state
        .update(&app_handle, |current_state| {
            current_state.issue_count = reviews.len() as i32;
            current_state.reviews = reviews;
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
) -> Result<(), String> {
    // Extract the needed values while holding the lock
    let token = {
        let state_guard = state.state.lock().unwrap();

        // Get auth info from state
        let auth = state_guard.auth.as_ref().ok_or("Not authenticated")?;
        let token = auth.token.as_ref().ok_or("No token found")?;

        // Clone the values we need
        token.clone()
    }; // MutexGuard is dropped here

    // Now we can make the async call safely
    let user = get_user_info(&token).await.map_err(|e| e.to_string())?;

    // You might want to do something with the user info here
    // For example, update the state with the user info
    Ok(())
}

#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, ManagedState>,
    token: String,
    hostname: Option<String>,
) -> Result<(), String> {
    println!("{}", &token);
    println!("{:?}", &hostname);

    // Create new GitHub client
    let client = match &hostname {
        Some(host) => GitHubClient::new_enterprise(&token, host).map_err(|e| e.to_string())?,
        None => GitHubClient::new(&token).map_err(|e| e.to_string())?,
    };

    // Get user info using the client
    let user = client.get_user_info().await.map_err(|e| e.to_string())?;

    // Update both the client and auth state
    {
        // Update GitHub client
        let mut github_client = state.github_client.lock().unwrap();
        *github_client = Some(client);

        // Update auth state
        state
            .update(&app_handle, |current_state| {
                current_state.auth = Some(crate::state::types::AuthState {
                    token: Some(token),
                    user: Some(user),
                    hostname,
                });
            })
            .map_err(|e| e.to_string())?;
    }

    Ok(())
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
            current_state.auth = None;
            current_state.reviews = vec![];
            current_state.issue_count = 0;
            // Add any other state fields that need to be reset
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}
