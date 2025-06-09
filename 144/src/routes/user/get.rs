use serde::{Deserialize, Serialize};
use warp::{Filter, Reply, Rejection};
use serde_json::json;

use crate::util::{AppError, AppResult};
use crate::util::qdrant::{qdrant_path, qdrant_post};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub username: Option<String>,
    pub picture: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub zone_id: Option<String>,
}

pub fn route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and_then(r)
}

pub async fn r(user_id: String) -> Result<impl Reply, Rejection> {
    f(user_id).await.map_or_else(
        |e| {
            log::error!("Error fetching user: {:#?}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({"error": "User not found"})),
                warp::http::StatusCode::NOT_FOUND,
            ))
        },
        |v| Ok(warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::OK)),
    )
}

async fn f(user_id: String) -> AppResult<UserResponse> {
    log::info!("Fetching user with ID: {}", user_id);
    
    // Retrieve user from Qdrant
    let user_result = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [user_id],
            "with_payload": true
        })
    ).await?;
    
    let user_points = user_result["result"].as_array()
        .ok_or_else(|| AppError::new_plain("Invalid response format"))?;
        
    if user_points.is_empty() {
        return Err(AppError::new_plain("User not found"));
    }
    
    let user_data = &user_points[0];
    let payload = &user_data["payload"];
    
    // Extract user properties from payload
    // The payload keys are shortened to save space (n = name, t = description, etc.)
    let user = UserResponse {
        id: user_id,
        name: payload["n"].as_str().map(|s| s.to_string()),
        email: payload["email"].as_str().map(|s| s.to_string()),
        description: payload["t"].as_str().map(|s| s.to_string()),
        username: payload["u"].as_str().map(|s| s.to_string()),
        picture: payload["p"].as_str().map(|s| s.to_string()),
        age: payload["age"].as_i64().map(|i| i as i32),
        gender: payload["g"].as_str().map(|s| s.to_string()),
        zone_id: payload["z"].as_str().map(|s| s.to_string()),
    };
    
    log::debug!("Successfully retrieved user data for ID: {}", user.id);
    Ok(user)
} 