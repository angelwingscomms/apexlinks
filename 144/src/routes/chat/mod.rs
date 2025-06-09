pub mod matching;
pub mod websocket;
pub mod types;
pub mod storage;

use warp::Filter;
use warp::filters::cors::cors;
use crate::util::with_auth;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    match_route()
        .or(websocket_route())
        .or(get_messages_route())
        .or(search_messages_route())
        .or(mark_read_route())
        .or(unread_messages_route())
}

fn match_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("chat")
        .and(warp::path("match"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(matching::find_match)
}

fn websocket_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("chat")
        .and(warp::path("ws"))
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(websocket::handle_websocket)
        })
}

// New routes for message functionality

fn get_messages_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chat" / "messages" / String)
        .and(warp::get())
        .and(with_auth())
        .and_then(
            |session_id: String, user_id: String| async move {
                let messages = storage::get_session_messages(&session_id).await
                    .map_err(|e| {
                        log::error!("Error fetching messages: {:?}", e);
                        warp::reject::custom(crate::util::AppError::new_rejection("Failed to fetch messages"))
                    })?;
                
                // Mark all messages as read by this user
                let unread_message_ids = messages.iter()
                    .filter(|m| !m.read_by.contains(&user_id))
                    .map(|m| m.id.clone())
                    .collect::<Vec<_>>();
                
                if !unread_message_ids.is_empty() {
                    storage::mark_messages_as_read(&user_id, &unread_message_ids).await
                        .map_err(|e| {
                            log::error!("Error marking messages as read: {:?}", e);
                            warp::reject::custom(crate::util::AppError::new_rejection("Failed to mark messages as read"))
                        })?;
                }
                
                Ok::<_, warp::Rejection>(warp::reply::json(&messages))
            }
        )
}

fn search_messages_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chat" / "search")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_auth())
        .and_then(
            |search: serde_json::Value, user_id: String| async move {
                let query = search["query"].as_str()
                    .ok_or_else(|| warp::reject::custom(crate::util::AppError::new_rejection("Missing query parameter")))?;
                
                let limit = search["limit"].as_u64().unwrap_or(20) as usize;
                
                let messages = storage::search_messages(&user_id, query, limit).await
                    .map_err(|e| {
                        log::error!("Error searching messages: {:?}", e);
                        warp::reject::custom(crate::util::AppError::new_rejection("Failed to search messages"))
                    })?;
                
                Ok::<_, warp::Rejection>(warp::reply::json(&messages))
            }
        )
}

fn mark_read_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chat" / "mark-read")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_auth())
        .and_then(
            |body: serde_json::Value, user_id: String| async move {
                let message_ids = body["message_ids"].as_array()
                    .ok_or_else(|| warp::reject::custom(crate::util::AppError::new_rejection("Missing message_ids parameter")))?
                    .iter()
                    .filter_map(|id| id.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>();
                
                storage::mark_messages_as_read(&user_id, &message_ids).await
                    .map_err(|e| {
                        log::error!("Error marking messages as read: {:?}", e);
                        warp::reject::custom(crate::util::AppError::new_rejection("Failed to mark messages as read"))
                    })?;
                
                Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({ "success": true })))
            }
        )
}

fn unread_messages_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chat" / "unread")
        .and(warp::get())
        .and(with_auth())
        .and_then(
            |user_id: String| async move {
                let messages = storage::get_unread_messages(&user_id).await
                    .map_err(|e| {
                        log::error!("Error fetching unread messages: {:?}", e);
                        warp::reject::custom(crate::util::AppError::new_rejection("Failed to fetch unread messages"))
                    })?;
                
                Ok::<_, warp::Rejection>(warp::reply::json(&messages))
            }
        )
} 