use tokio_tungstenite::tungstenite::Message;
use voice_channel::run;
use futures_util::SinkExt;
use futures_util::StreamExt;
#[tokio::test]
async fn health_check_works() {
    
    spawn_app().await;
    
    let (mut ws_stream, _) = tokio_tungstenite::connect_async(format!("ws://127.0.0.1:5555/ws/1"))
        .await
        .expect("Failed to connect");

    // Send a message
    ws_stream
        .send(Message::Text("Hello, server".to_string()))
        .await
        .expect("Failed to send message");

    // Receive a message from the server
    let msg = ws_stream
        .next()
        .await
        .expect("No message received")
        .expect("Error in receiving message");
    assert_eq!(msg, Message::Text("Hello, client".to_string()));
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> u16 {
    let app = run().await;
    let listner = tokio::net::TcpListener::bind("0.0.0.0:5555").await.unwrap();
    let port = listner.local_addr().unwrap().port();
    tokio::spawn(async move { axum::serve(listner, app).await.unwrap() });
    port
}
