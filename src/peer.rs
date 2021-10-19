use std::{sync::{Arc, Mutex}};

use anyhow::Result;
use tokio::{sync::watch::{Receiver, Sender}};
use webrtc_data::data_channel::DataChannel;
use libp2p::{PeerId};
use webrtc::{api::{APIBuilder, setting_engine::SettingEngine}, data::data_channel::{RTCDataChannel}, peer::{
        configuration::RTCConfiguration,
        ice::{
            ice_candidate::{RTCIceCandidate, RTCIceCandidateInit},
            ice_server::RTCIceServer
        },
        peer_connection::RTCPeerConnection,
        peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription
    }};

use crate::{discovery::{Node}};

#[derive(Clone)]
pub(crate) enum PeerState {
    Initializing,
    Connecting,
    Connected(Arc<DataChannel>),
    Disconnected
    // TODO: Come up with a state when a peer is truly dead and should be discarded (so we can stop read/write loops)
}

impl std::fmt::Debug for PeerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected(_) => f.debug_tuple("Connected").finish(),
            Self::Disconnected => write!(f, "Disconnected"),
        }
    }
}

#[derive(Clone)]
pub struct Peer {
    pub(crate) id: PeerId,
    pub(crate) connection_state: Receiver<PeerState>,
    node: Node,
}

impl std::fmt::Debug for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Peer")
        .field("id", &self.id)
        .field("connection_state", &*self.connection_state.borrow())
        .finish()
    }
}

