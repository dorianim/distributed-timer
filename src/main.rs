use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;


async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(
    mut socket: WebSocket,
) {

    let (mut sender, mut receiver) = socket.split();
    // send a message to the client
    sender
        .send("Hello from the server".into())
        .await
        .unwrap();

    // receive messages from the client
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(msg) = msg {
            println!("Received message: {:?}", msg);
        }
    }

    println!("Client disconnected");
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/ws", get(ws_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
