use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::crud_handlers::{
    add_friend_handler, change_flag_handler, create_matched_handler, get_accepted_boys_handler,
    get_boys_handler, get_girl_request_handler, get_girls_handler, get_matched_handler,
    reject_handler,
};

pub fn matchmaking_routes() -> Router {
    Router::new()
        .route("/getboys", get(get_boys_handler))
        .route("/getacceptedboys", post(get_accepted_boys_handler))
        .route("/getgirls", get(get_girls_handler))
        .route("/getgirlrequests", post(get_girl_request_handler))
        .route("/addfriend", post(add_friend_handler))
        .route("/changeflag", post(change_flag_handler))
        .route("/creatematch", post(create_matched_handler))
        .route("/getmatched", post(get_matched_handler))
        .route("/reject", post(reject_handler))
}
