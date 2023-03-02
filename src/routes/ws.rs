use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::SinkExt;
use futures::StreamExt;

use crate::SharedState;

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    // send a message to the client
    sender.send("Hello from the server".into()).await.unwrap();

    // receive messages from the client
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(msg) = msg {
            println!("Received message: {:?}", msg);
        }
    }

    println!("Client disconnected");
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(ws_handler))
}
