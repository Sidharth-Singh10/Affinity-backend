use axum::{routing::post, Router};

use crate::handlers::auth_handlers::login_handler;

pub fn auth_routes() -> Router {
    Router::new().route("/login", post(login_handler))
}
