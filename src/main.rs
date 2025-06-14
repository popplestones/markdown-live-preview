use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::messages::Message;

mod http_server;
mod messages;
mod tcp_server;

#[derive(Debug, Default, Serialize)]
struct AppState {
    pub content: Vec<String>,
    pub cursor: (usize, usize),
    pub messages: Vec<Message>,

    #[serde(skip_serializing)]
    pub ws_clients: Vec<tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>>,
}

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = Arc::new(RwLock::new(AppState::default()));

    // Spawn TCP message listener and HTTP preview server
    tokio::spawn(tcp_server::run_tcp_listener(state.clone()));
    tokio::spawn(http_server::run_http_server(state.clone()));

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
