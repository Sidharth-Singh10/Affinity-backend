use axum::{routing::get, Router};

use crate::handlers::aws_handlers::{create_presigned_upload_url, upload_to_aws};

pub fn aws_routes() -> Router {
    Router::new()
        .route("/uploadaws", get(upload_to_aws))
        .route("/presigned_url", get(create_presigned_upload_url))
}
