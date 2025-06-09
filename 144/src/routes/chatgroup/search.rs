use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post}};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGroupSearchRequest {
    pub query: Option<String>,
    pub zone_id: Option<String>,
    pub user_id: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chatgroup" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ChatGroupSearchRequest) -> impl Reply {
    match search_chatgroups(request).await {
        Ok(results) => warp::reply::json(&results),
        Err(e) => {
            log::error!("Chat group search error: {:#?}", e);
            warp::reply::json(&json!({
                "error": e.to_string()
            }))
        }
    }
}

async fn search_chatgroups(request: ChatGroupSearchRequest) -> AppResult<serde_json::Value> {
    let limit = request.limit.unwrap_or(20).min(100);
    let offset = request.offset.unwrap_or(0);
    
    // Build filter for tenant_id = "cg" (chat groups)
    let mut filter = json!({
        "must": [
            {
                "key": "s",
                "match": {
                    "value": "cg"
                }
            }
        ]
    });
    
    // Add zone filter if provided
    if let Some(zone_id) = &request.zone_id {
        filter["must"].as_array_mut().unwrap().push(json!({
            "key": "z",
            "match": {
                "value": zone_id
            }
        }));
    }
    
    // Add user filter if provided
    if let Some(user_id) = &request.user_id {
        filter["must"].as_array_mut().unwrap().push(json!({
            "key": "u",
            "match": {
                "value": user_id
            }
        }));
    }
    
    // Build search request
    let mut search_request = json!({
        "filter": filter,
        "limit": limit,
        "offset": offset,
        "with_payload": true
    });
    
    // Add vector search if query is provided
    if let Some(query) = &request.query {
        let embedding_vec = embedding(query.clone()).await?;
        
        let embedding_floats: Vec<f32> = embedding_vec
            .as_array()
            .ok_or(AppError::new_plain("Embedding is not an array"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
            
        search_request["vector"] = json!(embedding_floats);
    } else {
        // If no query provided, sort by creation time (if available) or use simple discovery
        search_request["with_vectors"] = json!(false);
    }
    
    // Execute search
    let response = qdrant_post(
        &qdrant_path("collections/i/points/search").await?,
        search_request
    ).await?;
    
    Ok(response)
} 