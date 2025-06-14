const socket = new WebSocket("ws://localhost:3000/ws");

socket.addEventListener("open", () => {
  console.log("[WS] Connected");
});

socket.addEventListener("message", (event) => {
  try {
    const msg = JSON.parse(event.data);

    if (msg.event === "full_render") {
      const next = document.createElement("div");
      next.innerHTML = msg.html;
      const current = document.getElementById("content");
      const updated = next.querySelector("#content");
      if (current && updated) {
        morphdom(current, updated);

        const cursor = document.getElementById("cursor");
        if (cursor) {
          cursor.scrollIntoView({ behavior: "smooth", block: "center" });
        }
      }
    }
  } catch (e) {
    console.error("[WS] Failed to handle message: ", e);
  }
});

socket.addEventListener("close", () => {
  console.warn("[WS] Connection closed");
});

socket.addEventListener("error", (err) => {
  console.error("[WS] Error: ", err);
});
