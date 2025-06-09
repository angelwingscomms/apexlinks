use warp::{Filter, Reply};
use serde_json::json;
use crate::util::{AppResult, AppError, qdrant::{qdrant_path, qdrant_delete}};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / "delete" / String)
        .and(warp::delete())
        .then(handler)
}

pub async fn handler(item_id: String) -> impl Reply {
    match delete_item(item_id).await {
        Ok(_) => warp::reply::with_status(
            "Item deleted successfully",
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Item delete error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn delete_item(item_id: String) -> AppResult<()> {
    // Delete the item from the Qdrant collection
    let delete_result = qdrant_delete(
        &qdrant_path("collections/i/points").await?,
        json!({
            "points": [item_id],
            "wait": true
        })
    ).await?;

    // Check if deletion was successful
    if let Some(status) = delete_result.get("status") {
        if status.as_str() == Some("ok") {
            return Ok(());
        }
    }

    Err(AppError::new_plain("Failed to delete item"))
} 