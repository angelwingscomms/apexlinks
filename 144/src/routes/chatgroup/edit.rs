use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::util::{AppResult, AppError, embedding, qdrant::{qdrant_path, qdrant_post}};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGroupEditRequest {
    pub n: Option<String>,   // name
    pub t: Option<String>,   // text
    pub l: Option<String>,   // link (can be updated but must be provided in initial creation)
    pub i: Option<String>,   // image
    pub z: Option<String>,   // zone id
    pub a: Option<String>,   // additional field
    pub u: String,           // user id for auth check
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chatgroup" / "edit" / String)
        .and(warp::put())
        .and(warp::body::json())
        .then(handler)
}

pub async fn handler(chatgroup_id: String, request: ChatGroupEditRequest) -> impl Reply {
    match edit_chatgroup(chatgroup_id, request).await {
        Ok(_) => {
            let success_msg = "Chat group updated successfully".to_string();
            warp::reply::with_status(
                success_msg,
                warp::http::StatusCode::OK,
            )
        },
        Err(e) => {
            log::error!("Chat group edit error: {:#?}", e);
            let error_msg = format!("Error: {}", e);
            warp::reply::with_status(
                error_msg,
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn edit_chatgroup(chatgroup_id: String, request: ChatGroupEditRequest) -> AppResult<()> {
    // Get current chat group data
    let current_group = get_chatgroup(&chatgroup_id).await?;
    
    // Check if the user is the owner of the chat group
    if current_group.user_id != request.u {
        return Err(AppError::new_plain("Not authorized to edit this chat group"));
    }
    
    // Prepare updated data
    let mut payload_update = json!({});
    
    if let Some(name) = &request.n {
        payload_update["n"] = json!(name);
    }
    
    if let Some(text) = &request.t {
        payload_update["t"] = json!(text);
    }
    
    if let Some(link) = &request.l {
        payload_update["l"] = json!(link);
    }
    
    if let Some(image) = &request.i {
        payload_update["i"] = json!(image);
    }
    
    if let Some(zone) = &request.z {
        payload_update["z"] = json!(zone);
    }
    
    if let Some(additional) = &request.a {
        payload_update["a"] = json!(additional);
    }
    
    // If name or text was updated, we need to update the vector too
    let mut vector_update = None;
    if request.n.is_some() || request.t.is_some() {
        let name = request.n.as_ref().unwrap_or(&current_group.name);
        let text = request.t.as_ref().unwrap_or(&current_group.text);
        
        let embedding_text = format!("{} {}", name, text);
        let embedding_vec = embedding(embedding_text).await?;
        
        let embedding_floats: Vec<f32> = embedding_vec
            .as_array()
            .ok_or(AppError::new_plain("Embedding is not an array"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
            
        vector_update = Some(embedding_floats);
    }
    
    // Prepare the update request
    let mut update_json = json!({
        "points": [{
            "id": chatgroup_id,
            "payload": payload_update
        }]
    });
    
    // Add vector update if needed
    if let Some(vector) = vector_update {
        update_json["points"][0]["vector"] = json!(vector);
    }
    
    // Update the chat group in the database
    qdrant_post(
        &qdrant_path("collections/i/points?wait=true").await?,
        update_json
    ).await?;
    
    Ok(())
}

#[derive(Debug)]
struct ChatGroup {
    name: String,
    text: String,
    user_id: String,
}

async fn get_chatgroup(chatgroup_id: &str) -> AppResult<ChatGroup> {
    let response = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({
            "ids": [chatgroup_id],
            "with_payload": ["n", "t", "u"]
        })
    ).await?;

    let result = response["result"].as_array()
        .and_then(|arr| arr.first())
        .ok_or(AppError::new_plain("Chat group not found"))?;

    let payload = &result["payload"];
    
    Ok(ChatGroup {
        name: payload["n"].as_str().unwrap_or("").to_string(),
        text: payload["t"].as_str().unwrap_or("").to_string(),
        user_id: payload["u"].as_str().unwrap_or("").to_string(),
    })
} 