use axum::{
    Json, Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::{Html, IntoResponse},
    routing::get,
};
use comrak::markdown_to_html;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{SharedState, messages::OutgoingMessage};
const GITHUB_MARKDOWN_CSS: &str = include_str!("../assets/github-markdown-dark.css");

pub async fn run_http_server(state: SharedState) -> anyhow::Result<()> {
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🌐 Serving preview at http://localhost:3000");

    if webbrowser::open("http://localhost:3000").is_ok() {
        println!("🚀 Browser launched");
    }

    axum::serve(listener, app).await?;

    Ok(())
}

pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(serve_preview))
        .route("/ws", get(ws_handler))
        .route("/messages", get(get_state))
        .with_state(state)
}
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(ws: WebSocket, state: SharedState) {
    let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        tokio::sync::mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    let (mut sender, mut _receiver) = ws.split();

    let send_task = tokio::spawn(async move {
        tokio::pin!(rx);
        while let Some(msg) = rx.next().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Register this sender in the shared state

    {
        let mut state = state.write().await;
        state.ws_clients.push(tx);
    }

    // Optionally: handle incoming message here (not needed in our case)

    send_task.await.ok();
}
async fn serve_preview(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().await;

    let mut options = comrak::Options::default();
    options.extension.alerts = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some("---".into());
    options.extension.alerts = true;
    options.render.unsafe_ = true;
    let lines = inject_cursor(state.content.clone(), state.cursor);
    let html = markdown_to_html(&lines.join("\n"), &options);

    let live_js = include_str!("../static/live.js");

    let full = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Live Preview</title>
  <style>{GITHUB_MARKDOWN_CSS}</style>
  <style>
    body {{
      margin: 2rem auto;
      max-width: 800px;
      padding: 0 1rem;
    }}
  </style>
  <script src="https://unpkg.com/morphdom@2.7.5/dist/morphdom-umd.min.js"></script>
  <script>
  {live_js}</script>
</head>
<body class="markdown-body">
<div id="content">
  {html}
  </div>
</body>
</html>"#
    );
    Html(full)
}

async fn get_state(
    axum::extract::State(state): axum::extract::State<SharedState>,
) -> impl IntoResponse {
    let messages = {
        let state = state.read().await;
        state.messages.clone()
    };
    Json(messages)
}
fn inject_cursor(mut lines: Vec<String>, cursor: (usize, usize)) -> Vec<String> {
    use html_escape::encode_text;

    if cursor.0 < lines.len() {
        let line = &mut lines[cursor.0];
        if cursor.1 <= line.len() {
            let (before, after) = line.split_at(cursor.1);

            let (cursor_html, rest) = if let Some(cursor_char) = after.chars().next() {
                let char_len = cursor_char.len_utf8();
                (
                    format!(
                        "<span id=\"cursor\" class=\"cursor-position\">{}</span>",
                        encode_text(&cursor_char.to_string())
                    ),
                    &after[char_len..],
                )
            } else {
                (
                    r#"<span id="cursor" class="cursor-position"> </span>"#.to_string(),
                    "",
                )
            };

            *line = format!(
                "{}{}{}",
                encode_text(before),
                cursor_html,
                encode_text(rest)
            );
        }
    } else {
        // Handle case where cursor is on a new line beyond existing lines
        lines.push(r#"<span id="cursor" class="cursor-position"> </span>"#.to_string());
    }

    lines
}

pub async fn render_and_broadcast(state: &SharedState) {
    let state_guard = state.read().await;

    let mut options = comrak::Options::default();
    options.extension.alerts = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.front_matter_delimiter = Some("---".into());
    options.render.unsafe_ = true;

    let lines = inject_cursor(state_guard.content.clone(), state_guard.cursor);
    let html = markdown_to_html(&lines.join("\n"), &options);

    let wrapped = format!(r#"<div id="content">{html}</div>"#);
    let msg = OutgoingMessage::FullRender { html: wrapped };
    let json = match serde_json::to_string(&msg) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("❌ Failed to serialize message: {e}");
            return;
        }
    };

    for client in &state_guard.ws_clients {
        let _ = client.send(axum::extract::ws::Message::Text(json.clone().into()));
    }
}
