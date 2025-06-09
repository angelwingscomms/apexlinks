use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use warp::reply::Reply;
use std::convert::Infallible;
use reqwest::Client;

use crate::util::{AppError, AppResult, embed};
use crate::util::qdrant::{qdrant_path, qdrant_get, qdrant_post};
use crate::constants;
use super::user_tags::UserTagsMetadata;

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResponse {
    pub matched_user_id: Option<String>,
    pub matched_user_tags: Option<String>,
    pub score: Option<f32>,
}

/// Find a matching user based on semantic similarity
pub async fn find_match(request: MatchRequest) -> Result<impl Reply, Infallible> {
    match process_match_request(request).await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => {
            eprintln!("Error finding match: {}", e);
            Ok(warp::reply::json(&json!({"error": e.to_string(), "status": "error", "code": 500})))
        }
    }
}

async fn process_match_request(request: MatchRequest) -> AppResult<MatchResponse> {
    let collection_name = "voice_chat_users";
    
    // First, get the user's tags from Qdrant
    let user_point = qdrant_get(
        &qdrant_path(&format!("collections/{}/points/{}", collection_name, request.user_id)).await?
    ).await?;
    
    let user_data = user_point.get("result")
        .and_then(|r| r.get("payload"))
        .ok_or_else(|| AppError::new_plain("User not found"))?;
    
    let user_tags = user_data.get("tags")
        .and_then(|t| t.as_str())
        .ok_or_else(|| AppError::new_plain("User tags not found"))?;
    
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
    
    // Now search for similar users
    let search_result = search_similar_users(user_tags, &api_key, request.user_id.clone()).await?;
    
    // If no matches found
    if search_result.is_empty() {
        return Ok(MatchResponse {
            matched_user_id: None,
            matched_user_tags: None,
            score: None,
        });
    }
    
    // Return the best match
    let best_match = &search_result[0];
    
    Ok(MatchResponse {
        matched_user_id: Some(best_match.user_id.clone()),
        matched_user_tags: Some(best_match.tags.clone()),
        score: Some(best_match.score),
    })
}

#[derive(Debug)]
struct MatchResult {
    user_id: String,
    tags: String,
    score: f32,
}

async fn search_similar_users(user_tags: &str, api_key: &str, current_user_id: String) -> AppResult<Vec<MatchResult>> {
    // Generate embedding for user's tags
    let embedding = embed(user_tags.to_string()).await?;
    
    // Search in Qdrant
    let collection_name = "voice_chat_users";
    let search_body = json!({
        "vector": embedding,
        "limit": 5,
        "with_payload": true,
        "filter": {
            "must_not": [
                {
                    "key": "user_id",
                    "match": {
                        "value": current_user_id
                    }
                }
            ]
        }
    });
    
    let search_result = qdrant_post(
        &format!("{}collections/{}/points/search", qdrant_path("").await?, collection_name),
        search_body
    ).await?;
    
    // Parse results
    let mut matches = Vec::new();
    
    if let Some(results) = search_result.get("result").and_then(|r| r.as_array()) {
        for result in results {
            let score = result.get("score").and_then(|s| s.as_f64()).map(|s| s as f32);
            let payload = result.get("payload");
            
            if let (Some(score), Some(payload)) = (score, payload) {
                let user_id = payload.get("user_id").and_then(|id| id.as_str());
                let tags = payload.get("tags").and_then(|t| t.as_str());
                
                if let (Some(user_id), Some(tags)) = (user_id, tags) {
                    matches.push(MatchResult {
                        user_id: user_id.to_string(),
                        tags: tags.to_string(),
                        score,
                    });
                }
            }
        }
    }
    
    Ok(matches)
} 