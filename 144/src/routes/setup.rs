use warp::{Filter, Reply, Rejection};
use std::convert::Infallible;
use serde_json::json;
use reqwest::Client;

use crate::util::{AppError, AppResult};
use crate::util::qdrant::{qdrant_path, qdrant_put};

/// Setup route to recreate collections with correct dimensions
pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("setup")
        .and(warp::post())
        .and_then(r_setup)
}

/// Public route handler for setup
pub async fn r_setup() -> Result<impl Reply, Infallible> {
    Ok(f_setup().await.map_or_else(
        |e| {
            log::error!("{:#?}", e);
            warp::reply::with_status(
                warp::reply::json(&"An error occurred on our side".to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        },
        |v| warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::OK),
    ))
}

/// Internal business logic for setup
async fn f_setup() -> AppResult<serde_json::Value> {
    log::info!("Setting up Qdrant collections with correct vector dimensions");

    // Delete existing collection i if it exists
    if let Err(e) = delete_collection("i").await {
        log::warn!("Could not delete collection i (may not exist): {}", e);
    }

    // Recreate collection i with correct vector size (768 for Gemini embeddings)
    log::info!("Creating collection 'i' with 768 dimensions");
    qdrant_put(
        &qdrant_path("collections/i?wait=true").await?,
        json!({"vectors": {"size": 768, "distance": "Cosine"}}),
    ).await?;

    // Ensure collection r exists
    log::info!("Creating collection 'r' for ID tracking");
    qdrant_put(
        &qdrant_path("collections/r?wait=true").await?,
        json!({"vectors": {"size": 1, "distance": "Cosine"}}),
    ).await?;

    // Add initial point to collection r
    qdrant_put(
        &qdrant_path("collections/r/points?wait=true").await?,
        json!({"points": [{
            "id": "b4ea369a-d21e-40b4-afe7-4e84a4a7cd91",
            "payload": {
                "i": 0
            },
            "vector": [0]
        }]}),
    ).await?;
    
    // Initialize chat message collections
    log::info!("Initializing chat message collection");
    if let Err(e) = crate::routes::chat::storage::init_messages_collection().await {
        log::error!("Error initializing chat message collection: {}", e);
    }

    log::info!("Collections setup completed successfully");

    Ok(json!({
        "status": "success",
        "message": "Collections recreated with correct dimensions",
        "collections": {
            "i": "768-dimensional vectors for embeddings",
            "r": "1-dimensional vectors for ID tracking",
            "messages": "768-dimensional vectors for chat messages"
        }
    }))
}

async fn delete_collection(collection_name: &str) -> AppResult<()> {
    let url = qdrant_path(&format!("collections/{}", collection_name)).await?;
    
    let response = Client::new()
        .delete(&url)
        .header(
            "api-key",
            crate::constants::SECRETS
                .lock()
                .await
                .get("QDRANT_KEY")
                .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?,
        )
        .send()
        .await
        .map_err(|e| AppError::new("failed to delete collection", e))?;

    if !response.status().is_success() && response.status().as_u16() != 404 {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Failed to delete collection {}: {}", 
            collection_name,
            error_text
        )));
    }

    Ok(())
} 