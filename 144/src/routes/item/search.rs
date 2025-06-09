use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post}},
};

#[derive(Debug, Deserialize)]
pub struct ItemSearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub zone_id: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub tenant_id: Option<String>,
    pub item_type: Option<String>, // Optional filter: "product", "service", or null for both
}

#[derive(Debug, Serialize)]
pub struct ItemSearchResponse {
    pub items: Vec<ItemSearchResult>,
}

#[derive(Debug, Serialize)]
pub struct ItemSearchResult {
    pub id: String,
    pub description: String,
    pub price: f64,
    pub user_id: String,
    pub zone_id: Option<String>,
    pub images: Vec<String>,
    pub location: String,
    pub position: serde_json::Value,
    pub score: Option<f32>,
    pub item_type: String, // "product" or "service"
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ItemSearchRequest) -> impl Reply {
    match search_items(request).await {
        Ok(results) => warp::reply::with_status(
            serde_json::to_string(&results).unwrap_or_else(|_| "{}".to_string()),
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Item search error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn search_items(request: ItemSearchRequest) -> AppResult<ItemSearchResponse> {
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
    let mut must_conditions = vec![];
    
    // Filter by item type if specified
    if let Some(item_type) = &request.item_type {
        match item_type.as_str() {
            "product" => must_conditions.push(json!({"key": "s", "match": {"value": "p"}})),
            "service" => must_conditions.push(json!({"key": "s", "match": {"value": "s"}})),
            _ => return Err(AppError::new_plain("Invalid item_type value. Must be 'product' or 'service'")),
        }
    } else {
        // If no item type is specified, include both products and services
        must_conditions.push(json!({"key": "s", "match": {"value": "p", "must_not": false}}));
        must_conditions.push(json!({"key": "s", "match": {"value": "s", "must_not": false}}));
    }

    if let Some(zone_id) = request.zone_id {
        must_conditions.push(json!({"key": "z", "match": {"value": zone_id}}));
    }

    if let Some(tenant_id) = request.tenant_id {
        must_conditions.push(json!({"key": "u", "match": {"value": tenant_id}}));
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

    let items: Vec<ItemSearchResult> = points
        .iter()
        .filter_map(|point| {
            let payload = &point["payload"];
            let item_type = match payload["s"].as_str()? {
                "p" => "product",
                "s" => "service",
                _ => return None, // Skip unknown item types
            };
            
            Some(ItemSearchResult {
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
                item_type: item_type.to_string(),
            })
        })
        .collect();

    Ok(ItemSearchResponse { items })
} 