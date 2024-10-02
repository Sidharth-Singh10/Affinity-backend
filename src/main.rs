use rusty_backend::run;

mod bcrypts;
mod configs;
mod handlers;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    let app = run().await;

    axum::serve(listener, app).await.unwrap();
}
