#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5555").await.unwrap();
    let app = voice_channel::run().await;
    axum::serve(listener, app).await.unwrap();
}
