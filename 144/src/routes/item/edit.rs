use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post, qdrant_put}},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemEditRequest {
    pub description: Option<String>,
    pub price: Option<f64>,
    pub images: Option<Vec<String>>,
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / "edit" / String)
        .and(warp::put())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(item_id: String, request: ItemEditRequest) -> impl Reply {
    match edit_item(item_id, request).await {
        Ok(_) => warp::reply::with_status(
            "Item updated successfully",
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Item edit error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn edit_item(item_id: String, request: ItemEditRequest) -> AppResult<()> {
    // First, get the current item to check if it exists and to get current values
    let current_item = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [item_id],
            "with_payload": true,
            "with_vector": true
        })
    ).await?;

    let item = current_item["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("Item not found"))?;

    let mut payload = item["payload"].clone();
    let mut needs_vector_update = false;

    // Update description if provided
    if let Some(description) = request.description {
        payload["t"] = json!(description);
        needs_vector_update = true;
    }

    // Update price if provided
    if let Some(price) = request.price {
        payload["c"] = json!(price);
    }

    // Update images if provided
    if let Some(images) = request.images {
        payload["images"] = json!(images);
    }

    // Create updated point
    let mut update_data = json!({
        "points": [
            {
                "id": item_id,
                "payload": payload
            }
        ]
    });

    // If description changed, update the vector embedding
    if needs_vector_update {
        let description = payload["t"].as_str().unwrap_or("");
        let embedding_vec = embedding(description.to_string()).await?;
        
        let embedding_floats: Vec<f32> = embedding_vec
            .as_array()
            .ok_or(AppError::new_plain("Embedding is not an array"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        update_data["points"][0]["vector"] = json!(embedding_floats);
    }

    // Send the update request
    qdrant_put(
        &qdrant_path("collections/i/points?wait=true").await?,
        update_data
    ).await?;

    Ok(())
} 