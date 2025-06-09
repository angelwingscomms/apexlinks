use serde::{Deserialize, Serialize};
use warp::{Filter, Reply, Rejection};
use serde_json::json;

use crate::util::{AppError, AppResult, embedding};
use crate::util::qdrant::{qdrant_path, qdrant_post};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub s: String, // tenant/sort identifier
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub users: Vec<UserSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSearchResult {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub username: Option<String>,
    pub zone_id: Option<String>,
    pub score: Option<f32>,
}

pub fn route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("user" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(r)
}

pub async fn r(request: SearchRequest) -> Result<impl Reply, Rejection> {
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

async fn f(request: SearchRequest) -> AppResult<SearchResponse> {
    let limit = request.limit.unwrap_or(10);
    
    // Log the search request
    log::info!("Starting search for '{}' with tenant '{}'", 
               &request.query, &request.s);
    
    // Create embedding for the search query
    log::debug!("Generating embedding for query: {}", &request.query);
    let search_embedding = match embedding(request.query).await {
        Ok(embedding) => embedding,
        Err(e) => {
            log::error!("Failed to generate embedding: {}", e);
            return Err(e);
        }
    };
    
    let search_embedding_floats: Vec<f32> = match search_embedding
        .as_array()
        .ok_or(AppError::new_plain("Search embedding is not an array")) {
            Ok(array) => {
                array.iter()
                    .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                    .collect()
            },
            Err(e) => {
                log::error!("Failed to process embedding: {}", e);
                return Err(e);
            }
        };

    log::debug!("Performing Qdrant search with {} dimensional vector", search_embedding_floats.len());
    
    // Perform vector search in Qdrant with proper filtering
    let search_body = json!({
        "vector": search_embedding_floats,
        "limit": limit,
        "with_payload": true,
        "filter": {
            "must": [
                {
                    "key": "s",
                    "match": {
                        "value": request.s
                    }
                }
            ]
        }
    });

    log::debug!("Sending search request to Qdrant with tenant filter s='{}' in query", request.s);
    let search_result = match qdrant_post(
        &qdrant_path("collections/i/points/search").await?,
        search_body
    ).await {
        Ok(result) => result,
        Err(e) => {
            log::error!("Qdrant search failed: {}", e);
            return Err(e);
        }
    };

    log::debug!("Processing Qdrant search results");
    let points = match search_result["result"].as_array() {
        Some(points) => points,
        None => {
            let error = AppError::new_plain("Failed to extract points from response");
            log::error!("{}", error);
            return Err(error);
        }
    };

    log::debug!("Found {} results with tenant filter applied", points.len());

    // Convert results to UserSearchResult structs
    let users: Vec<UserSearchResult> = points
        .iter()
        .filter_map(|point| {
            let payload = &point["payload"];
            
            // Debug logging for each result
            if let Some(id) = point["id"].as_str() {
                log::debug!("Processing user result: {}", id);
            }
            
            Some(UserSearchResult {
                id: point["id"].as_str()?.to_string(),
                name: payload["n"].as_str().map(|s| s.to_string()),
                email: payload["email"].as_str().map(|s| s.to_string()),
                description: payload["t"].as_str().map(|s| s.to_string()),
                username: payload["u"].as_str().map(|s| s.to_string()),
                zone_id: payload["z"].as_str().map(|s| s.to_string()),
                score: point["score"].as_f64().map(|f| f as f32),
            })
        })
        .collect();

    log::info!("Search completed. Tenant: '{}', Results: {}", 
               request.s, users.len());

    Ok(SearchResponse { users })
} 