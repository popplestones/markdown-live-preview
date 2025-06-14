# 🦀 markdown-live-preview

A Rust-powered live Markdown preview server that integrates with Neovim via TCP.  
Renders GitHub-flavored Markdown (GFM) using [Comrak](https://github.com/kivikakk/comrak), served via [Axum](https://github.com/tokio-rs/axum), and controlled in real time from an [md-live-preview.nvim](https://github.com/popplestones/md-live-preview.nvim) plugin.

> [!NOTE]
> This project is designed to be paired with a Neovim plugin that streams buffer changes and cursor position.

---

## 📦 Features

| Feature              | Status | Notes |
|----------------------|--------|-------|
| Live buffer preview  | ✅     | Renders latest Markdown buffer to HTML |
| Cursor tracking      | ✅     | Receives cursor position updates |
| Incremental updates  | ✅     | Only sends changed lines |
| GFM rendering        | ✅     | Uses `comrak` with alerts, tables, etc. |
| Local TCP interface  | ✅     | Neovim pushes updates to `127.0.0.1:3001` |
| HTML server          | ✅     | Axum serves preview on `http://localhost:3000` |
| Auto-launch browser  | ✅     | Opens system browser on start |
| WebSocket live reload | 🚧    | Planned |

---


## 🚀 Getting Started

### 🔧 Prerequisites

- Rust (1.70+ recommended)
- Neovim 0.9+
- `md-live-preview.nvim` installed and configured

### 🛠 Build and run

```bash
git clone https://github.com/popplestones/markdown-live-preview
cd markdown-live-preview
cargo build --release
./target/release/markdown-live-preview
```

### 🔌 Communication Protocol

The server listens on `127.0.0.1:3001` for newline-delimited JSON messages.

#### ✉️ Incoming message format

```json
{
  "event": "init",
  "data": {
    "content": ["# Hello", "world!"],
    "cursor": [1, 0]
  }
}
```

Supported `event` types:

| Event         | Payload Structure                                |
|----------------|--------------------------------------------------|
| init           | { content: Vec<String>, cursor: (usize, usize) } |
| buffer_change  | { line: usize, new_text: String }                |
| cursor_moved   | { cursor: (usize, usize) }                       |


> [!TIP]
> Lines are indexed from zero.

## 🌍 Web Interface

 - URL: [http://localhost:3000](http://localhost:3000)
 - Automatically opens in the default browser on launch
 - Reflects live updates from Neovim

## 🛠 Architecture

```text
[ Neovim (plugin) ] --> TCP (127.0.0.1:3001) --> [ markdown-live-preview ]
                                              --> Axum --> HTML preview (localhost:3000)
```

## 🧪 Debugging

You can log incoming messages using:

```bash
RUST_LOG=debug ./target/release/markdown-live-preview
```

> [!WARNING]
> Always restart the server if you change port bindings or encounter connection issues.

## 📜 License

MIT © Shane Poppleton

## 🙏 Acknowledgments

- [Comrak](https://github.com/kivikakk/comrak)
- [Axum](https://github.com/tokio-rs/axum)
- [Lua TCP with vim.loop](https://neovim.io/)
