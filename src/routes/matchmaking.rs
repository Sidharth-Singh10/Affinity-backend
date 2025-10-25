use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::crud_handlers::{
    change_status_to_contest, create_match, get_accepted_matches, get_boys_game_scores_for_girl,
    get_boys_handler, get_contest_matches, get_girl_request, get_girls_handler, reject_match,
};

pub fn matchmaking_routes() -> Router {
    Router::new()
        .route("/getboys", get(get_boys_handler))
        .route("/getacceptedboys", post(get_contest_matches))
        .route("/getgirls", get(get_girls_handler))
        .route("/getGirlRequest", post(get_girl_request))
        .route("/creatematch", post(create_match))
        .route("/changeflag", post(change_status_to_contest))
        .route("/reject", post(reject_match))
        .route("/getAcceptedMatched", get(get_accepted_matches))
        .route(
            "/getBoysGameScoresForGirl",
            post(get_boys_game_scores_for_girl),
        )
}
