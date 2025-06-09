use serde_json::json;

use crate::{
    util::{AppError, AppResult},
    constants::SECRETS,
};

#[derive(serde::Serialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    /// creates a new message instance from a message qdrant point
    pub async fn new(x: &serde_json::Value) -> AppResult<Self> {
        Ok(Message {
            role: {
                if x["payload"]["u"]
                    .as_i64()
                    .ok_or(AppError::new_plain(&format!(
                        "converting field 'u' to i64 for {:#?}",
                        x
                    )))?
                    == 1
                {
                    "user".to_string()
                } else {
                    "assistant".to_string()
                }
            },
            content: x["payload"]["m"]
                .as_str()
                .ok_or(AppError::new_plain(&format!(
                    "converting field 'm' to string for {:#?}",
                    x
                )))?
                .to_string(),
        })
    }
}

pub async fn groq(messages: Vec<Message>) -> AppResult<String> {
    Ok(reqwest::Client::new()
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                SECRETS
                    .lock()
                    .await
                    .get("GROQ")
                    .ok_or("GROQ not found in env")
                    .map_err(|e| AppError::new_plain(e))?
            ),
        )
        .header("Content-Type", "application/json")
        // get messages
        // map
        //  .a to role assistant
        //  .u to role user
        .json(&json!({"model": "llama-3.1-70b-versatile", "messages": messages}))
        .send()
        .await
        .map_err(|e| AppError::new("upsert_points", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| AppError::new("qdrant response to json", e))?["choices"][0]["message"]
        ["content"]
        .as_str()
        .ok_or(AppError::new_plain("groq response content not string"))?
        .to_string())
}