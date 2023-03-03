use axum::{http::Request, response::Response, Router};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::{env, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
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
    pub segments: Vec<Segment>,
    pub name: String,
    pub repeat: bool,
    pub start_at: u64,
    pub password: String,
    pub id: String, // 5 random chars
}

type SharedState = Arc<AppState>;
pub struct AppState {
    redis: redis::aio::ConnectionManager,
    jwt_key: String,
    redis_string: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let redis_string = env::var("REDIS_STRING").expect("REDIS_STRING is not set");
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY is not set");
    let client = redis::Client::open(redis_string.to_owned()).unwrap();
    let manager = redis::aio::ConnectionManager::new(client).await.unwrap();

    let state = Arc::new(AppState {
        redis: manager,
        jwt_key,
        redis_string,
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
                    println!(
                        "Response {}, {}ms",
                        _response.status(),
                        _latency.as_millis()
                    );
                }),
        )
        .with_state(Arc::clone(&state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
