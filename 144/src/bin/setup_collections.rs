use i144::util::qdrant::{qdrant_path, qdrant_put};
use i144::util::{AppError, AppResult};
use serde_json::json;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("Setting up Qdrant collections with correct vector dimensions...");

    // Delete existing collection i if it exists
    if let Err(e) = delete_collection("i").await {
        println!("Note: Could not delete collection i (may not exist): {}", e);
    }

    // Recreate collection i with correct vector size (768 for Gemini embeddings)
    println!("Creating collection 'i' with 768 dimensions...");
    qdrant_put(
        &qdrant_path("collections/i?wait=true").await?,
        json!({"vectors": {"size": 768, "distance": "Cosine"}}),
    ).await?;

    // Create a payload index for the 's' field to enable filtering
    println!("Creating payload index for 's' field...");
    qdrant_put(
        &qdrant_path("collections/i/index?wait=true").await?,
        json!({
            "field_name": "s",
            "field_schema": "keyword"
        }),
    ).await?;

    // Ensure collection r exists
    println!("Creating collection 'r' for ID tracking...");
    qdrant_put(
        &qdrant_path("collections/r?wait=true").await?,
        json!({"vectors": {"size": 1, "distance": "Cosine"}}),
    ).await?;

    // Add initial point to collection r
    qdrant_put(
        &qdrant_path("collections/r/points?wait=true").await?,
        json!({"points": [{
            "id": "b4ea369a-d21e-40b4-afe7-4e84a4a7cd91",
            "payload": {
                "i": 0
            },
            "vector": [0]
        }]}),
    ).await?;

    println!("Collections setup completed successfully!");
    println!("Collection 'i': 768-dimensional vectors for embeddings");
    println!("Collection 'r': 1-dimensional vectors for ID tracking");

    Ok(())
}

async fn delete_collection(collection_name: &str) -> AppResult<()> {
    let url = qdrant_path(&format!("collections/{}", collection_name)).await?;
    
    let response = Client::new()
        .delete(&url)
        .header(
            "api-key",
            i144::constants::SECRETS
                .lock()
                .await
                .get("QDRANT_KEY")
                .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?,
        )
        .send()
        .await
        .map_err(|e| AppError::new("failed to delete collection", e))?;

    if !response.status().is_success() && response.status().as_u16() != 404 {
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Failed to delete collection {}: {}", 
            collection_name,
            error_text
        )));
    }

    Ok(())
} 