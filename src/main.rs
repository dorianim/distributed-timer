use axum::{Router, http::{Request},response::{Response},};
use serde::{Deserialize, Serialize};

use std::{env, time::Duration};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{TraceLayer};
use tracing::Span;
mod routes;

#[derive(Serialize, Deserialize, Clone)]
pub struct Segment {
    label: String,
    time: u32,
    sound: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Timer {
    // Return after TimerRequest
    segments: Vec<Segment>,
    name: String,
    repeat: bool,
    start_at: u64,
    password: String,
    id: String, // 5 random chars
}

type SharedState = Arc<AppState>;
pub struct AppState {
    redis: RwLock<redis::Connection>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let client = redis::Client::open(env::var("REDIS_STRING").expect("REDIS_STRING is not set"));

    let state = Arc::new(AppState {
        redis: RwLock::new(client.unwrap().get_connection().unwrap()),
    });

    //let state = SharedState::default();

    let app = Router::new()
        .nest("/api/ws", routes::ws::routes())
        .nest("/api/timer", routes::timer::routes())
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .on_request(|_request: &Request<_>, _span: &Span| {
                    println!("Request {}", _request.uri());
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    println!("Response {}, {}ms", _response.status(), _latency.as_millis());
                }))
        .with_state(Arc::clone(&state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
