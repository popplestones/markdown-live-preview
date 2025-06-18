use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};

use crate::{SharedState, http_server::render_and_broadcast, messages::Message};

pub async fn run_tcp_listener(state: SharedState) -> anyhow::Result<()> {
    run_tcp_listener_on("127.0.0.1:3001", state).await?;
    Ok(())
}

pub async fn run_tcp_listener_on(addr: &str, state: SharedState) -> anyhow::Result<SocketAddr> {
    let listener = TcpListener::bind(addr).await?;
    let actual_addr = listener.local_addr()?;
    println!("🔌 Listening for buffer updates on {actual_addr}");

    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let shared_state = state.clone();

            tokio::spawn(async move {
                let reader = BufReader::new(stream);
                let mut lines = reader.lines();

                while let Ok(Some(line)) = lines.next_line().await {
                    match serde_json::from_str::<Message>(&line) {
                        Ok(msg) => {
                            let mut state = shared_state.write().await;

                            match &msg {
                                Message::Init(m) => {
                                    state.content = m.content.clone();
                                    state.cursor = m.cursor;
                                }
                                Message::BufferChange(m) => {
                                    if m.line < state.content.len() {
                                        state.content[m.line] = m.new_text.clone();
                                    } else {
                                        state.content.push(m.new_text.clone());
                                    }
                                }
                                Message::CursorMoved(m) => {
                                    state.cursor = m.cursor;
                                }
                            }
                            state.messages.push(msg.clone());
                            drop(state);
                            render_and_broadcast(&shared_state).await;
                        }
                        Err(e) => eprintln!("❌ Error parsing message: {e}"),
                    }
                }
            });
        }
    });
    Ok(actual_addr)
}
