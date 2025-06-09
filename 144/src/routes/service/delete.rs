use warp::{Filter, Reply};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("service" / "delete" / String)
        .and(warp::delete())
        .then(handler)
}

pub async fn handler(service_id: String) -> impl Reply {
    // TODO: Implement service deletion
    warp::reply::with_status(
        format!("Service delete not yet implemented for service: {}", service_id),
        warp::http::StatusCode::NOT_IMPLEMENTED,
    )
} 