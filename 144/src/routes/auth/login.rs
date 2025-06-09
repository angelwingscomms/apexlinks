use warp::reply::Reply;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    util::{AppError, AppResult, qdrant::{qdrant_path, qdrant_post}},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub q: Option<String>,
}

pub async fn r(q: Login) -> impl Reply {
    println!("search r, q: {:#?}", q);
    if let Some(q) = &q.q {
        if q.len() > 144 {
            return warp::reply::with_status(
                "search query too long".to_string(),
                warp::http::StatusCode::BAD_REQUEST,
            );
        }
    }
    let a = q.q.unwrap_or_default();
    f(a).await.map_or_else(
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

pub async fn f(a: String) -> AppResult<String> {
    let mut body = serde_json::Map::new();
    let mut must: Vec<serde_json::Value> = vec![];
    must.push(json!({"key": "a", "match": {"value": a}}));
    // let mut date = serde_json::Map::new();

    /*    if let Some(s) = q.s {
        date.insert("gt".into(), s.into());
    }
    if let Some(e) = q.e {
        date.insert("lt".into(), e.into());
    }
    if !date.is_empty() {
        must.push(json!({"key": "d", "range": date}));
    }
    if let Some(a) = q.a {
    } */

    // if let Some(i) = q.i {
    //     must.push(json!({"key": "i", "match": {"value": i}}));
    //     must.push(json!({"key": "c", "match": {"value": CUSTOMER_SERVICE_AI_CHAT_MESSAGE}}));
    // } else {
    // must.push(json!({"key": "c", "match": {"value": AI_CHAT_BOT}}));
    // }
    // body.insert("filter".into(), json!({"must": must}));
    body.insert("limit".into(), 1.into());
    body.insert("with_payload".into(), vec!["p"].into());
    let res = qdrant_post(&qdrant_path("collections/i/points/query").await?, body).await?;
    println!("search res: {:#?}", res);
    let mut results = res["result"]["points"]
        .as_array()
        .ok_or(AppError::new_plain("points as array"))?.to_owned();
    let e = results
        .iter_mut()
        .map(|v| v["payload"].clone())
        .collect::<Vec<serde_json::Value>>();
    println!("search done");
    Ok(serde_json::to_string(&e).map_err(|e| AppError::new("serde_json::to_string(res_vec)", e))?)
}
