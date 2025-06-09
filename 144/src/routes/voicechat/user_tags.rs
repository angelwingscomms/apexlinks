use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::reply::Reply;
use std::convert::Infallible;

use crate::util::{AppError, AppResult, embed, id};
use crate::util::qdrant::{qdrant_path, qdrant_put};
use crate::constants;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationRequest {
    pub tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistrationResponse {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTagsMetadata {
    pub user_id: String,
    pub tags: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Register a user with their tags
pub async fn register_user(request: UserRegistrationRequest) -> Result<impl Reply, Infallible> {
    match process_registration(request).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => {
            eprintln!("Error registering user: {}", e);
            Ok(warp::reply::json(&json!({"error": e.to_string(), "status": "error", "code": 500})))
        }
    }
}

async fn process_registration(request: UserRegistrationRequest) -> AppResult<UserRegistrationResponse> {
    // Create a unique user ID
    let user_id = Uuid::now_v7().to_string();
    
    // Access the API key
    let api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            let secrets = constants::get_secrets().await
                .map_err(|e| AppError::new("accessing secrets", e))?;
            secrets.get("GEMINI_API_KEY")
                .ok_or_else(|| AppError::new_plain("GEMINI_API_KEY not found in secrets"))?
                .to_string()
        }
    };

    // Ensure the voice_chat_users collection exists
    let collection_name = "voice_chat_users";
    let create_collection_result = qdrant_put(
        &qdrant_path(&format!("collections/{}?wait=true", collection_name)).await?,
        json!({
            "vectors": {
                "size": 768,  // Gemini text-embedding-004 dimension
                "distance": "Cosine"
            }
        }),
    ).await;

    // Ignore error if collection already exists
    if let Err(e) = create_collection_result {
        if !e.to_string().contains("already exists") {
            return Err(e);
        }
    }

    // Create embedding for user's tags
    let embedding = embed(request.tags.clone()).await?;
    
    // Create metadata
    let metadata = UserTagsMetadata {
        user_id: user_id.clone(),
        tags: request.tags.clone(),
        timestamp: chrono::Utc::now(),
    };

    // Store the user data in Qdrant
    qdrant_put(
        &qdrant_path(&format!("collections/{}/points?wait=true", collection_name)).await?,
        json!({
            "points": [{
                "id": user_id,
                "vector": embedding,
                "payload": metadata
            }]
        }),
    ).await?;

    Ok(UserRegistrationResponse { user_id })
} 