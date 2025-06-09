use serde_json::json;
use warp::reply::Reply;

use crate::{
    util::AppResult, constants::REAL, util::qdrant::{qdrant_path, qdrant_put}, util::{embed, id}
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct AddRequest {
    pub t: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Add {
    pub t: String,
    pub d: String,
    pub s: String
}

impl From<AddRequest> for Add {
    fn from(value: AddRequest) -> Self {
        Self {
            t: value.t,
            d: chrono::Utc::now().to_rfc3339().into(),
            s: REAL.to_string()
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
    let add_res = qdrant_put(
            &qdrant_path("collections/i/points?wait=true").await?,
            json!({"points":[{"id":id, "payload": s, "vector": embed(s.t).await? }]}),
        ).await?;
    println!("add_res: {}", add_res);
    Ok(id)
}
