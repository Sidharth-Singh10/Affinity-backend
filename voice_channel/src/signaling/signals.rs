use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::SinkExt;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum SignalMessage {
    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String },
}

pub type RoomConnections = Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>;

const SFU_SERVER_URL: &str = "http://localhost:5555/sfu";
#[derive(Clone)]
pub struct SignallngServerState {
    pub rooms: RoomConnections,
    pub http_client: reqwest::Client,
}

// WebSocket handler for signaling.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
    State(state): State<SignallngServerState>,
) -> impl IntoResponse {
    let SignallngServerState { rooms, http_client } = state;
    ws.on_upgrade(move |socket| handle_connection(socket, room_id, rooms, http_client))
}

// Handle each WebSocket connection.
pub async fn handle_connection(
    socket: WebSocket,
    room_id: String,
    rooms: RoomConnections,
    http_client: reqwest::Client,
) {
    let (mut sender, mut receiver) = socket.split();

    let room_sender = {
        let mut rooms = rooms.lock().await;
        rooms
            .entry(room_id.clone())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(16);
                tx
            })
            .clone()
    };

    let mut room_receiver = room_sender.subscribe();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(Ok(msg)) = receiver.next() => {
                    if let Ok(text) = msg.to_text() {
                        let parsed_msg: SignalMessage = match serde_json::from_str(text) {
                            Ok(msg) => msg,
                            Err(_) => continue,
                        };

                        match parsed_msg {
                            SignalMessage::Offer { sdp } => {
                                // Forward the offer to the SFU server
                                if let Ok(response) = http_client.post(SFU_SERVER_URL)
                                    .json(&SignalMessage::Offer { sdp })
                                    .send().await {
                                    if let Ok(answer) = response.json::<SignalMessage>().await {
                                        // Send the answer back to the client
                                        if sender.send(Message::Text(serde_json::to_string(&answer).unwrap())).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                            _ => {
                                // Broadcast message to other clients in the room.
                                if room_sender.send(Message::Text(serde_json::to_string(&parsed_msg).unwrap())).is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
                Ok(msg) = room_receiver.recv() => {
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
}
