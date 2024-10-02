use axum::{
    http::{self, Method},
    Extension, Router,
};

use routes::*;
use sea_orm::Database;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod bcrypts;
mod configs;
mod handlers;
mod model;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let db_string = (*utils::constants::DATABASE_URL).clone();

    // Use ALLOWED_ORIGINS from constants.rs
    let allowed_origins = (*utils::constants::ALLOWED_ORIGINS).clone();

    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ORIGIN,
            http::header::SET_COOKIE,
        ])
        .allow_credentials(true);

    // Configure CORS for each allowed origin
    for origin in &allowed_origins {
        println!("Allowing origin: {}", origin);
        if let Ok(header_value) = http::header::HeaderValue::from_str(origin) {
            cors = cors.allow_origin(AllowOrigin::exact(header_value));
        } else {
            println!(
                "Warning: Failed to convert origin to HeaderValue: {}",
                origin
            );
        }
    }

    let db = Database::connect(db_string)
        .await
        .expect("could not connect");
    let app: Router<()> = Router::new()
        .nest("/auth", auth_routes())
        .nest("/user", user_routes())
        .layer(cors)
        .layer(Extension(db));

    // let app: Router<()> = Router::new()

    //     // auth
    //     .route("/login", post(login_handler))
    //     .route("/signup", post(signup_handler))
    //     .route("/sendpassreset", get(send_pass_reset_handler))
    //     .route("/newpassword", get(new_password_handler))
    //     .route("/otp", get(otp_handler))
    //
    //     // user
    //     .route("/getuser", post(get_user_handler))
    //     .route("/getallusers", get(get_all_users_handler))
    //     .route("/updatecharacter", put(update_user_character_handler))
    //     .route("/getuserbyid", post(get_user_by_id_handler))
    //
    //     // friends
    //     .route("/getboys", get(get_boys_handler))
    //     .route("/getacceptedboys", post(get_accepted_boys_handler))
    //     .route("/getgirls", get(get_girls_handler))
    //     .route("/getgirlrequests", post(get_girl_request_handler))
    //     .route("/addfriend", post(add_friend_handler))
    //     .route("/changeflag", post(change_flag_handler))
    //
    //     // score
    //     .route("/updatescore", put(update_score_handler))
    //     .route("/updatecontestscore", put(update_contest_score_handler))
    //
    //     // match
    //     .route("/creatematch", post(create_matched_handler))
    //     .route("/getmatched", post(get_matched_handler))
    //     .route("/reject", post(reject_handler))
    //
    //     .layer(cors)
    //     .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening");

    axum::serve(listener, app).await.unwrap();
}
