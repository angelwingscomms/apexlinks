pub mod add;
pub mod edit;
pub mod delete;
pub mod search;
pub mod types;

use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    add::route()
        .or(edit::route())
        .or(delete::route())
        .or(search::route())
} 