use std::sync::Arc;

use serde::Serialize;

pub mod http_server;
pub mod messages;
pub mod tcp_server;

use messages::Message;
use tokio::sync::RwLock;

#[derive(Debug, Default, Serialize)]
pub struct AppState {
    pub content: Vec<String>,
    pub cursor: (usize, usize),
    pub messages: Vec<Message>,

    #[serde(skip_serializing)]
    pub ws_clients: Vec<tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>>,
}

pub type SharedState = Arc<RwLock<AppState>>;
