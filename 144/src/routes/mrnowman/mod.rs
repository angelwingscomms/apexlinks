pub mod add;
pub mod chat;

use add::{add, Add};
use tokio::task;

use crate::{
    constants::{MR_NOWMAN_CHAT, MR_NOWMAN_MESSAGE},
    util::groq::{groq, Message},
};

use serde_json::json;

use crate::{
    util::qdrant::{qdrant_path, qdrant_post, qdrant_put},
    util::{embed, id},
    util::{AppError, AppResult},
};

use warp::reply::Reply;

#[derive(serde::Deserialize, Debug)]
pub struct M {
    m: String,
    i: Option<String>,
}

pub async fn m(s: M, a: Option<std::net::SocketAddr>) -> impl Reply {
    f(s, a).await.map_or_else(
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

pub async fn f(s: M, addr: Option<std::net::SocketAddr>) -> AppResult<serde_json::Value> {
    let ud = chrono::Utc::now().to_rfc3339();

    // system and instruction messages
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: "You are a friendly chat assistant".to_string(),
    }, Message {
        role: "assistant".to_string(),
        content: r#"You are a customer service assistant that answers questions about Udoflow. You are very friendly and respond as human like as possible. Be as concise as possible. Always give out information that is relevant to the question asked. Give out information in an organized format, adding numbering and bullet points were deemed important. Do not give away unnecessary information that you are not asked. Be as fluent as possible and use your logical thinking to answer any question asked by the user relating to avoiding procrastination, and staying disciplined and motivated on tasks."#.to_string(),
    }];

    let final_response: serde_json::Value;
    let chat_id: String;
    let completion: String;

    // if chat_id in request
    if let Some(i) = s.i {
        chat_id = i;

        // prepare messages
        let stored_messages: Vec<Message> = futures::future::try_join_all(qdrant_post(
            &qdrant_path("collections/i/points/query").await?,
            json!({"with_payload": ["m", "u"],"filter": {"must": [{"key": "i", "match": {"value": chat_id}}, {"key": "c", "match": {"value": MR_NOWMAN_MESSAGE}}]}}),
        )
        .await?["result"]["points"]
            .as_array()
            .ok_or(AppError::new_plain(&format!("didn't get array when trying to get stored messages for chat_id: {}", chat_id)))?
            .iter().map(|x| async {Message::new(x).await})).await?;
        messages.extend(stored_messages.clone());
        messages.extend(vec![Message {
            role: "user".to_string(),
            content: s.m.clone(),
        }]);

        completion = groq(messages).await?;

        // set chat_id
        let user_date = ud.clone();
        let chat_id_clone = chat_id.clone();
        task::spawn(async move {
            qdrant_put(
                &qdrant_path("collections/i/points").await?,
                json!({"points":[{"id":chat_id_clone, "payload": {"d": user_date}, "vector": embed(serde_json::to_string(&stored_messages.clone()).map_err(|e| AppError::new("stored_messages to string : m", e))?).await? }]}),
            ).await
        });

        // set response
        final_response = json!(completion.clone());
    } else {
        chat_id = id();

        // prepare messages
        messages.extend(vec![Message {
            content: s.m.clone(),
            role: "user".to_string(),
        }]);
        completion = groq(messages).await?;
        final_response = json!({"m": completion, "i": chat_id});

        // set chat_id
        let sm = s.m.clone();
        let user_date = ud.clone();
        let cc = completion.clone();
        qdrant_put(
            &qdrant_path("collections/i/points").await?,
            json!({"points":[{"id":chat_id, "payload": {"d": user_date, "c": MR_NOWMAN_CHAT}, "vector": embed(serde_json::to_string(&vec![&sm, &cc]).map_err(|e| AppError::new("creating embedding for i, turning vec arg to string", e))?).await? }]}),
        ).await?;
    };

    task::spawn(add(
        Add {
            a: completion,
            c: s.m,
            u: None,
            ud,
            ad: chrono::Utc::now().to_rfc3339(), // *learn when this is evaluated
            i: chat_id,
        },
        addr,
    ));
    Ok(final_response.into())
}

/*pub async fn next_p(i: i64) -> AppResult<i64> {
    let id: i64 = qdrant_post(
        &qdrant_path("collections/i/points").await?,
        json!({"ids": [i], "with_payload": ["p"]}),
    )
    .await?["result"][0]["payload"]["p"]
        .as_i64()
        .unwrap_or(0);
    let next = id + 1;
    qdrant_post(
        &qdrant_path("collections/i/points/payload?wait=true").await?,
        json!({"payload": {"p": next}, "points": [i]}),
    )
    .await?;
    Ok(id)
} */
