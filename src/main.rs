use axum::Router;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use std::{env, sync::RwLock};
use tower_http::cors::{Any, CorsLayer};

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
        .with_state(Arc::clone(&state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
