use warp::{Filter, Reply};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("zone" / "delete" / String)
        .and(warp::delete())
        .then(handler)
}

pub async fn handler(zone_id: String) -> impl Reply {
    // TODO: Implement zone deletion
    warp::reply::with_status(
        format!("Zone delete not yet implemented for zone: {}", zone_id),
        warp::http::StatusCode::NOT_IMPLEMENTED,
    )
} 