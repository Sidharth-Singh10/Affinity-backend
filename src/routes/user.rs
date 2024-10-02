use axum::{routing::get, Router};

use crate::handlers::crud_handlers::get_all_users_handler;

pub fn user_routes() -> Router {
    Router::new().route("/getallusers", get(get_all_users_handler))
}
