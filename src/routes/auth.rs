use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::auth_handlers::{
    login_handler, new_password_handler, otp_handler, send_pass_reset_handler, signup_handler,
};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/sendpassreset", get(send_pass_reset_handler))
        .route("/newpassword", get(new_password_handler))
        .route("/otp", get(otp_handler))
}
