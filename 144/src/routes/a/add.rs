use serde_json::json;
// use tokio::sync::mpsc::Sender;
use warp::reply::Reply;

use crate::{
    constants::A,
    util::qdrant::{qdrant_path, qdrant_put},
    util::AppResult,
    util::{embed, id},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddRequest {
    pub i: Option<String>,
    pub c: Option<i32>,
    pub t: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Add {
    /// text
    pub t: String,
    /// cost
    pub c: Option<i32>,
    /// date
    pub d: String,
    /// sort (for multitenancy)
    pub s: String,
    /// parent id
    pub i: Option<String>,
}

impl From<AddRequest> for Add {
    fn from(value: AddRequest) -> Self {
        Self {
            i: value.i,
            t: value.t,
            c: value.c,
            d: chrono::Utc::now().to_rfc3339().into(),
            // point differentiator for multitenancy, pattern to be used for different kinds of points
            s: A.to_string(),
        }
    }
}

pub async fn r(s: AddRequest /*a: Option<std::net::SocketAddr> */) -> impl Reply {
    f(s).await.map_or_else(
        |e| {
            log::error!("{:#?}", e);
            warp::reply::with_status(
                warp::reply::json(&"An error occured on our side".to_string()),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        },
        |v| warp::reply::with_status(warp::reply::json(&v), warp::http::StatusCode::OK),
    )
}

pub async fn f(s: AddRequest /*_addr: Option<std::net::SocketAddr> */) -> AppResult<String> {
    let id = id();
    let s: Add = s.into();
    let vector = embed(s.t.clone()).await?;
    // let vector2 = embed("dlfkjsldfkjslkfsljfs").await?;
    println!("{:#?}", vector);
    let add_res = qdrant_put(
        &qdrant_path("collections/i/points?wait=true").await?,
        json!({"points":[{"id":id, "payload": s, "vector": vector }]}),
    )
    .await?;
    println!("add_res: {}", add_res);
    Ok(id)
}
