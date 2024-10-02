use rusty_backend::run;
#[tokio::test]
async fn health_check_works() {
    // Arrange
    let port = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get( format!("http://127.0.0.1:{}/health_check",port))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> u16 {
    let app = run().await;
    let listner = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listner.local_addr().unwrap().port();
    tokio::spawn(async move { axum::serve(listner, app).await.unwrap() });
    port
}
