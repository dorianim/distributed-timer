use crate::SharedState;
use crate::{Segment, Timer};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use axum::{response::IntoResponse, Json};
use redis::Commands;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Sha3_256;
use std::str;

const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";

#[derive(Serialize, Deserialize)]
struct TimerResponse {
    segments: Vec<Segment>,
    name: String,
    repeat: bool,
    start_at: u64,
    id: String,
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

fn sha3_from_string(string: String) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(string);
    hasher.finalize().to_vec()
}

fn generate_id(name: String) -> String {
    // Generate id
    let id_hash_u8: Vec<u8> = sha3_from_string(name);
    let mut id_hash = String::new();
    for i in 0..5 {
        id_hash.push(ALPHANUMERIC[(id_hash_u8[i] as usize) % 62] as char);
    }
    id_hash
}

async fn create_timer(
    State(state): State<SharedState>,
    Json(request): Json<TimerRequest>,
) -> Json<Timer> {
    // Start password hash
    let password_hash_u8 = sha3_from_string(request.password);
    let id_hash = generate_id(request.name.clone());
    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        password: hex::encode(password_hash_u8),
        id: id_hash,
    };

    let mut redis = state.as_ref().redis.write().unwrap();
    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .unwrap();

    Json(timer)
}

async fn get_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<TimerResponse> {
    let mut redis = state.as_ref().redis.write().unwrap();
    let timer: TimerResponse =
        serde_json::from_str(&redis.get::<String, String>(id).unwrap()).unwrap();
    Json(timer)
}

async fn update_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(request): Json<TimerRequest>,
) -> StatusCode {
    let mut redis = state.as_ref().redis.write().unwrap();
    let password_hash_u8 = sha3_from_string(request.password);

    let old_timer: Timer =
        serde_json::from_str(&redis.get::<String, String>(id.clone()).unwrap()).unwrap();
    if old_timer.password != hex::encode(password_hash_u8.clone()) {
        return StatusCode::UNAUTHORIZED;
    }
    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        password: hex::encode(password_hash_u8),
        id: id,
    };
    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .unwrap();
    StatusCode::OK
}

async fn delete_timer(State(state): State<SharedState>) -> impl IntoResponse {
    String::from("Not Implemented")
}

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/", post(create_timer).delete(delete_timer))
        .route("/:id", get(get_timer).put(update_timer))
}
