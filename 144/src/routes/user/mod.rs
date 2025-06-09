pub mod search;
pub mod similarity;
pub mod join_zone;
pub mod leave_zone;
pub mod get;

use warp::Filter;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    search::route()
        .or(similarity::route())
        .or(join_zone::route())
        .or(leave_zone::route())
        .or(get::route())
} 