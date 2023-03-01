use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    routing::post,
    routing::put,
    Router,
};
use hex::encode;
use std::str::from_utf8;
use sha3::{Digest, Sha3_256};
use futures::{sink::SinkExt, stream::StreamExt};
use redis;
use redis::Commands;
use std::env;
use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use serde::{de::value::StringDeserializer, Deserialize, Serialize};

use tower_http::cors::{Any, CorsLayer};

use axum::Json;

#[derive(Clone)]
struct AppState {
    connections_string: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Segment {
    label: String,
    time: u32,
    sound: bool,
}

#[derive(Serialize, Deserialize)]
struct TimerResponse {
    // Save in Redis
    segments: Vec<Segment>,
    name: String,
    repeat: bool,
    start_at: u64,
}

#[derive(Serialize, Deserialize)]
struct TimerRequest {
    // Get from User
    segments: Vec<Segment>,
    name: String,
    password: String,
    repeat: bool,
    start_at: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Timer {
    // Return after TimerRequest
    segments: Vec<Segment>,
    name: String,
    repeat: bool,
    start_at: u64,
    password: String,
    id: String, // 5 random chars
}

async fn get_redis(key: String) -> String {
    let client = redis::Client::open(env::var("REDIS_STRING").expect("REDIS_STRING is not set"));
    let mut con = client.unwrap().get_connection().unwrap();
    let value: String = con.get(key).unwrap();
    value
}

async fn set_redis(key: String, value: String) {
    let client = redis::Client::open(env::var("REDIS_STRING").expect("REDIS_STRING is not set"));
    let mut con = client.unwrap().get_connection().unwrap();
    let _: () = con.set(key, value).unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
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

async fn create_timer(Json(request): Json<TimerRequest>) -> Json<Timer> {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(request.name.clone());

    // read hash digest
    let id_hash: String = hex::encode(&hasher.finalize()[0..5]);

    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        password: request.password,
        id: id_hash,
    };

    set_redis(timer.id.clone(), serde_json::to_string(&timer).unwrap()).await;

    Json(timer)
}

async fn update_timer(request: TimerRequest) {}

async fn delete_timer() -> impl IntoResponse {
    String::from("Not Implemented")
}

// async fn get_timer() -> Json<Timer> {
//     String::from("Not Implemented")
// }

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/api/ws", get(ws_handler))
        .route("/api/timer", post(create_timer).delete(delete_timer))
        .layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
