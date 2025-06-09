use warp::{Filter, Reply};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("product" / "delete" / String)
        .and(warp::delete())
        .then(handler)
}

pub async fn handler(product_id: String) -> impl Reply {
    // TODO: Implement product deletion
    warp::reply::with_status(
        format!("Product delete not yet implemented for product: {}", product_id),
        warp::http::StatusCode::NOT_IMPLEMENTED,
    )
} 