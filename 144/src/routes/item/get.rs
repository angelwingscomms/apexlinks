use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::util::{AppResult, AppError, qdrant::{qdrant_path, qdrant_post}};

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: String,
    pub description: String,
    pub price: f64,
    pub user_id: String,
    pub zone_id: Option<String>,
    pub images: Vec<String>,
    pub location: String,
    pub position: serde_json::Value,
    pub item_type: String, // "product" or "service"
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / "get" / String)
        .and(warp::get())
        .then(handler)
}

pub async fn handler(item_id: String) -> impl Reply {
    match get_item(item_id).await {
        Ok(item) => warp::reply::with_status(
            serde_json::to_string(&item).unwrap_or_else(|_| "{}".to_string()),
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Item get error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn get_item(item_id: String) -> AppResult<ItemResponse> {
    // Fetch the item from Qdrant
    let item_result = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [item_id],
            "with_payload": true
        })
    ).await?;

    let item = item_result["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("Item not found"))?;

    let payload = &item["payload"];
    
    // Determine item type
    let item_type = match payload["s"].as_str() {
        Some("p") => "product",
        Some("s") => "service",
        _ => return Err(AppError::new_plain("Unknown item type")),
    };

    Ok(ItemResponse {
        id: item["id"].as_str().unwrap_or("").to_string(),
        description: payload["t"].as_str().unwrap_or("").to_string(),
        price: payload["c"].as_f64().unwrap_or(0.0),
        user_id: payload["u"].as_str().unwrap_or("").to_string(),
        zone_id: payload["z"].as_str().map(|s| s.to_string()),
        images: payload["images"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        location: payload["l"].as_str().unwrap_or("").to_string(),
        position: payload["p"].clone(),
        item_type: item_type.to_string(),
    })
} 