use axum::{
    http::{self, Method},
    routing::{get, post},
    Extension, Router,
};
use controller::{code_handler, decode_jwt, login_handler, signup_handler};
use sea_orm::{ColIdx, Database};
use tower_http::cors::{Any, CorsLayer,AllowOrigin};
mod bcrypts;
mod controller;
mod db;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    let db_string = (*utils::constants::DATABASE_URL).clone();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))// Allow only this specific origin   
        .allow_headers([
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ORIGIN,
            http::header::SET_COOKIE,   
        ])
        .allow_credentials(true);
    
    let db = Database::connect(db_string)
        .await
        .expect("could not connect");
    let app: Router<()> = Router::new()
        
        .route("/login", post(login_handler))
        .route("/decode", get(decode_jwt))
        .route("/signup", post(signup_handler))
        .route("/runcode", post(code_handler))
        .layer(cors)
        .layer(Extension(db));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening");

    axum::serve(listner, app).await.unwrap();
}
