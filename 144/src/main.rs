use i144::constants::SECRETS;
use i144::routes;
// use futures_util::StreamExt;
use i144::routes::a::add::AddRequest;
use i144::routes::arbitrage::arbitrage;
use shuttle_runtime::SecretStore;
// use tokio::sync::mpsc;
use tokio::task;
// use warp::filters::ws::Message;
use warp::Filter;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let mut secrets_ = SECRETS.lock().await;
    *secrets_ = secrets;

    // let (tx, mut rx) = mpsc::channel::<String>(100);

    // #[cfg(debug_assertions)]
    // {
    //     if let Err(e) = setup().await {
    //         println!("setup error: {:#?}", e);
    //     } else {
    //         println!("setup done");
    //     }
    // }

    // Setup a task to periodically clean up old signaling data
    task::spawn(async {
        loop {
            if let Err(e) = routes::voicechat::signaling::cleanup_old_signals().await {
                eprintln!("Error cleaning up signals: {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(600)).await; // Clean every 10 minutes
        }
    });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type"]);

    Ok(warp::path("a")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<routes::a::search::Search>())
        .then(routes::a::search::r)
        // .or(warp::path("ws")
        //     .and(warp::ws())
        //     .map(move |ws: warp::ws::Ws| {
        //         // let rx = rx.clone();
        //         ws.on_upgrade(move |websocket| async move {
        //             let (mut ws_tx, _) = websocket.split();
        //             while let Some(msg) = rx.recv().await {
        //                 ws_tx.send(Message::text(msg)).await.unwrap()
        //             }
        //         })
        //     }))
        // .or(warp::path())
        .or(warp::path("a")
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::bytes())
            .then(|t: bytes::Bytes| async move {
                let request = AddRequest {
                    i: None,
                    c: None,
                    t: String::from_utf8_lossy(&t).to_string(),
                };
                routes::a::add::r(request).await
            }))
        .or(warp::path!("a" / String)
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::bytes())
            .then(|i: String, t: bytes::Bytes| async move {
                let request = AddRequest {
                    i: Some(i),
                    c: None,
                    t: String::from_utf8_lossy(&t).to_string(),
                };
                routes::a::add::r(request).await
            }))
        .or(warp::path("arbitrage").map(|| {
            task::spawn(arbitrage());
            warp::reply()
        }))
        .or(routes::setup::route())
        .or(routes::bible::routes())
        .or(routes::verses::routes())
        .or(routes::voicechat::routes().with(cors.clone()))
        .or(routes::chat::routes().with(cors.clone()))
        .or(routes::user::routes().with(cors.clone()))
        .or(routes::zone::routes().with(cors.clone()))
        .or(routes::auth::routes().with(cors.clone()))
        .or(routes::product::routes().with(cors.clone()))
        .or(routes::service::routes().with(cors.clone()))
        .or(routes::item::routes().with(cors.clone()))
        .or(routes::chatgroup::routes().with(cors.clone()))
        .boxed()
        .into())
}
