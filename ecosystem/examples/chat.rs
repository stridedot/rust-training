use std::{net::SocketAddr, sync::Arc};

use dashmap::DashMap;
use futures_util::{SinkExt as _, StreamExt, stream::SplitStream};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer as _, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

const MAX_MESSAGE_SIZE: usize = 1024;

struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(console).init();

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Starting chat server on {}", addr);

    let state = Arc::new(AppState::default());

    loop {
        let (stream, addr) = listener.accept().await?;
        tracing::info!("Accepted new connection from {:?}", addr);

        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(e) = handle_client(state, addr, stream).await {
                tracing::error!("Error handling client {:?}: {:?}", addr, e);
            }
        });
    }
}

async fn handle_client(
    state: Arc<AppState>,
    addr: SocketAddr,
    stream: TcpStream,
) -> anyhow::Result<()> {
    let mut framed = Framed::new(stream, LinesCodec::new());
    framed.send("Enter your username:").await?;
    let username = match framed.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    let mut peer = state.add_peer(addr, username, framed).await;
    state
        .broadcast(addr, Arc::new(Message::user_joined(&peer.username)))
        .await?;

    while let Some(message) = peer.stream.next().await {
        let message = match message {
            Ok(message) => message,
            Err(e) => {
                tracing::error!("Failed to receive message from {:?}: {:?}", addr, e);
                break;
            }
        };

        let message = Message::chat(&peer.username, &message);
        state.broadcast(addr, Arc::new(message)).await?;
    }

    state.peers.remove(&addr);
    state
        .broadcast(addr, Arc::new(Message::user_left(&peer.username)))
        .await?;

    Ok(())
}

#[derive(Default)]
struct AppState {
    peers: DashMap<SocketAddr, Sender<Arc<Message>>>,
}

impl AppState {
    async fn add_peer(
        &self,
        addr: SocketAddr,
        username: String,
        framed: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MESSAGE_SIZE);
        self.peers.insert(addr, tx);

        let (mut stream_tx, stream_rx) = framed.split();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = stream_tx.send(message.to_string()).await {
                    tracing::error!("Failed to send message to {:?}: {:?}", addr, e);
                    break;
                }
            }
        });

        Peer {
            username,
            stream: stream_rx,
        }
    }

    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) -> anyhow::Result<()> {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }

            if let Err(e) = peer.value().send(message.clone()).await {
                tracing::error!("Failed to send message to {:?}: {:?}", peer.key(), e);
                self.peers.remove(peer.key());
            }
        }

        Ok(())
    }
}

enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

impl Message {
    fn user_joined(username: &str) -> Self {
        let content = format!("{} has joined the chat", username);
        Self::UserJoined(content)
    }

    fn user_left(username: &str) -> Self {
        let content = format!("{} has left the chat", username);
        Self::UserLeft(content)
    }

    fn chat(sender: &str, content: &str) -> Self {
        Self::Chat {
            sender: sender.to_string(),
            content: content.to_string(),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::UserJoined(content) => write!(f, "[{} :)]", content),
            Message::UserLeft(content) => write!(f, "[{} :(]", content),
            Message::Chat { sender, content } => write!(f, "<{}>: {}", sender, content),
        }
    }
}
