use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::SinkExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

use futures_util::StreamExt;
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum SignalMessage {
    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String },
}

pub type RoomConnections = Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>;

// WebSocket handler for signaling.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(rooms): State<RoomConnections>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, room_id, rooms))
}

// Handle each WebSocket connection.
pub async fn handle_connection(socket: WebSocket, room_id: String, rooms: RoomConnections) {
    let (mut sender, mut receiver) = socket.split();

    let room_sender = {
        let rooms = rooms.lock();
        rooms
            .await
            .entry(room_id.clone())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(16);
                tx
            })
            .clone()
    };

    let mut room_receiver = room_sender.subscribe();

    tokio::spawn(async move {
        // Forward messages to other clients in the same room.
        loop {
            tokio::select! {
                Some(Ok(msg)) = receiver.next() => {
                    if let Ok(text) = msg.to_text() {
                        let parsed_msg: SignalMessage = match serde_json::from_str(text) {
                            Ok(msg) => msg,
                            Err(_) => continue,
                        };

                        // Broadcast message to other clients in the room.
                        if room_sender.send(Message::Text(serde_json::to_string(&parsed_msg).unwrap())).is_err() {
                            break;
                        }
                    }
                }
                Ok(msg) = room_receiver.recv() => {
                    // Forward received broadcast messages to the client.
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
}
