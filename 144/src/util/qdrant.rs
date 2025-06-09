use serde::Serialize;

use crate::{util::{AppResult, AppError}, constants::SECRETS};

#[cfg(not(test))]
pub async fn qdrant_path(path: &str) -> AppResult<String> {
    Ok(format!(
        "{}/{}",
        SECRETS
            .lock()
            .await
            .get("QDRANT_URL")
            .ok_or("QDRANT_URL not found in env")
            .map_err(|e| AppError::new_plain(e))?,
        path
    ))
}

#[cfg(test)]
pub async fn qdrant_path(path: &str) -> AppResult<String> {
    // For tests, we just return the path as is
    Ok(path.to_string())
}

#[cfg(not(test))]
pub async fn qdrant_put(path: &str, body: impl Serialize) -> AppResult<serde_json::Value> {
    let response = reqwest::Client::new()
        .put(path)
        .header(
            "api-key",
            SECRETS
                .lock()
                .await
                .get("QDRANT_KEY")
                .ok_or("QDRANT_KEY not found in env")
                .map_err(|e| AppError::new_plain(e))?,
        )
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::new("qdrant put request failed", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Qdrant PUT failed with status {}: {}", 
            status, 
            error_text
        )));
    }

    response
        .json()
        .await
        .map_err(|e| AppError::new("failed to parse Qdrant PUT response as JSON", e))
}

#[cfg(test)]
pub async fn qdrant_put(path: &str, body: impl Serialize) -> AppResult<serde_json::Value> {
    // For tests, we track the call and return a mock response
    use std::sync::atomic::{AtomicBool, Ordering};
    use once_cell::sync::Lazy;
    
    static CALLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
    CALLED.store(true, Ordering::SeqCst);
    
    // Log for debugging
    println!("Test qdrant_put called with path: {}", path);
    
    Ok(serde_json::json!({
        "status": "success",
        "time": 0.001,
        "result": {}
    }))
}

#[cfg(not(test))]
pub async fn qdrant_post(path: &str, body: impl Serialize) -> AppResult<serde_json::Value> {
    let response = reqwest::Client::new()
        .post(path)
        .header(
            "api-key",
            SECRETS
                .lock()
                .await
                .get("QDRANT_KEY")
                .ok_or("QDRANT_KEY not found in env")
                .map_err(|e| AppError::new_plain(e))?,
        )
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::new("qdrant post request failed", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Qdrant POST failed with status {}: {}", 
            status, 
            error_text
        )));
    }

    response
        .json()
        .await
        .map_err(|e| AppError::new("failed to parse Qdrant POST response as JSON", e))
}

#[cfg(test)]
pub async fn qdrant_post(path: &str, body: impl Serialize) -> AppResult<serde_json::Value> {
    // For tests, we track the call and return a mock response
    use std::sync::atomic::{AtomicBool, Ordering};
    use once_cell::sync::Lazy;
    
    static CALLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
    CALLED.store(true, Ordering::SeqCst);
    
    // Log for debugging
    println!("Test qdrant_post called with path: {}", path);
    
    // Record the body for assertion in tests
    println!("Request body: {}", serde_json::to_string(&body).unwrap_or_default());
    
    // Check if this is related to user creation
    let body_json = serde_json::to_value(body)
        .unwrap_or(serde_json::json!({}));
        
    if let Some(points) = body_json.get("points").and_then(|p| p.as_array()) {
        if !points.is_empty() {
            if let Some(payload) = points[0].get("payload") {
                if payload.get("google_id").is_some() {
                    // This is user creation - set a specific flag
                    use std::sync::atomic::{AtomicBool, Ordering};
                    use once_cell::sync::Lazy;
                    
                    static USER_CREATED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
                    USER_CREATED.store(true, Ordering::SeqCst);
                    
                    println!("User creation detected in test");
                }
            }
        }
    }
    
    Ok(serde_json::json!({
        "status": "success",
        "time": 0.001,
        "result": {}
    }))
}

#[cfg(not(test))]
pub async fn qdrant_get(path: &str) -> AppResult<serde_json::Value> {
    let response = reqwest::Client::new()
        .get(path)
        .header(
            "api-key",
            SECRETS
                .lock()
                .await
                .get("QDRANT_KEY")
                .ok_or("QDRANT_KEY not found in env")
                .map_err(|e| AppError::new_plain(e))?,
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| AppError::new("qdrant get request failed", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Qdrant GET failed with status {}: {}", 
            status, 
            error_text
        )));
    }

    response
        .json()
        .await
        .map_err(|e| AppError::new("failed to parse Qdrant GET response as JSON", e))
}

#[cfg(test)]
pub async fn qdrant_get(path: &str) -> AppResult<serde_json::Value> {
    // For tests, we track the call and return a mock response
    use std::sync::atomic::{AtomicBool, Ordering};
    use once_cell::sync::Lazy;
    
    static CALLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
    CALLED.store(true, Ordering::SeqCst);
    
    // Log for debugging
    println!("Test qdrant_get called with path: {}", path);
    
    Ok(serde_json::json!({
        "status": "success",
        "time": 0.001,
        "result": {}
    }))
}

// Functions for test assertions
#[cfg(test)]
pub mod test_utils {
    use std::sync::atomic::{AtomicBool, Ordering};
    use once_cell::sync::Lazy;
    
    pub static USER_CREATED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
    
    pub fn reset_test_state() {
        USER_CREATED.store(false, Ordering::SeqCst);
    }
    
    pub fn was_user_created() -> bool {
        USER_CREATED.load(Ordering::SeqCst)
    }
}