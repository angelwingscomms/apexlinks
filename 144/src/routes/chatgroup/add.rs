use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, embedding, id, qdrant::{qdrant_path, qdrant_post}},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGroupAddRequest {
    pub n: String,         // name
    pub t: String,         // text
    pub l: String,         // link (now required)
    pub i: Option<String>, // image
    pub u: String,         // user id
    pub z: Option<String>, // zone id
    pub a: Option<String>, // additional field
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chatgroup" / "add")
        .and(warp::post())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(request: ChatGroupAddRequest) -> impl Reply {
    match add_chatgroup(request).await {
        Ok(chatgroup_id) => warp::reply::with_status(
            chatgroup_id,
            warp::http::StatusCode::OK,
        ),
        Err(e) => {
            log::error!("Chat group add error: {:#?}", e);
            warp::reply::with_status(
                format!("Error: {}", e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn add_chatgroup(request: ChatGroupAddRequest) -> AppResult<String> {
    // Get user details to inherit zone and position if not provided
    let user_data = get_user_data(&request.u).await?;
    
    // Create embedding from chat group name and text
    let embedding_text = format!("{} {}", request.n, request.t);
    let embedding_vec = embedding(embedding_text).await?;
    
    let embedding_floats: Vec<f32> = embedding_vec
        .as_array()
        .ok_or(AppError::new_plain("Embedding is not an array"))?
        .iter()
        .map(|v| v.as_f64().unwrap_or(0.0) as f32)
        .collect();

    // Create chat group object
    let chatgroup_id = id();
    let point = json!({
        "id": chatgroup_id,
        "vector": embedding_floats,
        "payload": {
            "n": request.n,               // name
            "t": request.t,               // text
            "l": request.l,               // link
            "i": request.i,               // image
            "u": request.u,               // user
            "z": request.z.or(user_data.zone_id), // zone (inherited if not provided)
            "s": "cg",                    // tenant id for chat groups
            "a": request.a,               // additional field
        }
    });

    qdrant_post(
        &qdrant_path("collections/i/points?wait=true").await?,
        json!({
            "points": [point]
        })
    ).await?;

    Ok(chatgroup_id)
}

#[derive(Debug)]
struct UserData {
    zone_id: Option<String>,
}

async fn get_user_data(user_id: &str) -> AppResult<UserData> {
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

    let payload = &user_data["payload"];
    
    Ok(UserData {
        zone_id: payload["z"].as_str().map(|s| s.to_string()),
    })
} 