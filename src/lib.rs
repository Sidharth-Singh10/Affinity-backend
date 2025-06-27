use std::sync::Arc;

use axum::{
    http::{self, Method},
    Extension, Router,
};

use middlewares::auth::authorization_middleware;
use redis::{Client, Commands, Connection, RedisResult};
use routes::*;
use sea_orm::Database;
use tokio::sync::Mutex;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod bcrypts;
mod configs;
mod errors;
mod handlers;
mod middlewares;
mod model;
mod routes;
mod utils;

pub async fn run() -> Router<()> {
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

    let redis_client = Arc::new(RedisClient::new());

    let app: Router<()> = Router::new()
        .nest("/user", user_routes())
        .nest("/matchmaking", matchmaking_routes())
        .nest("/score", score_routes())
        // .layer(axum::middleware::from_fn(authorization_middleware))
        .nest("/auth", auth_routes())
        .nest("/diagnostics", diagnostics_routes())
        .nest("/aws", aws_routes())
        .layer(cors)
        .layer(Extension(db))
        .layer(Extension(redis_client));
    // .layer(Extension(redis_con));

    println!("Listening");
    app
}

pub struct RedisClient {
    connection: Arc<Mutex<Connection>>,
}

impl RedisClient {
    // Initialize RedisClient with a new connection
    pub fn new() -> Self {
        let client = Client::open("redis://127.0.0.1:6379/").expect("Invalid Redis URL");
        let connection = client.get_connection().expect("Failed to connect to Redis");
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// set a value in Redis
    pub async fn set_value(&self, key: &str, value: &str) -> RedisResult<()> {
        let mut con = self.connection.lock().await;
        con.set(key, value)
    }

    /// get a value from Redis
    pub async fn get_value(&self, key: &str) -> RedisResult<Option<String>> {
        let mut con = self.connection.lock().await;
        con.get(key)
    }
}
impl Default for RedisClient {
    fn default() -> Self {
        Self::new()
    }
}
