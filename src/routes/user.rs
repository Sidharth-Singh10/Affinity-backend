use axum::{
    routing::{get, post, put},
    Router,
};

use crate::handlers::crud_handlers::{
    get_all_users_handler, get_user_avatar, get_user_by_id_handler, get_user_handler,
    update_user_avatar, update_user_character_handler,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/getuser", get(get_user_handler))
        .route("/getallusers", get(get_all_users_handler))
        .route("/updatecharacter", put(update_user_character_handler))
        .route("/getuserbyid", post(get_user_by_id_handler))
        .route("/getuseravatar", get(get_user_avatar))
        .route("/updateuseravatar", post(update_user_avatar))
}
