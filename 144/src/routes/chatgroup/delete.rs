use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::util::{AppResult, AppError, qdrant::{qdrant_path, qdrant_post}};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGroupDeleteRequest {
    pub user_id: String,
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chatgroup" / "delete" / String)
        .and(warp::delete())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(chatgroup_id: String, request: ChatGroupDeleteRequest) -> impl Reply {
    match delete_chatgroup(chatgroup_id, request).await {
        Ok(_) => {
            let success_msg = "Chat group deleted successfully".to_string();
            warp::reply::with_status(
                success_msg,
                warp::http::StatusCode::OK,
            )
        },
        Err(e) => {
            log::error!("Chat group delete error: {:#?}", e);
            let error_msg = format!("Error: {}", e);
            warp::reply::with_status(
                error_msg,
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn delete_chatgroup(chatgroup_id: String, request: ChatGroupDeleteRequest) -> AppResult<()> {
    // Check if the user is the owner of the chat group
    let is_owner = check_ownership(&chatgroup_id, &request.user_id).await?;
    
    if !is_owner {
        return Err(AppError::new_plain("Not authorized to delete this chat group"));
    }
    
    // Delete the chat group
    qdrant_post(
        &qdrant_path("collections/i/points/delete").await?,
        json!({
            "points": [chatgroup_id]
        })
    ).await?;
    
    Ok(())
}

async fn check_ownership(chatgroup_id: &str, user_id: &str) -> AppResult<bool> {
    let response = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [chatgroup_id],
            "with_payload": ["u"]
        })
    ).await?;

    let result = response["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("Chat group not found"))?;

    let payload = &result["payload"];
    let owner_id = payload["u"].as_str().unwrap_or("");
    
    Ok(owner_id == user_id)
} 