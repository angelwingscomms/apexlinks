use warp::{Filter, Reply};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, qdrant::{qdrant_path, qdrant_post}},
};
use super::super::zone::types::UserLeaveZoneRequest;

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user" / "leave_zone")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: UserLeaveZoneRequest) -> impl Reply {
    match leave_zone(request).await {
        Ok(message) => warp::reply::with_status(
            message,
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Leave zone error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn leave_zone(request: UserLeaveZoneRequest) -> AppResult<String> {
    // TODO: Get user ID from authentication context
    // For now, using a placeholder user ID
    let user_id = "placeholder_user_id";

    // Get zone details to check if user created it
    let zone_check = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [request.zone_id],
            "with_payload": true
        })
    ).await?;

    let zone_data = zone_check["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("Zone not found"))?;

    // TODO: Check if user created this zone
    // This would require storing creator information in the zone payload
    // For now, we'll allow leaving any zone
    
    // Validate zone exists and user is in it
    let user_check = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [user_id],
            "with_payload": ["z"]
        })
    ).await?;

    let user_data = user_check["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("User not found"))?;

    let current_zone = user_data["payload"]["z"].as_str();
    if current_zone != Some(&request.zone_id) {
        return Err(AppError::new_plain("User is not in this zone"));
    }

    // Remove user from zone by clearing the zone field
    qdrant_post(
        &qdrant_path("collections/i/points/payload?wait=true").await?,
        json!({
            "payload": {"z": null},
            "points": [user_id]
        })
    ).await?;

    Ok(format!("Successfully left zone: {}", request.zone_id))
} 