use serde::Deserialize;
use warp::{Filter, Reply, Rejection};
use std::convert::Infallible;

use crate::{
    gemini_embed::{search_similar_verses, VerseMetadata},
    util::{AppError, AppResult},
};

/// Search request for finding similar Bible verses
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    /// The query text to find similar verses for
    pub query: String,
    /// The maximum number of results to return
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    5
}

/// Define the route for searching Bible verses
pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("verses")
        .and(warp::path("search"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(r)
}

/// Public route handler function
pub async fn r(request: SearchRequest) -> Result<impl Reply, Infallible> {
    Ok(f(request).await.map_or_else(
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

/// Internal business logic function
async fn f(req: SearchRequest) -> AppResult<Vec<VerseMetadata>> {
    // Validate query length
    if req.query.len() > 200 {
        return Err(AppError::new_plain("Query text too long. Please limit to 200 characters."));
    }
    
    // Limit to reasonable number
    let limit = if req.limit > 100 { 100 } else { req.limit };
    
    // Search for similar verses
    let results = search_similar_verses(&req.query, limit).await?;
    
    Ok(results)
} 