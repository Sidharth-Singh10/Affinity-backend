use rusty_backend::run;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    let app = run().await;

    axum::serve(listener, app).await.unwrap();
}
