use std::{task::{Context, Poll}};

use libp2p::{NetworkBehaviour, PeerId, development_transport, identity, kad::{AddProviderOk, GetProvidersOk, GetRecordOk, KademliaEvent, PutRecordOk, QueryId, QueryResult, Quorum, Record, record::{self, Key, store::MemoryStore}}, mdns::{Mdns, MdnsConfig, MdnsEvent}, swarm::{SwarmBuilder, NetworkBehaviourEventProcess, SwarmEvent}};
use futures::prelude::*;
use tokio::sync::{broadcast::Receiver};

pub(crate) const PREFIX: &str = "nestest";

type Kademlia = libp2p::kad::Kademlia<MemoryStore>;

// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct MyBehaviour {
    kademlia: Kademlia,
    mdns: Mdns,

    #[behaviour(ignore)]
    event_bus: EventBus,

    #[behaviour(ignore)]
    #[allow(dead_code)]
    t: Receiver<KademliaEvent2>, //TODO: keep the event bus alive somehow. Could probably be removed now..
}

impl MyBehaviour {
    fn new(local_peer_id: PeerId, mdns: Mdns) -> Self {
        // Create a Kademlia behaviour.
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);

        let (event_bus, t) = tokio::sync::broadcast::channel(16);
        
        Self {
            kademlia,
            mdns,
            event_bus,
            t
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    // Called when `kademlia` produces an event.
    fn inject_event(&mut self, message: KademliaEvent) {
        match message {
            KademliaEvent::OutboundQueryCompleted { result, id, .. } => {
                match result {
                    QueryResult::StartProviding(result) => {
                        let result = result.map_err(|_| format!("TODO: error for add provider error"));
                        let _ = self.event_bus.send(KademliaEvent2 { query_id: id, response: KademliaResponse::StartProviding(result) });
                    },
                    QueryResult::GetProviders(result) => {
                        let result = result.map_err(|_| format!("TODO: error for get providers error"));
                        let _ = self.event_bus.send(KademliaEvent2 { query_id: id, response: KademliaResponse::GetProviders(result) });
                    },
                    QueryResult::PutRecord(result) => {
                        let result = result.map_err(|_| format!("TODO: error for put record error"));
                        let _ = self.event_bus.send(KademliaEvent2 { query_id: id, response: KademliaResponse::PutRecord(result) });
                    },
                    QueryResult::GetRecord(result) => {
                        let result = result.map_err(|_| format!("TODO: error for get record error"));
                        let _ = self.event_bus.send(KademliaEvent2 { query_id: id, response: KademliaResponse::GetRecord(result) });
                    },
                    _ => {
                        // Ignore the rest
                    }
                }
            },
            _ => {}
        }
    }
}

type EventBus = tokio::sync::broadcast::Sender<KademliaEvent2>;
#[derive(Clone)]
enum KademliaResponse {
    StartProviding(CommandResult<AddProviderOk>),
    GetProviders(CommandResult<GetProvidersOk>),
    PutRecord(CommandResult<PutRecordOk>),
    GetRecord(CommandResult<GetRecordOk>),
}
#[derive(Clone)]
struct KademliaEvent2 {
    query_id: QueryId,
    response: KademliaResponse
}

type Responder<T> = tokio::sync::oneshot::Sender<CommandResult<T>>;
type CommandResult<T> = Result<T, String>;

#[derive(Debug)]
enum Command {
    StartProviding(record::Key, Responder<AddProviderOk>),
    GetProviders(record::Key, Responder<GetProvidersOk>),
    PutRecord(record::Record, Responder<PutRecordOk>),
    GetRecord(record::Key, Responder<GetRecordOk>),
}

type CommandBus = tokio::sync::mpsc::Sender<Command>;

#[derive(Clone)]
pub(crate) struct Node {
    command_bus: CommandBus,
    pub(crate) local_peer_id: PeerId
}

impl Node {
    pub(crate) async fn new() -> Self {
        let (command_bus, local_peer_id) = Node::setup_discovery().await;

        Self {
            command_bus, local_peer_id
        }
    }

    pub(crate) async fn start_providing(self: &Self, key: Key) -> CommandResult<AddProviderOk> {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        self.command_bus.send(Command::StartProviding(key, resp_tx)).await.unwrap();
        resp_rx.await.unwrap()
    }

    pub(crate) async fn get_providers(self: &Self, key: Key) -> CommandResult<GetProvidersOk> {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        self.command_bus.send(Command::GetProviders(key, resp_tx)).await.unwrap();
        resp_rx.await.unwrap()
    }
    
