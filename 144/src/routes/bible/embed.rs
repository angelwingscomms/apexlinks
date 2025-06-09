use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use warp::{Filter, Reply, Rejection};
use std::convert::Infallible;

use crate::{
    gemini_embed::{BibleVerse, process_bible_verses, get_embedding_status, reset_embedding_progress, EmbeddingProgress},
    util::{AppError, AppResult},
};

/// Request for embedding operation
#[derive(Debug, Deserialize)]
pub struct EmbedRequest {
    // No tenant_id needed since we use s="v" for all verses
}

/// Response for the embedding operation
#[derive(Debug, Serialize)]
pub struct EmbedResponse {
    /// Status of the operation
    pub status: String,
    /// Number of verses processed
    pub count: usize,
    /// Current progress information
    pub progress: Option<EmbeddingProgress>,
}

/// Define the route for embedding Bible verses from ylt.json
pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let embed_route = warp::path("bible")
        .and(warp::path("embed"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(r_embed);
        
    let status_route = warp::path("bible")
        .and(warp::path("embed"))
        .and(warp::path("status"))
        .and(warp::get())
        .and_then(r_status);
        
    let reset_route = warp::path("bible")
        .and(warp::path("embed"))
        .and(warp::path("reset"))
        .and(warp::post())
        .and_then(r_reset);
        
    embed_route.or(status_route).or(reset_route)
}

/// Public route handler for embedding
pub async fn r_embed(request: EmbedRequest) -> Result<impl Reply, Infallible> {
    Ok(f_embed(request).await.map_or_else(
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

/// Public route handler for status check
pub async fn r_status() -> Result<impl Reply, Infallible> {
    Ok(f_status().await.map_or_else(
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

/// Public route handler for reset
pub async fn r_reset() -> Result<impl Reply, Infallible> {
    Ok(f_reset().await.map_or_else(
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

/// Internal business logic for embedding
async fn f_embed(_req: EmbedRequest) -> AppResult<EmbedResponse> {
    log::info!("Processing Bible verse embedding request");
    
    // Default file path for YLT verses
    let file_path = "ylt.json";
    
    // Get current progress before starting
    let _initial_progress = get_embedding_status().await?;
    
    // Count total verses
    let verse_count = count_verses_in_file(file_path).await?;
    
    // Process the embedding
    process_bible_verses(file_path).await?;
    
    // Get final progress
    let final_progress = get_embedding_status().await?;
    
    Ok(EmbedResponse {
        status: "success".to_string(),
        count: verse_count,
        progress: final_progress,
    })
}

/// Internal business logic for status check
async fn f_status() -> AppResult<EmbedResponse> {
    log::info!("Checking Bible verse embedding status");
    
    // Default file path for YLT verses
    let file_path = "ylt.json";
    
    // Count total verses
    let verse_count = count_verses_in_file(file_path).await?;
    
    // Get current progress
    let progress = get_embedding_status().await?;
    
    Ok(EmbedResponse {
        status: "status_check".to_string(),
        count: verse_count,
        progress,
    })
}

/// Internal business logic for reset
async fn f_reset() -> AppResult<serde_json::Value> {
    log::info!("Resetting Bible verse embedding progress");
    
    reset_embedding_progress().await?;
    
    Ok(serde_json::json!({
        "status": "reset_complete",
        "message": "Embedding progress has been reset"
    }))
}

/// Count verses in file
async fn count_verses_in_file(file_path: &str) -> AppResult<usize> {
    // Validate inputs
    if file_path.is_empty() || file_path.len() > 500 {
        return Err(AppError::new_plain("Invalid file path"));
    }
    
    // Open and read the file asynchronously to count verses
    let mut file = File::open(file_path)
        .await
        .map_err(|e| AppError::new(&format!("failed to open file {}", file_path), e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .map_err(|e| AppError::new("failed to read file", e))?;
    
    let verses: Vec<BibleVerse> = serde_json::from_str(&contents)
        .map_err(|e| AppError::new("failed to parse Bible verses JSON", e))?;
    
    Ok(verses.len())
} 