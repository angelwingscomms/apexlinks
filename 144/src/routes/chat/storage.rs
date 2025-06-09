use crate::util::{AppResult, AppError, embed::embed, qdrant::{qdrant_path, qdrant_post, qdrant_put, qdrant_get}};
use super::types::{ChatMessage, MessageType};
use serde_json::json;
use uuid::Uuid;
use futures::future::join_all;

const CHAT_COLLECTION: &str = "messages";

/// Initialize the messages collection in Qdrant
pub async fn init_messages_collection() -> AppResult<()> {
    // Check if collection exists first
    let collections_url = qdrant_path("collections").await?;
    let collections = qdrant_get(&collections_url).await?;
    
    let collections_list = collections["result"]["collections"].as_array()
        .ok_or_else(|| AppError::new_plain("Invalid response format"))?;
    
    let collection_exists = collections_list.iter()
        .any(|c| c["name"].as_str().unwrap_or("") == CHAT_COLLECTION);
    
    if !collection_exists {
        // Create the messages collection
        let create_url = qdrant_path(&format!("collections/{}", CHAT_COLLECTION)).await?;
        let create_body = json!({
            "vectors": {
                "size": 768,
                "distance": "Cosine"
            },
            "optimizers_config": {
                "indexed_only": false
            },
            "shard_number": 1,
            "on_disk_payload": true
        });
        
        qdrant_put(&create_url, create_body).await?;
        
        // Create necessary payload indexes
        let index_url = qdrant_path(&format!("collections/{}/index", CHAT_COLLECTION)).await?;
        
        // Create index for session_id
        let session_id_index = json!({
            "field_name": "session_id",
            "field_schema": "keyword"
        });
        qdrant_put(&index_url, session_id_index).await?;
        
        // Create index for sender_id
        let sender_id_index = json!({
            "field_name": "sender_id",
            "field_schema": "keyword"
        });
        qdrant_put(&index_url, sender_id_index).await?;
        
        // Create index for timestamp
        let timestamp_index = json!({
            "field_name": "timestamp",
            "field_schema": "integer"
        });
        qdrant_put(&index_url, timestamp_index).await?;
        
        // Create index for read_by (for unread messages)
        let read_by_index = json!({
            "field_name": "read_by",
            "field_schema": "keyword"
        });
        qdrant_put(&index_url, read_by_index).await?;
    }
    
    Ok(())
}

/// Save a message with embedding
pub async fn save_message(mut message: ChatMessage) -> AppResult<ChatMessage> {
    // Only generate embeddings for text messages
    if message.message_type == MessageType::Text && message.embedding.is_none() {
        message.embedding = Some(embed(message.message.clone()).await?);
    }
    
    let points_url = qdrant_path(&format!("collections/{}/points", CHAT_COLLECTION)).await?;
    
    let vector = match &message.embedding {
        Some(embedding) => embedding.clone(),
        None => vec![0.0; 768], // Default vector for system messages
    };
    
    let point = json!({
        "id": message.id,
        "vector": vector,
        "payload": {
            "session_id": message.session_id,
            "sender_id": message.sender_id,
            "message": message.message,
            "timestamp": message.timestamp,
            "message_type": serde_json::to_string(&message.message_type)
                .map_err(|e| AppError::new("Failed to serialize message type", e))?,
            "read_by": message.read_by
        }
    });
    
    let body = json!({
        "points": [point]
    });
    
    qdrant_put(&points_url, body).await?;
    
    Ok(message)
}

/// Get all messages for a session
pub async fn get_session_messages(session_id: &str) -> AppResult<Vec<ChatMessage>> {
    let search_url = qdrant_path(&format!("collections/{}/points/scroll", CHAT_COLLECTION)).await?;
    
    let body = json!({
        "filter": {
            "must": [
                {
                    "key": "session_id",
                    "match": {
                        "value": session_id
                    }
                }
            ]
        },
        "limit": 100,
        "with_vector": false,
        "with_payload": true
    });
    
    let response = qdrant_post(&search_url, body).await?;
    
    parse_message_results(response)
}

/// Search messages by content similarity
pub async fn search_messages(user_id: &str, query: &str, limit: usize) -> AppResult<Vec<ChatMessage>> {
    // Generate embedding for the search query
    let query_embedding = embed(query.to_string()).await?;
    
    // First get all sessions the user is part of
    let sessions = get_user_sessions(user_id).await?;
    
    if sessions.is_empty() {
        return Ok(vec![]);
    }
    
    // Create filter for sessions
    let session_filters = sessions.iter()
        .map(|session_id| {
            json!({
                "key": "session_id",
                "match": {
                    "value": session_id
                }
            })
        })
        .collect::<Vec<_>>();
    
    let search_url = qdrant_path(&format!("collections/{}/points/search", CHAT_COLLECTION)).await?;
    
    let body = json!({
        "vector": query_embedding,
        "filter": {
            "must": [
                {
                    "should": session_filters
                }
            ]
        },
        "limit": limit,
        "with_vector": false,
        "with_payload": true
    });
    
    let response = qdrant_post(&search_url, body).await?;
    
    parse_message_results(response)
}

