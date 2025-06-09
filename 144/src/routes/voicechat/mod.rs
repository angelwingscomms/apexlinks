use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::util::AppResult;

pub mod user_tags;
pub mod matching;
pub mod signaling;

#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceChatUser {
    pub id: String,
    pub tags: String,
}

/// Get voice chat related routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let voice_chat_routes = warp::path("voicechat");
    
    let register_route = voice_chat_routes
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(user_tags::register_user);
    
    let find_match_route = voice_chat_routes
        .and(warp::path("match"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(matching::find_match);

    let signaling_route = voice_chat_routes
        .and(warp::path("signal"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(signaling::relay_signal);
    
    register_route.or(find_match_route).or(signaling_route)
} 