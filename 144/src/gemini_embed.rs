use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::{timeout, Duration};
use crate::util::{AppResult, AppError, embed, id};
use crate::util::qdrant::{qdrant_path, qdrant_put};
use crate::constants::{COLLECTION, BATCH_SIZE_BIBLE, QDRANT_TIMEOUT_SECS};
use once_cell::sync::Lazy;
use std::sync::Arc;
use reqwest::Client;

// Reusable HTTP client
static HTTP_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

// Special ID for tracking embedding progress in Qdrant
const EMBEDDING_PROGRESS_ID: &str = "embedding_progress_v1";

/// Structure for Bible verse from ylt.json
#[derive(Debug, Serialize, Deserialize)]
pub struct BibleVerse {
    pub b: u32,     // book
    pub c: u32,     // chapter
    pub v: u32,     // verse
    pub t: String,  // text
}

/// Metadata to be stored alongside the embedding
#[derive(Debug, Serialize, Deserialize)]
pub struct VerseMetadata {
    pub book: u32,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
    pub reference: String,
    pub s: String,  // tenant/sort identifier for multitenancy (always "v" for verses)
}

/// Embedding progress tracking stored in Qdrant
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingProgress {
    pub last_book: u32,
    pub last_chapter: u32,
    pub last_verse: u32,
    pub total_processed: usize,
    pub last_updated: String,
    pub s: String,  // Always "progress" for tracking
}

/// Validate collection name against whitelist
fn validate_collection(name: &str) -> AppResult<()> {
    const ALLOWED_COLLECTIONS: &[&str] = &["i", "r", "a", "bible_verses", "users", "chat"];
    if !ALLOWED_COLLECTIONS.contains(&name) {
        return Err(AppError::new_plain("Invalid collection name"));
    }
    Ok(())
}

/// Sanitize text input
fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?-'\"".contains(*c))
        .take(10000)
        .collect()
}

