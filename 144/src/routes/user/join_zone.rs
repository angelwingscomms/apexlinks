use warp::{Filter, Reply};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, qdrant::{qdrant_path, qdrant_post, qdrant_get}},
};
use super::super::zone::types::UserJoinZoneRequest;

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user" / "join_zone")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: UserJoinZoneRequest) -> impl Reply {
    match join_zone(request).await {
        Ok(message) => warp::reply::with_status(
            message,
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Join zone error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn join_zone(request: UserJoinZoneRequest) -> AppResult<String> {
    // TODO: Get user ID from authentication context
    // For now, using a placeholder user ID
    let user_id = "placeholder_user_id";

    // Validate zone exists
    let zone_check = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [request.zone_id],
            "with_payload": true
        })
    ).await?;

    if zone_check["result"].as_array().unwrap_or(&vec![]).is_empty() {
        return Err(AppError::new_plain("Zone not found"));
    }

    // Update user's zone field
    qdrant_post(
        &qdrant_path("collections/i/points/payload?wait=true").await?,
        json!({
            "payload": {"z": request.zone_id},
            "points": [user_id]
        })
    ).await?;

    Ok(format!("Successfully joined zone: {}", request.zone_id))
} 