impl Peer {
    pub(crate) async fn new(id: PeerId, node: Node) -> Self {
        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                //TODO: add ICE-server?
                urls: vec![
                    "stun:stun.l.google.com:19302".to_owned(),
                    "stun:stun1.l.google.com:19302".to_owned(),
                    "stun:stun2.l.google.com:19302".to_owned(),
                    "stun:stun3.l.google.com:19302".to_owned(),
                    "stun:stun4.l.google.com:19302".to_owned()
                    ],
                ..Default::default()
            }],
            ..Default::default()
        };
        
        let mut s = SettingEngine::default();
        s.detach_data_channels();
        let api = Arc::new(APIBuilder::new()
        .with_setting_engine(s)
        .build());

        let (connection_state_sender, connection_state) = tokio::sync::watch::channel(PeerState::Initializing);

        tokio::spawn({
            let node = node.clone();
            async move {
                //TODO: Make this non-blocking
                Peer::connect(id, node, connection_state_sender, &api.new_peer_connection(config).await.unwrap()).await;
            }
        });

        Self {
            id,
            connection_state,
            node
        }
    }

    async fn connect(id: PeerId, node: Node, sender: Sender<PeerState>, connection: &RTCPeerConnection) {
        let (a, mut b) = tokio::sync::watch::channel(RTCPeerConnectionState::Unspecified);
        
        connection.on_peer_connection_state_change(Box::new({
            move |state: RTCPeerConnectionState| {
                //println!("Connection state changed: {:?}", state);
                a.send(state).unwrap();
                Box::pin(async move {})
            }
        }))
        .await;
        let local_id = node.local_peer_id.clone();
        let data_channel = if local_id > id {
            Peer::do_offer(id, local_id, connection, node.clone()).await.unwrap()
        } else {
            Peer::do_answer(id, local_id, connection, node.clone()).await.unwrap()
        };

        tokio::spawn(async move {
            loop {
                let data_channel = data_channel.clone();
                match *b.borrow() {
                    RTCPeerConnectionState::Disconnected | RTCPeerConnectionState::Failed | RTCPeerConnectionState::Closed => {
                        sender.send(PeerState::Disconnected).unwrap();
                    },
                    RTCPeerConnectionState::Connected => {
                        sender.send(PeerState::Connected(data_channel)).unwrap();
                    },
                    RTCPeerConnectionState::Connecting => { sender.send(PeerState::Connecting).unwrap(); },
                    _ => ()
                };
                if let Err(_) = b.changed().await {
                    break;
                }
            }
        });
    }

    async fn do_offer(id: PeerId, local_id: PeerId, peer_connection: &RTCPeerConnection, node: Node) -> Result<Arc<DataChannel>> {
        //println!("OFFER");
        let data_channel = peer_connection.create_data_channel("data", None).await?;
        
        // Create an offer to send to the other process
        let session_description = peer_connection.create_offer(None).await?;
        peer_connection.set_local_description(session_description.clone()).await?;

        let ice_candidates = Peer::gather_candidates(peer_connection).await;
        
        let offer = Signal { session_description, ice_candidates };
        Peer::put_signal(node.clone(), local_id, id, &bincode::serialize(&offer).unwrap()).await;

        let remote_offer = Peer::get_signal(id, local_id, node).await.unwrap();
        peer_connection.set_remote_description(remote_offer.session_description).await?;
        for candidate in remote_offer.ice_candidates {
            if let Err(err) = peer_connection.add_ice_candidate(RTCIceCandidateInit {
                candidate: candidate.to_json().await?.candidate,
                ..Default::default()
            }).await {
                panic!("{}", err);
            }
        }

        // Detach data_channel
        let (tx, rx) = tokio::sync::oneshot::channel();
        data_channel.clone().on_open(Box::new(move || {
            Box::pin(async move {
                let _ = tx.send(data_channel.detach().await.unwrap());
            })
        }))
        .await;
        let data_channel = rx.await.unwrap();

        Ok(data_channel)
    }

    async fn do_answer(id: PeerId, local_id: PeerId, peer_connection: &RTCPeerConnection, node: Node) -> Result<Arc<DataChannel>> {
        //println!("ANSWER");
        let remote_offer = Peer::get_signal(id, local_id, node.clone()).await.unwrap();
        peer_connection.set_remote_description(remote_offer.session_description).await?;

        for candidate in remote_offer.ice_candidates {
            if let Err(err) = peer_connection.add_ice_candidate(RTCIceCandidateInit {
                candidate: candidate.to_json().await?.candidate,
                ..Default::default()
            }).await {
                panic!("{}", err);
            }
        }

        let session_description = peer_connection.create_answer(None).await?;
        peer_connection.set_local_description(session_description.clone()).await?;

        let ice_candidates = Peer::gather_candidates(peer_connection).await;

        let offer = Signal {session_description, ice_candidates };
        
        Peer::put_signal(node, local_id, id, &bincode::serialize(&offer).unwrap()).await;

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        peer_connection.on_data_channel(Box::new(move |data_channel: Arc<RTCDataChannel>| {
            let tx = tx.clone();
            Box::pin(async move {
                // Detach data_channel
                data_channel.clone().on_open(Box::new(move || {
                    Box::pin(async move {
                        let _ = tx.send(data_channel.detach().await.unwrap()).await;
                    })
                }))
                .await;
            })
        })).await;
        let data_channel = rx.recv().await.unwrap();
        Ok(data_channel)
    }

    async fn put_signal(node: Node, from_peer: PeerId, to_peer: PeerId, offer: &Vec<u8>) {
        let key = format!("{}.signal.{}", from_peer, to_peer);
        node.put_record(&key, offer.to_vec(), None).await;
    }

    async fn get_signal(from_peer: PeerId, to_peer: PeerId, node: Node) -> Result<Signal, String> {
        let key = format!("{}.signal.{}", from_peer, to_peer);
        loop {
            if let Some(signal) = node.get_record(&key).await {
                break Ok(bincode::deserialize(&signal).unwrap());
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }

    async fn gather_candidates(peer_connection: &RTCPeerConnection) -> Vec<RTCIceCandidate> {
        //println!("Gather candidates...");
        let mut gather_complete = peer_connection.gathering_complete_promise().await;
        let candidates = Arc::new(Mutex::new(vec!()));
        peer_connection.on_ice_candidate(Box::new({
            let candidates = candidates.clone();
            move |c: Option<RTCIceCandidate>| {
                if let Some(candidate) = c {
                    candidates.lock().unwrap().push(candidate);
                }
                Box::pin(async move {})
            }
        })).await;
        
        let _ = gather_complete.recv().await;
        let candidates = candidates.lock().unwrap().clone();
        //println!("Got {} candidates!", candidates.len());
        candidates
    }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Signal {
    ice_candidates: Vec<RTCIceCandidate>,
    session_description: RTCSessionDescription,
}

#[derive(Clone, Debug, PartialEq)]
struct SignalKey {
    from_peer: PeerId,
    to_peer: PeerId
}
