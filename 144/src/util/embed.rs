use reqwest::Client;
use serde_json::json;

use super::{AppError, AppResult};
use crate::constants::SECRETS;

/// Main embed function - uses 768-dimensional embeddings by default
pub async fn embed(text: String) -> AppResult<Vec<f32>> {
    embed_768(text).await
}

/// Original Gemini API embedding with 768 dimensions
pub async fn embed_768(text: String) -> AppResult<Vec<f32>> {
    let api_key = SECRETS
        .lock()
        .await
        .get("GOOGLE")
        .ok_or_else(|| AppError::new_plain("GOOGLE API key not found in secrets"))?;

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/embedding-001:embedContent?key={}",
        api_key
    );

    let request_body = json!({
        "model": "models/embedding-001",
        "content": {
            "parts": [
                { "text": text }
            ]
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| AppError::new("sending embedding request to Gemini", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::new_plain(&format!(
            "Gemini API error: {}",
            error_text
        )));
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::new("parsing Gemini embedding response", e))?;

    let embedding = response_json["embedding"]["values"]
        .as_array()
        .ok_or_else(|| AppError::new_plain("Failed to extract embedding from response"))?
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect::<Vec<f32>>();

    Ok(embedding)
}

/// New Vertex AI embedding with 3072 dimensions
pub async fn embed_3072(text: String) -> AppResult<Vec<f32>> {
    let token = SECRETS
        .lock()
        .await
        .get("GOOGLE_TOKEN")
        .ok_or_else(|| AppError::new_plain("GOOGLE_TOKEN not found in secrets"))?;

    let project_id = SECRETS
        .lock()
        .await
        .get("GOOGLE_PROJECT_ID")
        .ok_or_else(|| AppError::new_plain("GOOGLE_PROJECT_ID not found in secrets"))?;

    let client = Client::new();
    let url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/us-central1/publishers/google/models/gemini-embedding-001:predict",
        project_id
    );

    let request_body = json!({
        "instances": [
            { "content": text }
        ],
        "parameters": {
            "outputDimensionality": 3072
        }
    });

    let response = client
        .post(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| AppError::new("sending embedding request to Vertex AI", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AppError::new_plain(&format!(
            "Vertex AI API error: {}",
            error_text
        )));
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| AppError::new("parsing Vertex AI embedding response", e))?;

    let embedding = response_json["predictions"][0]["embeddings"]["values"]
        .as_array()
        .ok_or_else(|| AppError::new_plain("Failed to extract embedding from response"))?
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect::<Vec<f32>>();

    Ok(embedding)
}