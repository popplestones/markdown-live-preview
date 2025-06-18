use std::sync::Arc;

use markdown_live_preview::{AppState, http_server, tcp_server};
use tokio::sync::RwLock;

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
