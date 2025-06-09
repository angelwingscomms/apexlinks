use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub description: String,
    pub interests: Vec<String>,
    pub age_range: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchRequest {
    pub description: String,
    pub interests: Vec<String>,
    pub age_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchResponse {
    pub match_found: bool,
    pub session_id: Option<String>,
    pub partner_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub session_id: String,
    pub sender_id: String,
    pub message: String,
    pub timestamp: i64,
    pub message_type: MessageType,
    pub embedding: Option<Vec<f32>>,
    pub id: String,
    pub read_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    System,
    UserDisconnected,
    UserConnected,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub id: String,
    pub user1_id: String,
    pub user2_id: String,
    pub created_at: i64,
    pub active: bool,
}

impl UserProfile {
    pub fn new(description: String, interests: Vec<String>, age_range: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            description,
            interests,
            age_range,
            embedding: None,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

impl ChatSession {
    pub fn new(user1_id: String, user2_id: String) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            user1_id,
            user2_id,
            created_at: chrono::Utc::now().timestamp(),
            active: true,
        }
    }
}

impl ChatMessage {
    pub fn new(
        session_id: String, 
        sender_id: String, 
        message: String, 
        message_type: MessageType
    ) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            session_id,
            sender_id: sender_id.clone(),
            message,
            timestamp: chrono::Utc::now().timestamp(),
            message_type,
            embedding: None,
            read_by: vec![sender_id], // Sender has read their own message
        }
    }
} 