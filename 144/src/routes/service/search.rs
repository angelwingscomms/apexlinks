use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post}},
};

#[derive(Debug, Deserialize)]
pub struct ServiceSearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub zone_id: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct ServiceSearchResponse {
    pub services: Vec<ServiceSearchResult>,
}

#[derive(Debug, Serialize)]
pub struct ServiceSearchResult {
    pub id: String,
    pub description: String,
    pub price: f64,
    pub user_id: String,
    pub zone_id: Option<String>,
    pub images: Vec<String>,
    pub location: String,
    pub position: serde_json::Value,
    pub score: Option<f32>,
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("service" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ServiceSearchRequest) -> impl Reply {
    match search_services(request).await {
        Ok(results) => warp::reply::with_status(
            serde_json::to_string(&results).unwrap_or_else(|_| "{}".to_string()),
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Service search error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn search_services(request: ServiceSearchRequest) -> AppResult<ServiceSearchResponse> {
    let limit = request.limit.unwrap_or(20);
    
    // Create embedding for the search query
    let search_embedding = embedding(request.query).await?;
    
    let search_embedding_floats: Vec<f32> = search_embedding
        .as_array()
        .ok_or(AppError::new_plain("Search embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Build filter conditions
    let mut must_conditions = vec![
        json!({"key": "s", "match": {"value": "s"}})
    ];

    if let Some(zone_id) = request.zone_id {
        must_conditions.push(json!({"key": "z", "match": {"value": zone_id}}));
    }

    if request.min_price.is_some() || request.max_price.is_some() {
        let mut range = serde_json::Map::new();
        if let Some(min) = request.min_price {
            range.insert("gte".to_string(), json!(min));
        }
        if let Some(max) = request.max_price {
            range.insert("lte".to_string(), json!(max));
        }
        must_conditions.push(json!({"key": "c", "range": range}));
    }

    let search_body = json!({
        "vector": search_embedding_floats,
        "filter": {
            "must": must_conditions
        },
        "limit": limit,
        "with_payload": true
    });

    let search_result = qdrant_post(
        &qdrant_path("collections/i/points/search").await?,
        search_body
    ).await?;

    let points = search_result["result"]
        .as_array()
        .ok_or_else(|| AppError::new_plain("Failed to extract points from response"))?;

    let services: Vec<ServiceSearchResult> = points
        .iter()
        .filter_map(|point| {
            let payload = &point["payload"];
            Some(ServiceSearchResult {
                id: point["id"].as_str()?.to_string(),
                description: payload["t"].as_str()?.to_string(),
                price: payload["c"].as_f64()?,
                user_id: payload["u"].as_str()?.to_string(),
                zone_id: payload["z"].as_str().map(|s| s.to_string()),
                images: payload["images"].as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default(),
                location: payload["l"].as_str().unwrap_or("").to_string(),
                position: payload["p"].clone(),
                score: point["score"].as_f64().map(|f| f as f32),
            })
        })
        .collect();

    Ok(ServiceSearchResponse { services })
} 