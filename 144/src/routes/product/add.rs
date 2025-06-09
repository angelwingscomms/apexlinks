use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, id, qdrant::{qdrant_path, qdrant_post}},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAddRequest {
    pub description: String,
    pub price: f64,
    pub images: Option<Vec<String>>,
    pub user_id: String, // TODO: Get from auth context
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("product" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ProductAddRequest) -> impl Reply {
    match add_product(request).await {
        Ok(product_id) => warp::reply::with_status(
            product_id,
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Product add error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn add_product(request: ProductAddRequest) -> AppResult<String> {
    // Get user details to inherit zone, location, and position
    let user_data = get_user_data(&request.user_id).await?;
    
    // Create embedding from product description
    let embedding_vec = embedding(request.description.clone()).await?;
    
    let embedding_floats: Vec<f32> = embedding_vec
        .as_array()
        .ok_or(AppError::new_plain("Embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Create product object
    let product_id = id();
    let point = json!({
        "id": product_id,
        "vector": embedding_floats,
        "payload": {
            "t": request.description,  // description
            "c": request.price,        // price
            "u": request.user_id,      // user
            "z": user_data.zone_id,    // zone (inherited)
            "images": request.images.unwrap_or_default(),
            "l": user_data.location,   // location (inherited)
            "p": user_data.position,   // position (inherited)
            "s": "p"                   // tenant id for products
        }
    });

    qdrant_post(
        &qdrant_path("collections/i/points?wait=true").await?,
        json!({
            "points": [point]
        })
    ).await?;

    Ok(product_id)
}

#[derive(Debug)]
struct UserData {
    zone_id: Option<String>,
    location: String,
    position: serde_json::Value,
}

async fn get_user_data(user_id: &str) -> AppResult<UserData> {
    let user_check = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [user_id],
            "with_payload": ["z", "l", "p"]
        })
    ).await?;

    let user_data = user_check["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("User not found"))?;

    let payload = &user_data["payload"];
    
    Ok(UserData {
        zone_id: payload["z"].as_str().map(|s| s.to_string()),
        location: payload["l"].as_str().unwrap_or("").to_string(),
        position: payload["p"].clone(),
    })
} 