use crate::constants::{/*I_ID,*/ SECRETS};
use thiserror::Error;
use uuid::{NoContext, Timestamp};
pub type AppResult<T> = Result<T, AppError>;

pub mod embed;
pub use embed::embed;

#[derive(Error, Debug)]
pub struct AppError {
    t: String,
}

impl warp::reject::Reject for AppError {}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.t)
    }
}
impl AppError {
    pub fn new(m: &str, e: impl std::error::Error) -> Self {
        AppError {
            t: format!("{}: {}", m, e.to_string()),
        }
    }

    pub fn new_plain(m: &str) -> Self {
        AppError { t: m.to_string() }
    }
    
    pub fn new_rejection(m: &str) -> Self {
        Self::new_plain(m)
    }
}

pub mod groq;
pub mod setup;
pub mod qdrant;
pub mod password;

// use crate::util::qdrant::{qdrant_path, qdrant_post};

/// returns and increments next qdrant id
// pub async fn id() -> AppResult<i64> {
//     let res = qdrant_post(
//         &qdrant_path("collections/r/points").await?,
//         json!({"ids": [I_ID], "with_payload": ["i"]}),
//     )
//     .await?;
//     println!("id res: {}", res);
//     let id = res["result"][0]["payload"]["i"]
//         .as_i64()
//         .ok_or(AppError::new_plain("no previous id in id()"))?;
//     let next = id + 1;
//     qdrant_post(
//         &qdrant_path("collections/r/points/payload?wait=true").await?,
//         json!({"payload": {"i": next}, "points": [I_ID]}),
//     )
//     .await?;
//     println!(
//         "new_id: {}",
//         qdrant_post(
//             &qdrant_path("collections/r/points").await?,
//             json!({"ids": [I_ID], "with_payload": ["i"]}),
//         )
//         .await?
//     );
//     println!("id: {}, next: {}", id, next);
//     Ok(id)
// }

pub fn id() -> String {
    uuid::Uuid::new_v7(Timestamp::now(NoContext)).to_string()
}

pub async fn embedding(query: String) -> AppResult<serde_json::Value> {
    let embedding_vec = embed::embed_768(query).await?;
    
    let embedding_json = serde_json::to_value(embedding_vec)
        .map_err(|e| AppError::new("converting embedding to JSON", e))?;
    
    Ok(embedding_json)
}

use warp::Filter;

/// Filter that extracts the user ID from the Authorization header
pub fn with_auth() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization")
        .map(|token: String| {
            // Simple token format: "Bearer user_id"
            token.strip_prefix("Bearer ")
                .unwrap_or(&token)
                .to_string()
        })
}
