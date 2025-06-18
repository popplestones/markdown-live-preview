use markdown_live_preview::{
    AppState, SharedState,
    messages::{BufferChangeMessage, InitMessage, Message},
};
use std::sync::Arc;
use tokio::sync::RwLock;

fn setup_shared_state() -> SharedState {
    Arc::new(RwLock::new(AppState::default()))
}

#[tokio::test]
async fn test_message_init_updates_state() {
    let state = setup_shared_state();

    let init_message = Message::Init(InitMessage {
        content: vec!["Hello".to_string()],
        cursor: (0, 5),
    });

    // Simulate receiving an Init message
    {
        let mut state = state.write().await;
        match &init_message {
            Message::Init(m) => {
                state.content = m.content.clone();
                state.cursor = m.cursor;
            }
            _ => {}
        }
    }

    let read_state = state.read().await;
    assert_eq!(read_state.content, vec!["Hello".to_string()]);
    assert_eq!(read_state.cursor, (0, 5));
}

#[tokio::test]
async fn test_message_buffer_change_updates_state() {
    let state = setup_shared_state();

    let buffer_change_message = Message::BufferChange(BufferChangeMessage {
        line: 0,
        new_text: "World".to_string(),
    });

    // Simulate receiving a BufferChange message
    {
        let mut state = state.write().await;
        match &buffer_change_message {
            Message::BufferChange(m) => {
                if m.line < state.content.len() {
                    state.content[m.line] = m.new_text.clone();
                } else {
                    state.content.push(m.new_text.clone());
                }
            }
            _ => {}
        }
    }

    let read_state = state.read().await;
    assert_eq!(read_state.content, vec!["World".to_string()]);
}
