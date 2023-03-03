use crate::SharedState;
use crate::{Segment, Timer};
use axum::body::{Body, BoxBody};
use axum::extract::{Path, State};
use axum::http::{request, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use axum::{
    headers::authorization::{Authorization, Bearer},
    response::IntoResponse,
    Json, TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Sha3_256;
use std::str;

const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerResponse {
    pub segments: Vec<Segment>,
    pub name: String,
    pub repeat: bool,
    pub start_at: u64,
    pub id: String,
}

impl Into<TimerResponse> for Timer {
    fn into(self) -> TimerResponse {
        TimerResponse {
            segments: self.segments,
            name: self.name,
            repeat: self.repeat,
            start_at: self.start_at,
            id: self.id,
        }
    }
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

#[derive(Serialize, Deserialize)]
struct TokenRequest {
    id: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    id: String,
}

async fn auth_middleware<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth = request
        .headers()
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap();
    let auth = auth.split(" ").collect::<Vec<&str>>()[1];
    let token = decode::<Claims>(
        auth,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );

    if !token.is_ok()
        || request.uri().to_string() != format!("/api/timer/{}", token.unwrap().claims.id)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

async fn create_token(
    State(state): State<SharedState>,
    Json(request): Json<TokenRequest>,
) -> Result<String, StatusCode> {
    // Check Password from ID
    let mut redis = state.as_ref().redis.clone();
    let pw_hash = hex::encode(sha3_from_string(request.password.clone()));
    let timer: Timer = serde_json::from_str(
        &redis
            .get::<String, String>(request.id.clone())
            .await
            .unwrap(),
    )
    .unwrap();
    if pw_hash != timer.password {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let my_claims = Claims {
        id: request.id.clone(),
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    Ok(token)
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
        id_hash.push(ALPHANUMERIC[(id_hash_u8[i] as usize) % 26] as char);
    }
    id_hash
}

async fn create_timer(
    State(state): State<SharedState>,
    Json(request): Json<TimerRequest>,
) -> Result<Json<TimerResponse>, StatusCode> {
    // Timer already exists
    let mut redis = state.as_ref().redis.clone();
    let id_hash = generate_id(request.name.clone());
    if redis.exists::<String, bool>(id_hash.clone()).await.unwrap() {
        return Err(StatusCode::CONFLICT);
    }

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

    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .await
        .unwrap();

    // Set update id
    redis
        .set::<String, u32, ()>(String::from("updated:") + &timer.id, 0)
        .await
        .unwrap();

    let timer_response = TimerResponse {
        segments: timer.segments,
        name: timer.name,
        repeat: timer.repeat,
        start_at: timer.start_at,
        id: timer.id,
    };

    Ok(Json(timer_response))
}

async fn get_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Json<TimerResponse> {
    let mut redis = state.as_ref().redis.clone();
    let timer: TimerResponse =
        serde_json::from_str(&redis.get::<String, String>(id).await.unwrap()).unwrap();
    Json(timer)
}

async fn update_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(request): Json<TimerRequest>,
) -> impl IntoResponse {
    let mut redis = state.as_ref().redis.clone();
    let password_hash_u8 = sha3_from_string(request.password);

    let old_timer: Timer =
        serde_json::from_str(&redis.get::<String, String>(id.clone()).await.unwrap()).unwrap();
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
        .await
        .unwrap();

    redis
        .incr::<String, u32, ()>(String::from("updated:") + &timer.id, 1)
        .await
        .unwrap();
    StatusCode::OK
}

async fn delete_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut redis = state.as_ref().redis.clone();
    redis.del::<String, ()>(id).await.unwrap();
    StatusCode::OK
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/token", post(create_token)).route("/", post(create_timer)).route(
        "/:id",
        get(get_timer).put(update_timer).delete(delete_timer),
    )
}