/// Get embedding progress from Qdrant
async fn get_embedding_progress() -> AppResult<Option<EmbeddingProgress>> {
    // First validate that the collection exists
    validate_collection(COLLECTION)?;
    
    let url = qdrant_path(&format!("collections/{}/points/{}", COLLECTION, EMBEDDING_PROGRESS_ID)).await?;
    
    let response = timeout(
        Duration::from_secs(QDRANT_TIMEOUT_SECS),
        HTTP_CLIENT
            .get(&url)
            .header(
                "api-key",
                crate::constants::SECRETS
                    .lock()
                    .await
                    .get("QDRANT_KEY")
                    .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?,
            )
            .send()
    ).await
    .map_err(|_| AppError::new_plain("Qdrant get progress request timed out"))?
    .map_err(|e| AppError::new("failed to get embedding progress from Qdrant", e))?;
    
    let status_code = response.status().as_u16();
    
    // Handle different status codes appropriately
    if status_code == 404 {
        return Ok(None); // No progress found
    }
    
    if status_code == 400 {
        // 400 could mean the collection doesn't exist or the point ID format is invalid
        log::warn!("Bad request when fetching embedding progress. Collection might not exist or point ID format is invalid.");
        return Ok(None);
    }
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(AppError::new_plain(&format!(
            "Failed to get embedding progress: {} - {}", 
            status,
            error_text
        )));
    }
    
    let response_json = response
        .json::<Value>()
        .await
        .map_err(|e| AppError::new("failed to parse Qdrant progress response", e))?;
    
    if let Some(payload) = response_json.get("result").and_then(|r| r.get("payload")) {
        match serde_json::from_value::<EmbeddingProgress>(payload.clone()) {
            Ok(progress) => Ok(Some(progress)),
            Err(e) => {
                log::warn!("Failed to deserialize embedding progress: {}", e);
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

/// Save embedding progress to Qdrant
async fn save_embedding_progress(progress: &EmbeddingProgress) -> AppResult<()> {
    let point = json!({
        "id": EMBEDDING_PROGRESS_ID,
        "vector": vec![0.0; 768], // Dummy vector for progress tracking (768 dimensions for Gemini)
        "payload": progress
    });
    
    timeout(
        Duration::from_secs(QDRANT_TIMEOUT_SECS),
        qdrant_put(
            &qdrant_path(&format!("collections/{}/points?wait=true", COLLECTION)).await?,
            json!({
                "points": [point]
            }),
        )
    ).await
    .map_err(|_| AppError::new_plain("Qdrant save progress timed out"))?
    .map_err(|e| AppError::new("failed to save embedding progress to Qdrant", e))?;
    
    Ok(())
}

/// Reset embedding progress (delete from Qdrant)
pub async fn reset_embedding_progress() -> AppResult<()> {
    validate_collection(COLLECTION)?;
    
    let url = qdrant_path(&format!("collections/{}/points/delete", COLLECTION)).await?;
    
    let response = timeout(
        Duration::from_secs(QDRANT_TIMEOUT_SECS),
        HTTP_CLIENT
            .post(&url)
            .header(
                "api-key",
                crate::constants::SECRETS
                    .lock()
                    .await
                    .get("QDRANT_KEY")
                    .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?,
            )
            .header("Content-Type", "application/json")
            .json(&json!({
                "points": [EMBEDDING_PROGRESS_ID]
            }))
            .send()
    ).await
    .map_err(|_| AppError::new_plain("Qdrant reset progress request timed out"))?
    .map_err(|e| AppError::new("failed to reset embedding progress", e))?;
    
    if !response.status().is_success() && response.status().as_u16() != 404 {
        return Err(AppError::new_plain(&format!(
            "Failed to reset embedding progress: {}", 
            response.status()
        )));
    }
    
    log::info!("Embedding progress reset successfully");
    Ok(())
}

/// Process verses from ylt.json, create embeddings, and store in Qdrant
pub async fn process_bible_verses(file_path: &str) -> AppResult<()> {
    // Validate inputs
    if file_path.is_empty() || file_path.len() > 500 {
        return Err(AppError::new_plain("Invalid file path"));
    }

    // Read and parse the JSON file asynchronously
    let mut file = File::open(file_path)
        .await
        .map_err(|e| AppError::new(&format!("failed to open Bible verses file at {}", file_path), e))?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .map_err(|e| AppError::new("failed to read Bible verses file", e))?;
    
    let verses: Vec<BibleVerse> = serde_json::from_str(&contents)
        .map_err(|e| AppError::new("failed to parse Bible verses from JSON", e))?;

    // Validate collection name
    validate_collection(COLLECTION)?;

    // Get existing progress from Qdrant
    let mut last_processed_index = 0;
    let mut total_processed = 0;
    
    if let Some(progress) = get_embedding_progress().await? {
        // Find the index of the last processed verse
        for (i, verse) in verses.iter().enumerate() {
            if verse.b > progress.last_book || 
               (verse.b == progress.last_book && verse.c > progress.last_chapter) ||
               (verse.b == progress.last_book && verse.c == progress.last_chapter && verse.v > progress.last_verse) {
                last_processed_index = i;
                break;
            }
        }
        total_processed = progress.total_processed;
        log::info!("Resuming Bible verse processing from verse after {} {}:{} (total processed: {})", 
            progress.last_book, progress.last_chapter, progress.last_verse, total_processed);
    }
    
    // Get the verses that need to be processed
    let verses_to_process = &verses[last_processed_index..];
    log::info!("Processing {} verses out of {} total verses", verses_to_process.len(), verses.len());

    // Process verses in batches to avoid rate limiting
    for (batch_index, chunk) in verses_to_process.chunks(BATCH_SIZE_BIBLE).enumerate() {
        let mut points = Vec::new();
        let mut last_verse = None;

        for verse in chunk {
            // Create a readable reference
            let reference = format!("{} {}:{}", verse.b, verse.c, verse.v);
            log::info!("Processing Bible verse: {}", reference);

            // Sanitize text before embedding
            let sanitized_text = sanitize_input(&verse.t);

            // Generate embedding using the embed utility with timeout
            let embedding = timeout(
                Duration::from_secs(30),
                embed(sanitized_text.clone())
            ).await
            .map_err(|_| AppError::new_plain("Embedding generation timed out"))?
            .map_err(|e| AppError::new("failed to generate embedding for verse", e))?;

            // Create metadata with multitenancy support (s="v" for verses)
            let metadata = VerseMetadata {
                book: verse.b,
                chapter: verse.c,
                verse: verse.v,
                text: sanitized_text,
                reference: reference.clone(),
                s: "v".to_string(),  // Use "v" for verses in multitenancy
            };

            // Create Qdrant point with UUID v7
            let point_id = id();
            points.push(json!({
                "id": point_id,
                "vector": embedding,
                "payload": metadata
            }));
            
            // Update last processed verse
            last_verse = Some((verse.b, verse.c, verse.v));
            total_processed += 1;
        }

        // Add batch to Qdrant with timeout
        let _add_result = timeout(
            Duration::from_secs(QDRANT_TIMEOUT_SECS),
            qdrant_put(
                &qdrant_path(&format!("collections/{}/points?wait=true", COLLECTION)).await?,
                json!({
                    "points": points,
                    "ordering": "weak"  // For better performance
                }),
            )
        ).await
        .map_err(|_| AppError::new_plain("Qdrant batch upload timed out"))?
        .map_err(|e| AppError::new(&format!("failed to upload batch {} to Qdrant", batch_index), e))?;

        log::info!("Successfully uploaded batch {} with {} verses", batch_index, chunk.len());
        
        // After successful batch upload, save the progress to Qdrant
        if let Some((book, chapter, verse)) = last_verse {
            let progress = EmbeddingProgress {
                last_book: book,
                last_chapter: chapter,
                last_verse: verse,
                total_processed,
                last_updated: chrono::Utc::now().to_rfc3339(),
                s: "progress".to_string(),
            };
            
            save_embedding_progress(&progress).await?;
            log::info!("Saved embedding progress: {} {}:{} (total: {})", book, chapter, verse, total_processed);
        }
        
        // Rate limiting between batches
        if batch_index < verses_to_process.len() / BATCH_SIZE_BIBLE - 1 {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    log::info!("Bible verse embedding completed! Total processed: {}", total_processed);
    Ok(())
}

/// Get embedding status
pub async fn get_embedding_status() -> AppResult<Option<EmbeddingProgress>> {
    validate_collection(COLLECTION)?;
    get_embedding_progress().await
}

/// Search for similar verses using semantic search
pub async fn search_similar_verses(query: &str, limit: usize) -> AppResult<Vec<VerseMetadata>> {
    // Validate inputs
    if query.is_empty() || query.len() > 1000 {
        return Err(AppError::new_plain("Invalid query length: must be between 1 and 1000 characters"));
    }
    
    if limit == 0 || limit > 100 {
        return Err(AppError::new_plain("Invalid limit: must be between 1 and 100"));
    }
    
    // Validate collection
    validate_collection(COLLECTION)?;
    
    // Sanitize query
    let sanitized_query = sanitize_input(query);

    // Generate embedding for the query with timeout
    let query_embedding = timeout(
        Duration::from_secs(30),
        embed(sanitized_query)
    ).await
    .map_err(|_| AppError::new_plain("Query embedding generation timed out"))?
    .map_err(|e| AppError::new("failed to generate embedding for search query", e))?;
    
    // Build search filter for verses (s="v")
    let filter = json!({
        "must": [
            {
                "key": "s",
                "match": { "value": "v" }
            }
        ]
    });
    
    // Search Qdrant collection with timeout
    let search_url = qdrant_path(&format!("collections/{}/points/search", COLLECTION)).await?;
    
    let response = timeout(
        Duration::from_secs(QDRANT_TIMEOUT_SECS),
        HTTP_CLIENT
            .post(&search_url)
            .header(
                "api-key",
                crate::constants::SECRETS
                    .lock()
                    .await
                    .get("QDRANT_KEY")
                    .ok_or_else(|| AppError::new_plain("QDRANT_KEY not found in secrets"))?,
            )
            .header("Content-Type", "application/json")
            .json(&json!({
                "vector": query_embedding,
                "limit": limit,
                "with_payload": true,
                "filter": filter
            }))
            .send()
    ).await
    .map_err(|_| AppError::new_plain("Qdrant search request timed out"))?
    .map_err(|e| AppError::new("failed to send search request to Qdrant", e))?;
    
    // Check response status
    if !response.status().is_success() {
        return Err(AppError::new_plain(&format!(
            "Qdrant search failed with status: {}", 
            response.status()
        )));
    }
    
    let response_json = response
        .json::<Value>()
        .await
        .map_err(|e| AppError::new("failed to parse Qdrant search response", e))?;
    
    // Extract results with proper error handling
    let results = response_json
        .get("result")
        .and_then(|r| r.as_array())
        .ok_or_else(|| AppError::new_plain("Invalid response format from Qdrant search"))?;
    
    // Convert to VerseMetadata with error handling
    let verses = results
        .iter()
        .filter_map(|point| {
            let payload = point.get("payload")?;
            match serde_json::from_value::<VerseMetadata>(payload.clone()) {
                Ok(verse) => Some(verse),
                Err(e) => {
                    log::warn!("Failed to deserialize verse metadata: {}", e);
                    None
                }
            }
        })
        .collect::<Vec<VerseMetadata>>();
    
    log::info!("Found {} similar verses for query", verses.len());
    
    Ok(verses)
} 