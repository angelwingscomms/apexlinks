use std::env;
use i144::gemini_embed::process_bible_verses;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Path to the YLT JSON file
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "ylt.json".to_string());

    println!("Starting Bible verse embedding process...");
    println!("Reading from file: {}", file_path);
    println!("Using multitenancy identifier: v (verses)");

    // Process the Bible verses
    process_bible_verses(&file_path).await?;

    println!("Bible verse embedding completed successfully!");

    Ok(())
} 