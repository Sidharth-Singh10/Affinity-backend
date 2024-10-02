use axum::{
    http::{self, Method},
    routing::{get, post, put},
    Extension, Router,
};
use dotenv::dotenv;
use handlers::{
    auth_handlers::{
        login_handler, new_password_handler, otp_handler, send_pass_reset_handler, signup_handler,
    },
    cp_handler::code_handler,
    crud_handlers::{
        add_friend_handler, change_flag_handler, create_matched_handler, get_accepted_boys_handler,
        get_all_users_handler, get_boys_handler, get_girl_request_handler, get_girls_handler,
        get_matched_handler, get_user_by_id_handler, get_user_handler, reject_handler,
        update_contest_score_handler, update_score_handler, update_user_character_handler,
    },
};
use http::{header::HeaderValue, uri::Uri};
use sea_orm::Database;
use std::env;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod bcrypts;
mod configs;
mod handlers;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv().ok();

    // Get the database URL from the environment, report an error and stop if not found
    let db_string = env::var("DATABASE_URL").unwrap_or_else(|_| {
        println!("Error: DATABASE_URL not found in environment.");
        std::process::exit(1); // Terminate the program if DATABASE_URL is missing
    });

    // Get allowed origins from .env, if not found, use an empty list
    let allowed_origins_env = env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| {
        println!("Warning: ALLOWED_ORIGINS not set in .env file, defaulting to an empty list.");
        String::new()
    });

    // Parse the allowed origins from the environment
    let allowed_origins = allowed_origins_env
        .split(',')
        .filter(|s| !s.trim().is_empty()) // Filter out any empty values
        .filter_map(|origin| {
            // Attempt to parse each origin into a Uri, print an error if it fails
            match origin.parse::<Uri>() {
                Ok(valid_origin) => Some(valid_origin),
                Err(_) => {
                    println!("Warning: Invalid origin URL: {}", origin);
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    // If no origins are found, print a message
    if allowed_origins.is_empty() {
        println!(
            "Warning: No valid origins found in ALLOWED_ORIGINS. CORS will not allow any origins."
        );
    }

    // Configure CORS layer with dynamic origins
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

    // Add the valid origins from the environment
    for origin in &allowed_origins {
        println!("Allowing origin: {}", origin);

        // Convert Uri to HeaderValue
        if let Ok(header_value) = HeaderValue::from_str(&origin.to_string()) {
            cors = cors.allow_origin(AllowOrigin::exact(header_value));
        } else {
            println!("Warning: Failed to convert Uri to HeaderValue: {}", origin);
        }
    }

    // Connect to the database
    let db = Database::connect(&db_string).await.unwrap_or_else(|err| {
        println!("Error: Failed to connect to the database: {}", err);
        std::process::exit(1); // Terminate if the database connection fails
    });

    // Set up the app routes and layers
    let app: Router<()> = Router::new()
        .route("/sendpassreset", get(send_pass_reset_handler))
        .route("/newpassword", get(new_password_handler))
        .route("/otp", get(otp_handler))
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/getuser", post(get_user_handler))
        .route("/getboys", get(get_boys_handler))
        .route("/getgirls", get(get_girls_handler))
        .route("/updatescore", put(update_score_handler))
        .route("/getallusers", get(get_all_users_handler))
        .route("/addfriend", post(add_friend_handler))
        .route("/updatecharacter", put(update_user_character_handler))
        .route("/getgirlrequests", post(get_girl_request_handler))
        .route("/getacceptedboys", post(get_accepted_boys_handler))
        .route("/changeflag", post(change_flag_handler))
        .route("/creatematch", post(create_matched_handler))
        .route("/getmatched", post(get_matched_handler))
        .route("/updatecontestscore", put(update_contest_score_handler))
        .route("/getuserbyid", post(get_user_by_id_handler))
        .route("/reject", post(reject_handler))
        .layer(cors)
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening");

    axum::serve(listener, app).await.unwrap();
}
