pub mod add;
pub mod search;
pub mod edit;
pub mod delete;

use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    add::route()
        .or(search::route())
        .or(edit::route())
        .or(delete::route())
} 