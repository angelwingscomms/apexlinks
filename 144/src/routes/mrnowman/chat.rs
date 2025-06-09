use serde_json::json;
use warp::reply::Reply;

use crate::{
    constants::{MR_NOWMAN_CHAT, MR_NOWMAN_MESSAGE},
    util::qdrant::{qdrant_path, qdrant_post},
    util::{embed, AppResult},
};

#[derive(serde::Deserialize, Debug)]
pub struct Chat {
    i: Option<i64>,
    f: Option<i64>,    // page
    s: Option<i64>,    //start date
    e: Option<i64>,    //end date
    a: Option<String>, //ip address
    l: Option<i64>,    //limit
    q: Option<String>, //search query
}

pub async fn chat(q: Chat) -> impl Reply {
    f(q).await.map_or_else(
        |e| {
            log::error!("{:#?}", e);
            warp::reply::with_status(
                "An error occured on our side".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        },
        |v| warp::reply::with_status(v, warp::http::StatusCode::OK),
    )
}

pub async fn f(q: Chat) -> AppResult<String> {
    let mut body = serde_json::Map::new();
    let mut must = vec![];
    let mut date = serde_json::Map::new();

    if let Some(s) = q.s {
        date.insert("gt".into(), s.into());
    }
    if let Some(e) = q.e {
        date.insert("lt".into(), e.into());
    }
    if !date.is_empty() {
        must.push(json!({"key": "d", "range": date}));
    }
    if let Some(a) = q.a {
        must.push(json!({"key": "a", "match": {"value": a}}));
    }
    if let Some(q) = q.q {
        body.insert("query".into(), embed(q).await?.into());
    }
    if let Some(f) = q.f {
        if f > 1 {
            body.insert("offset".into(), { (f - 1) * 7 }.into());
        }
    }
    if let Some(i) = q.i {
        must.push(json!({"key": "i", "match": {"value": i}}));
        must.push(json!({"key": "c", "match": {"value": MR_NOWMAN_MESSAGE}}));
    } else {
        must.push(json!({"key": "c", "match": {"value": MR_NOWMAN_CHAT}}));
    }
    body.insert("filter".into(), json!({"must": must}));
    body.insert("limit".into(), q.l.unwrap_or(7).into());
    body.insert("with_payload".into(), vec!["d", "m", "u"].into());
    let res = qdrant_post(&qdrant_path("collections/i/points/query").await?, body).await?;
    Ok(res["result"]["points"].to_string())
}
