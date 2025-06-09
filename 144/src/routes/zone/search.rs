use warp::{Filter, Reply};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post}},
};
use super::types::ZoneSearchRequest;

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("zone" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ZoneSearchRequest) -> impl Reply {
    match search_zones(request).await {
        Ok(results) => warp::reply::with_status(
            serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string()),
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Zone search error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn search_zones(request: ZoneSearchRequest) -> AppResult<serde_json::Value> {
    // Create embedding for the search query
    let search_embedding = embedding(request.query).await?;
    
    let search_embedding_floats: Vec<f32> = search_embedding
        .as_array()
        .ok_or(AppError::new_plain("Search embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Build search body with zone filter
    let mut search_body = json!({
        "vector": search_embedding_floats,
        "filter": {
            "must": [
                {"key": "s", "match": {"value": "z"}}
            ]
        },
        "limit": 50,
        "with_payload": true
    });

    // Perform the search
    let search_result = qdrant_post(
        &qdrant_path("collections/i/points/search").await?,
        search_body
    ).await?;

    // Process results and apply location filtering if provided
    let mut results = Vec::new();
    
    if let Some(points) = search_result["result"].as_array() {
        for point in points {
            if let Some(payload) = point["payload"].as_object() {
                // Apply location-based filtering if coordinates and radius are provided
                if let (Some(search_lat), Some(search_lng), Some(radius)) = 
                    (request.lat, request.lng, request.radius) {
                    
                    if let (Some(zone_lat), Some(zone_lng)) = (
                        payload.get("p").and_then(|p| p.get("lat")).and_then(|v| v.as_f64()),
                        payload.get("p").and_then(|p| p.get("lng")).and_then(|v| v.as_f64())
                    ) {
                        let distance_miles = calculate_distance(search_lat, search_lng, zone_lat, zone_lng);
                        if distance_miles > radius {
                            continue; // Skip this zone if it's outside the radius
                        }
                    }
                }
                
                // Add score to the result
                let mut result = payload.clone();
                if let Some(score) = point["score"].as_f64() {
                    result.insert("score".to_string(), json!(score));
                }
                if let Some(id) = point["id"].as_str() {
                    result.insert("id".to_string(), json!(id));
                }
                
                results.push(result);
            }
        }
    }

    Ok(json!(results))
}

// Calculate distance between two points in miles using Haversine formula
fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let r = 3959.0; // Earth's radius in miles
    let dlat = (lat2 - lat1).to_radians();
    let dlng = (lng2 - lng1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    r * c
} 