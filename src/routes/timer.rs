use crate::SharedState;
use crate::{Segment, Timer};
use axum::extract::State;
use axum::routing::post;
use axum::Router;
use axum::{response::IntoResponse, Json};
use redis::Commands;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Sha3_256;
use std::str;

const alphanumeric: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";

#[derive(Serialize, Deserialize)]
struct TimerResponse {
    // Save in Redis
    segments: Vec<Segment>,
    name: String,
    repeat: bool,
    start_at: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TimerRequest {
    // Get from User
    segments: Vec<Segment>,
    name: String,
    password: String,
    repeat: bool,
    start_at: u64,
}

pub async fn create_timer(
    State(state): State<SharedState>,
    Json(request): Json<TimerRequest>,
) -> Json<Timer> {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(request.name.clone());

    // read hash digest
    let id_hash_u8: &[u8] = &hasher.finalize()[0..5];
    let mut id_hash = String::new();
    for i in id_hash_u8 {
        id_hash.push(alphanumeric[(*i as usize) % 62] as char);
    }

    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        password: request.password,
        id: id_hash,
    };

    let mut redis = state.as_ref().redis.write().unwrap();
    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .unwrap();

    Json(timer)
}

pub async fn update_timer(State(state): State<SharedState>, Json(request): Json<TimerRequest>) {}

pub async fn delete_timer(State(state): State<SharedState>) -> impl IntoResponse {
    String::from("Not Implemented")
}

/*async fn get_timer(State(state): State<RedisState>) -> Json<Timer> {
    todo!()
}*/

pub fn routes() -> Router<SharedState> {
    Router::new().route(
        "/",
        post(create_timer).put(update_timer).delete(delete_timer),
    )
}
