use axum::{
    routing::{post, put},
    Router,
};

use crate::handlers::crud_handlers::{update_game_session_score, update_score_handler};

pub fn score_routes() -> Router {
    Router::new()
        .route("/updatescore", put(update_score_handler))
        .route("/updatecontestscore", post(update_game_session_score))
}