    pub(crate) async fn put_record(self: &Self, record: Record) -> CommandResult<PutRecordOk> {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        self.command_bus.send(Command::PutRecord(record, resp_tx)).await.unwrap();
        resp_rx.await.unwrap()
    }

    pub(crate) async fn get_record(self: &Self, key: Key) -> CommandResult<GetRecordOk> {
        let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
        self.command_bus.send(Command::GetRecord(key, resp_tx)).await.unwrap();
        resp_rx.await.unwrap()
    }

    async fn setup_discovery() -> (CommandBus, PeerId) {
        // Create a random key for ourselves.
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("We are peer {:?}", local_peer_id);
        // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol.
        let transport = development_transport(local_key).await.unwrap();

        // Create a swarm to manage peers and events.
        let mut swarm = {
            
            let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();
            let behaviour = MyBehaviour::new(local_peer_id, mdns);
            SwarmBuilder::new(transport, behaviour, local_peer_id)
            // We want the connection background tasks to be spawned
            // onto the tokio runtime.
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
        };
 
        // Listen on all interfaces and whatever port the OS assigns.
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
        let (command_bus, mut command_bus_rx) = tokio::sync::mpsc::channel::<Command>(10);
        let event_bus = swarm.behaviour().event_bus.clone();
        
        fn catch_event<T, F>(query_id: Result<QueryId, libp2p::kad::store::Error>, mut event_bus: tokio::sync::broadcast::Receiver<KademliaEvent2>, responder: Responder<T>, func: F)
            where F: Fn(KademliaResponse) -> Option<Result<T, String>> + Send,
            T: Send + 'static,
            F: 'static
        {
            let query_id = query_id.map_err(|_| format!("TODO: proper error handling"));
            match query_id {
                Ok(query_id) => {
                    tokio::spawn(async move {
                        let res = loop {
                            if let Ok(event) = event_bus.recv().await {
                                if event.query_id == query_id {
                                    let res = func(event.response);
                                    if let Some(res) = res {
                                        break res;
                                    }
                                }
                            }
                        };

                        let _ = responder.send(res);
                    });
                },
                Err(_) => {
                    let _ = responder.send(Err("TODO".to_string()));
                }
            }
        }
        // Kick it off.
        tokio::spawn(future::poll_fn(move |cx: &mut Context<'_>| {
            loop {
                match command_bus_rx.poll_recv(cx) {
                    
                    Poll::Ready(Some(command)) => {
                        match command {
                            Command::StartProviding(key, responder) => {
                                let query_id = swarm.behaviour_mut().kademlia.start_providing(key);
                                catch_event(query_id, event_bus.subscribe(), responder, |response| {
                                    match response {
                                        KademliaResponse::StartProviding(result) => Some(result),
                                        _ => None
                                    }
                                });
                            },
                            Command::GetProviders(key, responder) => {
                                let query_id = swarm.behaviour_mut().kademlia.get_providers(key);
                                catch_event(Ok(query_id), event_bus.subscribe(), responder, |response| {
                                    match response {
                                        KademliaResponse::GetProviders(result) => Some(result),
                                        _ => None
                                    }
                                });
                            }
                            Command::PutRecord(record, responder) => {
                                let query_id = swarm.behaviour_mut().kademlia.put_record(record.clone(), Quorum::One);
                                catch_event(query_id, event_bus.subscribe(), responder, |response| {
                                    match response {
                                        KademliaResponse::PutRecord(result) => Some(result),
                                        _ => None
                                    }
                                });
                            },
                            Command::GetRecord(key, responder) => {
                                let query_id = swarm.behaviour_mut().kademlia.get_record(&key, Quorum::One);
                                catch_event(Ok(query_id), event_bus.subscribe(), responder, |response| {
                                    match response {
                                        KademliaResponse::GetRecord(result) => Some(result),
                                        _ => None
                                    }
                                });
                            },
                        }
                    },
                    Poll::Ready(None) => return Poll::Ready(Ok::<(), String>(())),
                    Poll::Pending => break,
                }
            }
            
            loop {
                match swarm.poll_next_unpin(cx) {
                    Poll::Ready(Some(event)) => {
                        if let SwarmEvent::NewListenAddr { address, .. } = event {
                            println!("Listening on {:?}", address);
                        }
                    }
                    Poll::Ready(None) => return Poll::Ready(Ok::<(), String>(())),
                    Poll::Pending => break,
                }
            }
            Poll::Pending
        }));

        (command_bus, local_peer_id)
    }
}