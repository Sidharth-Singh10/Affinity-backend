use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;
use webrtc::api::{media_engine::MediaEngine, APIBuilder};
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use webrtc::rtp_transceiver::rtp_transceiver_direction::RTCRtpTransceiverDirection;
use webrtc::rtp_transceiver::{RTCRtpCodingParameters, RTCRtpTransceiverInit};
use webrtc::track::track_local::track_local_static_rtp::TrackLocalStaticRTP;
// use std::future::IntoFuture;

pub type PeerConnections = Arc<Mutex<HashMap<Uuid, Arc<RTCPeerConnection>>>>;

// Handler to handle WebRTC connection and SDP offer from clients
pub async fn connect_handler(
    peer_connections: axum::extract::State<PeerConnections>,
) -> impl axum::response::IntoResponse {
    // Configure Media Engine with basic video and audio codecs
    let mut media_engine = MediaEngine::default();
    media_engine.register_default_codecs().unwrap();

    // Set up WebRTC API
    let api = APIBuilder::new().with_media_engine(media_engine).build();

    // Create a new RTC peer connection configuration
    let config = RTCConfiguration::default();
    let peer_connection = Arc::new(api.new_peer_connection(config).await.unwrap());

    // Generate a unique ID for the new peer
    let user_id = Uuid::new_v4();

    // Store peer connection in shared state
    peer_connections
        .lock()
        .await
        .insert(user_id, peer_connection.clone());

    // Set up event handlers
    setup_peer_connection_handlers(peer_connection.clone(), peer_connections.clone().0, user_id)
        .await;

    format!("User connected with ID: {}", user_id)
}

// Set up event handlers to handle incoming tracks and ICE candidates
async fn setup_peer_connection_handlers(
    peer_connection: Arc<RTCPeerConnection>,
    peer_connections: PeerConnections,
    user_id: Uuid,
) {
    // Handle incoming tracks from this peer
    let peer_connections_clone = peer_connections.clone();

    peer_connection.on_track(Box::new(move |track, _receiver, _| {
        let peer_connections_clone = peer_connections_clone.clone();
        Box::pin(async move {
            println!("Received track from user: {:?}", user_id);

            // Forward track to all other peers
            let connections = peer_connections_clone.lock().await;
            for (&id, other_peer) in connections.iter() {
                if id != user_id {
                    let _ = forward_track_to_peer(track.clone(), other_peer.clone()).await;
                }
            }
        })
    }));
}

async fn forward_track_to_peer(
    track: Arc<webrtc::track::track_remote::TrackRemote>,
    peer: Arc<RTCPeerConnection>,
) -> Result<(), webrtc::Error> {
    let codec_parameters = track.codec();
    let codec_capability = RTCRtpCodecCapability {
        mime_type: codec_parameters.capability.mime_type.clone(),
        clock_rate: codec_parameters.capability.clock_rate,
        channels: codec_parameters.capability.channels,
        sdp_fmtp_line: codec_parameters.capability.sdp_fmtp_line.clone(),
        ..Default::default() // Use default values for other encoding parameters
    };
    // Create a local track to forward to the peer
    let local_track = Arc::new(TrackLocalStaticRTP::new(
        codec_capability,
        "audio".to_owned(), // Specify a media ID (e.g., "video" or "audio")
        "webrtc-rs".to_owned(),
    ));
    // Specify encoding settings for the transceiver
    let send_encodings = vec![RTCRtpCodingParameters {
        ssrc: 0,         // You can specify an SSRC if needed, otherwise let the system generate it
        payload_type: 0, // Automatically set based on codec
        ..Default::default()  // Use default values for other encoding parameters
    }];

    // Add the local track to the peer connection with an appropriate transceiver direction
    let transceiver_init = RTCRtpTransceiverInit {
        direction: RTCRtpTransceiverDirection::Sendonly,
        // send_encodings: todo!(),
        send_encodings,
    };

    // Add transceiver to peer connection for forwarding
    peer.add_transceiver_from_track(local_track.clone(), Some(transceiver_init))
        .await?;

    // Forward RTP packets from the remote track to the local track
    tokio::spawn(async move {
        while let Ok(rtp_packet) = track.read_rtp().await {
            if let Err(e) = local_track.write_rtp(&rtp_packet.0).await {
                eprintln!("Error forwarding RTP packet: {:?}", e);
            }
        }
    });

    Ok(())
}
use webrtc::track::track_local::TrackLocalWriter;
