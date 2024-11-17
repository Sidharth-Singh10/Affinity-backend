use std::{collections::HashMap, sync::Arc};

use axum::{routing::get, Router};
use sfu::connect_handler;
use signaling::{websocket_handler, RoomConnections, SignallngServerState};
use tokio::sync::Mutex;
mod sfu;
mod signaling;
pub async fn run() -> Router<()> {
    let rooms: RoomConnections = Arc::new(Mutex::new(HashMap::new()));
    let peer_connections: sfu::PeerConnections = Arc::new(Mutex::new(HashMap::new()));
    let http_client = reqwest::Client::new();

    Router::new()
        .route("/ws/:room_id", get(websocket_handler))
        .with_state(SignallngServerState { rooms, http_client })
        .route("/sfu", get(connect_handler))
        .with_state(peer_connections.clone())
}
