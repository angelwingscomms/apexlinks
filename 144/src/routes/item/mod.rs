pub mod search;

pub fn routes() -> impl warp::Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    search::route()
} 