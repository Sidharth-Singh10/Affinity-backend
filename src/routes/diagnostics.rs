use axum::{routing::get, Router};

use crate::handlers::tests_handlers::health_check;

pub fn diagnostics_routes() -> Router {
    Router::new().route("/health_check", get(health_check))
}
