use std::env;
use i144::gemini_embed::search_similar_verses;
use i144::util::AppError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Get search query from command line arguments
    let query = env::args()
        .nth(1)
        .ok_or_else(|| AppError::new_plain("Please provide a search query as the first argument"))?;

    // Get number of results to return (default: 5)
    let limit = env::args()
        .nth(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(5);

    println!("Searching for verses similar to: \"{}\"", query);
    println!("Requesting {} results", limit);
    println!("Using multitenancy identifier: v (verses)");

    // Perform the search
    let results = search_similar_verses(&query, limit).await?;

    // Display results
    println!("\nSearch Results:");
    println!("===============");

    if results.is_empty() {
        println!("No results found.");
    } else {
        for (i, verse) in results.iter().enumerate() {
            println!("{}. {} - {}", i+1, verse.reference, verse.text);
        }
    }

    Ok(())
} 