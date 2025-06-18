use markdown_live_preview::SharedState;
use markdown_live_preview::messages::BufferChangeMessage;
use markdown_live_preview::messages::Message;
use markdown_live_preview::tcp_server::run_tcp_listener_on;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
async fn test_tcp_listener_receives_data() {
    let state = SharedState::default();

    // Let the function bind the port and give us the actual address
    let addr = run_tcp_listener_on("127.0.0.1:0", state.clone())
        .await
        .expect("listener failed to start");

    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(addr)
        .await
        .expect("Failed to connect to server");

    let msg = Message::BufferChange(BufferChangeMessage {
        line: 0,
        new_text: "Hello from test".into(),
    });

    let json = serde_json::to_string(&msg).unwrap();
    stream
        .write_all(format!("{json}\n").as_bytes())
        .await
        .expect("failed to send");

    tokio::time::sleep(Duration::from_millis(100)).await;

    let guard = state.read().await;
    assert_eq!(guard.content, vec!["Hello from test"]);
    assert_eq!(guard.messages.len(), 1);
}
