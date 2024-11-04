use axum::{routing::get, Router};

use crate::handlers::aws_handlers::upload_to_aws;

pub fn aws_routes() -> Router {
    Router::new().route("/uploadaws", get(upload_to_aws))
}
