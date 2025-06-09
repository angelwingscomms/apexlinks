use warp::{Filter, Reply};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, id, qdrant::{qdrant_path, qdrant_post}},
    constants::SECRETS,
};
use super::types::{ZoneAddRequest, Zone, Position};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("zone" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ZoneAddRequest) -> impl Reply {
    match add_zone(request).await {
        Ok(zone_id) => warp::reply::with_status(
            zone_id,
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Zone add error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn add_zone(request: ZoneAddRequest) -> AppResult<String> {
    // Check if similar zone already exists
    if let Err(e) = check_similar_zone(&request.name, &request.description, &request.position).await {
        return Err(e);
    }

    // Upload images to IBM COS if provided
    let uploaded_images = if let Some(images) = request.images {
        upload_images_to_cos(images).await?
    } else {
        Vec::new()
    };

    // Create embedding from zone name and description
    let zone_data = json!({
        "name": request.name,
        "description": request.description
    });
    let embedding_vec = embedding(zone_data.to_string()).await?;
    
    // Convert embedding to Vec<f32>
    let embedding_floats: Vec<f32> = embedding_vec
        .as_array()
        .ok_or(AppError::new_plain("Embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Create zone object
    let zone_id = id();
    let zone = Zone {
        l: request.location_url,
        n: request.name,
        i: uploaded_images,
        p: request.position,
        s: "z".to_string(),
        t: request.description,
        embedding: embedding_floats.clone(),
    };

    // Store in Qdrant
    let point = json!({
        "id": zone_id,
        "vector": embedding_floats,
        "payload": {
            "l": zone.l,
            "n": zone.n,
            "i": zone.i,
            "p": zone.p,
            "s": zone.s,
            "t": zone.t
        }
    });

    qdrant_post(
        &qdrant_path("collections/i/points?wait=true").await?,
        json!({
            "points": [point]
        })
    ).await?;

    Ok(zone_id)
}

async fn check_similar_zone(name: &str, description: &str, position: &Position) -> AppResult<()> {
    // Create embedding for similarity search
    let search_data = json!({
        "name": name,
        "description": description
    });
    let search_embedding = embedding(search_data.to_string()).await?;
    
    let search_embedding_floats: Vec<f32> = search_embedding
        .as_array()
        .ok_or(AppError::new_plain("Search embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Search for similar zones
    let search_body = json!({
        "vector": search_embedding_floats,
        "filter": {
            "must": [
                {"key": "s", "match": {"value": "z"}}
            ]
        },
        "limit": 18,
        "with_payload": true
    });

    let search_result = qdrant_post(
        &qdrant_path("collections/i/points/search").await?,
        search_body
    ).await?;

    // Check if any similar zones exist in the same location
    if let Some(points) = search_result["result"].as_array() {
        for point in points {
            if let Some(score) = point["score"].as_f64() {
                if score > 0.8 { // High similarity threshold
                    if let Some(payload) = point["payload"].as_object() {
                        if let (Some(lat), Some(lng)) = (
                            payload.get("p").and_then(|p| p.get("lat")).and_then(|v| v.as_f64()),
                            payload.get("p").and_then(|p| p.get("lng")).and_then(|v| v.as_f64())
                        ) {
                            // Check if within 10 miles (approximately 0.145 degrees)
                            let distance = ((position.lat - lat).powi(2) + (position.lng - lng).powi(2)).sqrt();
                            if distance < 0.145 {
                                return Err(AppError::new_plain("Similar zone already exists in this location"));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

async fn upload_images_to_cos(images: Vec<String>) -> AppResult<Vec<String>> {
    // For now, return the images as-is
    // TODO: Implement actual IBM COS upload
    // This would require:
    // - IBM_COS_ENDPOINT
    // - IBM_COS_ACCESS_KEY_ID  
    // - IBM_COS_SECRET_ACCESS_KEY
    // - IBM_COS_BUCKET_NAME
    
    log::warn!("IBM COS upload not yet implemented, returning original image URLs");
    Ok(images)
} 