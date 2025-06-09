use super::types::{MatchRequest, MatchResponse, UserProfile, ChatSession};
use crate::util::{AppResult, AppError, embed};
use crate::util::qdrant::{qdrant_path, qdrant_put, qdrant_post};
use crate::constants::SECRETS;
use serde_json::{json, Value};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

// Global state for managing waiting users and active sessions
lazy_static::lazy_static! {
    static ref WAITING_USERS: Arc<Mutex<Vec<UserProfile>>> = Arc::new(Mutex::new(Vec::new()));
    pub static ref ACTIVE_SESSIONS: Arc<Mutex<HashMap<String, ChatSession>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn find_match(request: MatchRequest) -> Result<impl warp::Reply, Infallible> {
    match find_match_internal(request).await {
        Ok(response) => Ok(warp::reply::with_status(
            warp::reply::json(&response),
            warp::http::StatusCode::OK,
        )),
        Err(e) => {
            eprintln!("Error in find_match: {}", e);
            let error_response = MatchResponse {
                match_found: false,
                session_id: None,
                partner_id: None,
                message: "Error finding match".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

async fn find_match_internal(request: MatchRequest) -> AppResult<MatchResponse> {
    // Create user profile
    let mut user = UserProfile::new(
        request.description.clone(),
        request.interests.clone(),
        request.age_range.clone(),
    );

    // Get Gemini API key
    let secrets = SECRETS.lock().await;
    let api_key = secrets
        .get("GEMINI_API_KEY")
        .ok_or_else(|| AppError::new_plain("GEMINI_API_KEY not found in secrets"))?;

    // Create embedding for user description
    let embedding_text = format!("{} {}", request.description, request.interests.join(" "));
    let embedding = embed(embedding_text).await?;
    user.embedding = Some(embedding.clone());

    // Store user in Qdrant for future matching
    store_user_in_qdrant(&user).await?;

    // Try to find a match from waiting users first
    let mut waiting_users = WAITING_USERS.lock().await;
    
    // Look for compatible user in waiting queue
    for (index, waiting_user) in waiting_users.iter().enumerate() {
        if let Some(waiting_embedding) = &waiting_user.embedding {
            let similarity = calculate_cosine_similarity(&embedding, waiting_embedding);
            
            // If similarity is above threshold (e.g., 0.7), create a match
            if similarity > 0.7 {
                let matched_user = waiting_users.remove(index);
                drop(waiting_users); // Release the lock
                
                // Create chat session
                let session = ChatSession::new(user.id.clone(), matched_user.id.clone());
                
                // Store session
                let mut sessions = ACTIVE_SESSIONS.lock().await;
                sessions.insert(session.id.clone(), session.clone());
                
                return Ok(MatchResponse {
                    match_found: true,
                    session_id: Some(session.id),
                    partner_id: Some(matched_user.id),
                    message: "Match found! You can start chatting.".to_string(),
                });
            }
        }
    }
    
    // No immediate match found, try Qdrant search
    if let Ok(qdrant_match) = search_compatible_users(&user).await {
        if !qdrant_match.is_empty() {
            // For now, just add to waiting queue
            // In a production system, you'd want to notify the matched user
            waiting_users.push(user);
            
            return Ok(MatchResponse {
                match_found: false,
                session_id: None,
                partner_id: None,
                message: "Looking for matches... Please wait.".to_string(),
            });
        }
    }
    
    // No match found, add to waiting queue
    waiting_users.push(user);
    
    Ok(MatchResponse {
        match_found: false,
        session_id: None,
        partner_id: None,
        message: "No immediate match found. You've been added to the waiting queue.".to_string(),
    })
}

async fn store_user_in_qdrant(user: &UserProfile) -> AppResult<()> {
    let collection_name = "chat_users";
    
    // Create collection if it doesn't exist
    let create_result = qdrant_put(
        &qdrant_path(&format!("collections/{}?wait=true", collection_name)).await?,
        json!({
            "vectors": {
                "size": 768,  // Gemini embedding dimension
                "distance": "Cosine"
            }
        }),
    ).await;
    
    // Ignore error if collection already exists
    if let Err(e) = create_result {
        if !e.to_string().contains("already exists") {
            return Err(e);
        }
    }
    
    // Store user point
    if let Some(embedding) = &user.embedding {
        qdrant_put(
            &qdrant_path(&format!("collections/{}/points?wait=true", collection_name)).await?,
            json!({
                "points": [{
                    "id": user.id,
                    "vector": embedding,
                    "payload": {
                        "description": user.description,
                        "interests": user.interests,
                        "age_range": user.age_range,
                        "created_at": user.created_at
                    }
                }]
            }),
        ).await?;
    }
    
    Ok(())
}

async fn search_compatible_users(user: &UserProfile) -> AppResult<Vec<UserProfile>> {
    let collection_name = "chat_users";
    
    if let Some(embedding) = &user.embedding {
        let search_result = qdrant_post(
            &qdrant_path(&format!("collections/{}/points/query", collection_name)).await?,
            json!({
                "query": embedding,
                "limit": 5,
                "with_payload": true,
                "filter": {
                    "must_not": [{
                        "key": "id",
                        "match": { "value": user.id }
                    }]
                }
            }),
        ).await?;
        
        // Parse results (simplified for now)
        // In a real implementation, you'd convert the Qdrant response to UserProfile structs
        return Ok(Vec::new());
    }
    
    Ok(Vec::new())
}

fn calculate_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a * norm_b)
} 