pub mod search;

use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    search::route()
} 