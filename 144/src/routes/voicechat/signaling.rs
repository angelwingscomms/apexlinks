use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::reply::Reply;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::util::{AppError, AppResult};

// In-memory storage for pending signals
static SIGNAL_STORE: Lazy<Arc<Mutex<HashMap<String, Vec<SignalData>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalRequest {
    pub from_user_id: String,
    pub to_user_id: String,
    pub signal_type: String,  // "offer", "answer", "ice-candidate"
    pub signal_data: String,  // JSON stringified WebRTC data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalData {
    pub from_user_id: String,
    pub signal_type: String,
    pub signal_data: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalResponse {
    pub success: bool,
    pub pending_signals: Vec<SignalData>,
}

/// Relay WebRTC signaling data between users
pub async fn relay_signal(request: SignalRequest) -> Result<impl Reply, Infallible> {
    match process_signal(request).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => {
            eprintln!("Error relaying signal: {}", e);
            Ok(warp::reply::json(&json!({"error": e.to_string(), "status": "error", "code": 500})))
        }
    }
}

async fn process_signal(request: SignalRequest) -> AppResult<SignalResponse> {
    // Create signal data with timestamp
    let signal_data = SignalData {
        from_user_id: request.from_user_id.clone(),
        signal_type: request.signal_type.clone(),
        signal_data: request.signal_data.clone(),
        timestamp: chrono::Utc::now(),
    };
    
    // Store the signal for the recipient
    {
        let mut store = SIGNAL_STORE.lock().map_err(|_| AppError::new_plain("Failed to lock signal store"))?;
        let signals = store.entry(request.to_user_id.clone()).or_insert_with(Vec::new);
        signals.push(signal_data);
        
        // Cap the number of stored signals per user to prevent DOS
        const MAX_SIGNALS_PER_USER: usize = 50;
        if signals.len() > MAX_SIGNALS_PER_USER {
            signals.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));  // Sort by timestamp, newest first
            signals.truncate(MAX_SIGNALS_PER_USER);
        }
    }
    
    // Retrieve pending signals for the sender
    let pending_signals = {
        let mut store = SIGNAL_STORE.lock().map_err(|_| AppError::new_plain("Failed to lock signal store"))?;
        store.remove(&request.from_user_id).unwrap_or_default()
    };
    
    Ok(SignalResponse {
        success: true,
        pending_signals,
    })
}

// Clean up old signals (could be called periodically)
pub async fn cleanup_old_signals() -> AppResult<()> {
    let cutoff_time = chrono::Utc::now() - chrono::Duration::minutes(15);
    
    let mut store = SIGNAL_STORE.lock().map_err(|_| AppError::new_plain("Failed to lock signal store"))?;
    
    for signals in store.values_mut() {
        signals.retain(|signal| signal.timestamp > cutoff_time);
    }
    
    // Remove empty entries
    store.retain(|_, signals| !signals.is_empty());
    
    Ok(())
} 