use serde::{Deserialize, Serialize};
use warp::{Filter, Reply, Rejection};
use serde_json::json;

use crate::util::{AppError, AppResult, embed};
use crate::util::qdrant::{qdrant_path, qdrant_post};
use crate::constants::SECRETS;

#[derive(Debug, Deserialize)]
pub struct SimilarityRequest {
    pub user_id: Option<String>, // Find similar to this user
    pub text: Option<String>,    // Or find similar to this text
    pub s: String,               // tenant/sort identifier
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SimilarityResponse {
    pub similar_users: Vec<SimilarUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub tags: Option<String>,
    pub similarity_score: f32,
}

pub fn route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("user" / "similarity")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(r)
}

pub async fn r(request: SimilarityRequest) -> Result<impl Reply, Rejection> {
    f(request).await.map_or_else(
        |e| {
            log::error!("{:#?}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&"An error occured on our side".to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        },
        |v| Ok(warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::OK)),
    )
}

async fn f(request: SimilarityRequest) -> AppResult<SimilarityResponse> {
    let api_key = SECRETS
        .lock()
        .await
        .get("QDRANT_KEY")
        .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?;

    let limit = request.limit.unwrap_or(10);
    
    // Get the embedding vector
    let embedding = if let Some(user_id) = request.user_id {
        // Get user's embedding from Qdrant
        let get_url = qdrant_path(&format!("collections/users/points/{}", user_id)).await?;
        
        let response = reqwest::Client::new()
            .get(&get_url)
            .header("api-key", &api_key)
            .send()
            .await
            .map_err(|e| AppError::new("fetching user from qdrant", e))?;

        if !response.status().is_success() {
            return Err(AppError::new_plain("User not found"));
        }

        let user_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::new("parsing user data", e))?;

        user_data["result"]["vector"]
            .as_array()
            .ok_or_else(|| AppError::new_plain("User has no embedding"))?
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect::<Vec<f32>>()
    } else if let Some(text) = request.text {
        // Generate embedding from text
        embed(text).await?
    } else {
        return Err(AppError::new_plain("Either user_id or text must be provided"));
    };

    // Search for similar users
    let search_url = qdrant_path("collections/users/points/search").await?;
    
    let search_body = json!({
        "vector": embedding,
        "filter": {
            "must": [
                {
                    "key": "s",
                    "match": {
                        "value": request.s
                    }
                }
            ]
        },
        "limit": limit,
        "with_payload": true
    });

    let response = reqwest::Client::new()
        .post(&search_url)
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&search_body)
        .send()
        .await
        .map_err(|e| AppError::new("searching similar users in qdrant", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::new_plain(&format!(
            "Qdrant search error: {}",
            error_text
        )));
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::new("parsing qdrant search response", e))?;

    let results = response_json["result"]
        .as_array()
        .ok_or_else(|| AppError::new_plain("Failed to extract results from response"))?;

    let similar_users: Vec<SimilarUser> = results
        .iter()
        .filter_map(|result| {
            let payload = &result["payload"];
            Some(SimilarUser {
                id: result["id"].as_str()?.to_string(),
                name: payload["name"].as_str().map(|s| s.to_string()),
                email: payload["email"].as_str().map(|s| s.to_string()),
                tags: payload["tags"].as_str().map(|s| s.to_string()),
                similarity_score: result["score"].as_f64()? as f32,
            })
        })
        .collect();

    Ok(SimilarityResponse { similar_users })
} 