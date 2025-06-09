use super::types::{ChatMessage, MessageType};
use super::matching::ACTIVE_SESSIONS;
use super::storage;
use warp::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json;

// Global state for managing WebSocket connections
lazy_static::lazy_static! {
    static ref CONNECTIONS: Arc<Mutex<HashMap<String, Arc<Mutex<futures_util::stream::SplitSink<WebSocket, Message>>>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

pub async fn handle_websocket(websocket: WebSocket) {
    let (ws_sender, mut ws_receiver) = websocket.split();
    let connection_id = Uuid::now_v7().to_string();
    
    // Store the sender half for this connection
    let sender_arc = Arc::new(Mutex::new(ws_sender));
    {
        let mut connections = CONNECTIONS.lock().await;
        connections.insert(connection_id.clone(), sender_arc.clone());
    }
    
    // Handle incoming messages
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    if let Err(e) = handle_message(&connection_id, text).await {
                        eprintln!("Error handling message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    // Clean up connection
    {
        let mut connections = CONNECTIONS.lock().await;
        connections.remove(&connection_id);
    }
}

async fn handle_message(connection_id: &str, message_text: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Parse the incoming message
    let message: serde_json::Value = serde_json::from_str(message_text)?;
    
    match message["type"].as_str() {
        Some("chat") => {
            let session_id = message["session_id"].as_str()
                .ok_or("Missing session_id")?;
            let user_id = message["user_id"].as_str()
                .ok_or("Missing user_id")?;
            let text = message["message"].as_str()
                .ok_or("Missing message")?;
            
            // Verify the session exists and the user is part of it
            let sessions = ACTIVE_SESSIONS.lock().await;
            if let Some(session) = sessions.get(session_id) {
                if session.user1_id == user_id || session.user2_id == user_id {
                    let partner_id = if session.user1_id == user_id {
                        &session.user2_id
                    } else {
                        &session.user1_id
                    };
                    
                    // Create chat message
                    let chat_message = ChatMessage::new(
                        session_id.to_string(),
                        user_id.to_string(),
                        text.to_string(),
                        MessageType::Text,
                    );
                    
                    // Save message with embedding in the background
                    let chat_message_clone = chat_message.clone();
                    tokio::spawn(async move {
                        if let Err(e) = storage::save_message(chat_message_clone).await {
                            eprintln!("Error saving message: {:?}", e);
                        }
                    });
                    
                    // Send to both users in the session
                    broadcast_to_session(&session.id, &chat_message).await?;
                }
            }
        }
        Some("join_session") => {
            let session_id = message["session_id"].as_str()
                .ok_or("Missing session_id")?;
            let user_id = message["user_id"].as_str()
                .ok_or("Missing user_id")?;
            
            // Notify the session that a user has joined
            let join_message = ChatMessage::new(
                session_id.to_string(),
                "system".to_string(),
                format!("User {} has joined the chat", user_id),
                MessageType::UserConnected,
            );
            
            // Save system message
            let join_message_clone = join_message.clone();
            tokio::spawn(async move {
                if let Err(e) = storage::save_message(join_message_clone).await {
                    eprintln!("Error saving join message: {:?}", e);
                }
            });
            
            broadcast_to_session(session_id, &join_message).await?;
        }
        Some("leave_session") => {
            let session_id = message["session_id"].as_str()
                .ok_or("Missing session_id")?;
            let user_id = message["user_id"].as_str()
                .ok_or("Missing user_id")?;
            
            // Notify the session that a user has left
            let leave_message = ChatMessage::new(
                session_id.to_string(),
                "system".to_string(),
                format!("User {} has left the chat", user_id),
                MessageType::UserDisconnected,
            );
            
            // Save system message
            let leave_message_clone = leave_message.clone();
            tokio::spawn(async move {
                if let Err(e) = storage::save_message(leave_message_clone).await {
                    eprintln!("Error saving leave message: {:?}", e);
                }
            });
            
            broadcast_to_session(session_id, &leave_message).await?;
        }
        _ => {
            eprintln!("Unknown message type: {}", message_text);
        }
    }
    
    Ok(())
}

async fn broadcast_to_session(session_id: &str, message: &ChatMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let message_json = serde_json::to_string(message)?;
    let ws_message = Message::text(message_json);
    
    let connections = CONNECTIONS.lock().await;
    
    // In a real implementation, you'd maintain a mapping of user_id to connection_id
    // For now, we'll broadcast to all connections (simplified)
    for (_, sender) in connections.iter() {
        let mut sender_guard = sender.lock().await;
        if let Err(e) = sender_guard.send(ws_message.clone()).await {
            eprintln!("Error sending message: {}", e);
        }
    }
    
    Ok(())
} 