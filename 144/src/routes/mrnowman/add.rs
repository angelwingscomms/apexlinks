use serde_json::json;

use crate::{
    constants::MR_NOWMAN_MESSAGE,
    util::qdrant::{qdrant_path, qdrant_put}, 
    util::{embed, id, AppResult},
};

#[derive(serde::Deserialize)]
pub struct Add {
    /// assistant message
    pub a: String,
    /// client (user) message
    pub c: String,
    /// assistant date
    pub ad: String,
    /// user date
    pub ud: String,
    /// chat if
    pub i: String,
    /// user
    pub u: Option<String>,
}

/// saves message to db
pub async fn add(s: Add, addr: Option<std::net::SocketAddr>) -> AppResult<()> {
    // save user message
    if let Some(a) = addr {
        let sum_a_res = qdrant_put(
            &qdrant_path("collections/i/points?wait=true").await?,
            json!({"points":[{"id":id(), "payload": {"c": 1, "a": a.ip().to_string(), "m": s.c, "c": MR_NOWMAN_MESSAGE, "i": s.i, "d": s.ud}, "vector": embed(s.c).await? }]}),
        ).await?;
        println!("sum_a_res: {}", sum_a_res);
    } else {
        let sum_res = qdrant_put(
            &qdrant_path("collections/i/points?wait=true").await?,
            json!({"points":[{"id":id(), "payload": {"c": 1, "m": s.c, "c": MR_NOWMAN_MESSAGE, "i": s.i, "d": s.ud}, "vector": embed(s.c).await? }]}),
        ).await?;
        println!("sum_res: {}", sum_res);
    }

    // save assistant message
    qdrant_put(
        &qdrant_path("collections/i/points").await?,
        json!({"points":[{"id":id(), "payload": {"u": 0, "m": s.a, "c": MR_NOWMAN_MESSAGE, "i": s.i, "d": s.ad}, "vector": embed(s.a).await? }]}),
    ).await?;
    Ok(())
}
