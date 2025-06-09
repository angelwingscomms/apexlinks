pub mod embed;

/// Bible verse routes
pub fn routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    embed::route()
} 