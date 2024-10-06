use axum::{routing::put, Router};

use crate::handlers::crud_handlers::{update_contest_score_handler, update_score_handler};

pub fn score_routes() -> Router {
    Router::new()
        .route("/updatescore", put(update_score_handler))
        .route("/updatecontestscore", put(update_contest_score_handler))
}
