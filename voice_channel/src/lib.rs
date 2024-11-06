use std::{collections::HashMap, sync::Arc};

use axum::{routing::get, Router};
use signaling::{websocket_handler, RoomConnections};
use tokio::sync::Mutex;
mod signaling;

pub async fn run() -> Router<()> {
    let rooms: RoomConnections = Arc::new(Mutex::new(HashMap::new()));
    Router::new()
        .route("/ws/:room_id", get(websocket_handler))
        .with_state(rooms.clone())
}