/// Get unread messages for a user
pub async fn get_unread_messages(user_id: &str) -> AppResult<Vec<ChatMessage>> {
    // First get all sessions the user is part of
    let sessions = get_user_sessions(user_id).await?;
    
    if sessions.is_empty() {
        return Ok(vec![]);
    }
    
    // Create filter for sessions
    let session_filters = sessions.iter()
        .map(|session_id| {
            json!({
                "key": "session_id",
                "match": {
                    "value": session_id
                }
            })
        })
        .collect::<Vec<_>>();
    
    let search_url = qdrant_path(&format!("collections/{}/points/scroll", CHAT_COLLECTION)).await?;
    
    let body = json!({
        "filter": {
            "must": [
                {
                    "should": session_filters
                },
                {
                    "key": "sender_id",
                    "match": {
                        "value": {"$ne": user_id}
                    }
                },
                {
                    "key": "read_by",
                    "match": {
                        "value": {"$ne": user_id}
                    }
                }
            ]
        },
        "limit": 100,
        "with_vector": false,
        "with_payload": true
    });
    
    let response = qdrant_post(&search_url, body).await?;
    
    parse_message_results(response)
}

/// Mark messages as read by a user
pub async fn mark_messages_as_read(user_id: &str, message_ids: &[String]) -> AppResult<()> {
    if message_ids.is_empty() {
        return Ok(());
    }
    
    let futures = message_ids.iter().map(|message_id| {
        mark_single_message_as_read(user_id, message_id)
    });
    
    // Execute all updates in parallel
    let results = join_all(futures).await;
    
    // Check for errors
    for result in results {
        if let Err(e) = result {
            return Err(e);
        }
    }
    
    Ok(())
}

async fn mark_single_message_as_read(user_id: &str, message_id: &str) -> AppResult<()> {
    // First get the current message
    let get_url = qdrant_path(&format!("collections/{}/points", CHAT_COLLECTION)).await?;
    
    let get_body = json!({
        "ids": [message_id],
        "with_payload": true
    });
    
    let response = qdrant_post(&get_url, get_body).await?;
    
    let points = response["result"].as_array()
        .ok_or_else(|| AppError::new_plain("Invalid response format"))?;
    
    if points.is_empty() {
        return Err(AppError::new_plain("Message not found"));
    }
    
    let point = &points[0];
    let payload = &point["payload"];
    
    // Extract current read_by array
    let mut read_by: Vec<String> = match payload["read_by"].as_array() {
        Some(array) => array.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        None => vec![]
    };
    
    // Add user_id if not already present
    if !read_by.contains(&user_id.to_string()) {
        read_by.push(user_id.to_string());
        
        // Update the message
        let update_url = qdrant_path(&format!("collections/{}/points", CHAT_COLLECTION)).await?;
        
        let update_body = json!({
            "points": [
                {
                    "id": message_id,
                    "payload": {
                        "read_by": read_by
                    }
                }
            ]
        });
        
        qdrant_put(&update_url, update_body).await?;
    }
    
    Ok(())
}

/// Get all session IDs a user is part of
async fn get_user_sessions(user_id: &str) -> AppResult<Vec<String>> {
    let sessions_url = qdrant_path("collections/sessions/points/scroll").await?;
    
    let body = json!({
        "filter": {
            "should": [
                {
                    "key": "user1_id",
                    "match": {
                        "value": user_id
                    }
                },
                {
                    "key": "user2_id",
                    "match": {
                        "value": user_id
                    }
                }
            ]
        },
        "limit": 100,
        "with_vector": false,
        "with_payload": true
    });
    
    let response = qdrant_post(&sessions_url, body).await?;
    
    let points = response["result"]["points"].as_array()
        .ok_or_else(|| AppError::new_plain("Invalid response format"))?;
    
    let session_ids = points.iter()
        .filter_map(|p| p["id"].as_str().map(|s| s.to_string()))
        .collect();
    
    Ok(session_ids)
}

/// Parse message results from Qdrant response
fn parse_message_results(response: serde_json::Value) -> AppResult<Vec<ChatMessage>> {
    let points = match response["result"].get("points").and_then(|p| p.as_array()) {
        Some(points) => points,
        None => response["result"].as_array()
            .ok_or_else(|| AppError::new_plain("Invalid response format"))?,
    };
    
    let messages = points.iter().filter_map(|point| {
        let id = point["id"].as_str()?;
        let payload = &point["payload"];
        
        let session_id = payload["session_id"].as_str()?;
        let sender_id = payload["sender_id"].as_str()?;
        let message_text = payload["message"].as_str()?;
        let timestamp = payload["timestamp"].as_i64()?;
        
        // Parse message type
        let message_type_str = payload["message_type"].as_str()?;
        let message_type: MessageType = serde_json::from_str(message_type_str).ok()?;
        
        // Parse read_by array
        let read_by = match payload["read_by"].as_array() {
            Some(array) => array.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            None => vec![]
        };
        
        Some(ChatMessage {
            id: id.to_string(),
            session_id: session_id.to_string(),
            sender_id: sender_id.to_string(),
            message: message_text.to_string(),
            timestamp,
            message_type,
            embedding: None, // We don't return the embedding to save bandwidth
            read_by,
        })
    }).collect();
    
    Ok(messages)
} 