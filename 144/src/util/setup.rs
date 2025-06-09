use serde_json::json;

use crate::util::qdrant::{qdrant_path, qdrant_put};

use super::AppResult;

pub async fn setup() -> AppResult<()> {
    qdrant_put(
        &qdrant_path("collections/i?wait=true").await?,
        json!({"vectors": {"size": 768, "distance": "Cosine"}}),
    ).await?;
    qdrant_put(
        &qdrant_path("collections/r?wait=true").await?,
        json!({"vectors": {"size": 1, "distance": "Cosine"}}),
    ).await?;
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
    Ok(())
}
