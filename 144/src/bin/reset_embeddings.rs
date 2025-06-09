use i144::gemini_embed::reset_embedding_progress;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    println!("Resetting Bible verse embedding progress...");

    // Reset the embedding progress
    reset_embedding_progress().await?;

    println!("Embedding progress reset successfully!");

    Ok(())
} 