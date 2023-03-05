use crate::SharedState;
use crate::{Segment, Timer};
use axum::extract::{Path, State};
use axum::http::{Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{get, post, put};
use axum::Router;
use axum::{
    headers::authorization::{Authorization, Bearer},
    response::IntoResponse,
    Json, TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Sha3_256;
use std::str;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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

#[derive(Serialize, Deserialize, Debug)]
struct TimerCreationResponse {
    timer: TimerResponse,
    token: String,
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

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    id: String,
    exp: usize,
    iss: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token: String,
}

async fn auth_middleware<B>(
    State(state): State<SharedState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut validation = Validation::new(Algorithm::default());
    validation.set_issuer(&["de:itsblue:synced-timer"]);
    validation.validate_exp = false;

    let token = decode::<Claims>(
        auth.token(),
        &DecodingKey::from_secret(state.jwt_key.as_ref()),
        &validation,
    );

    if token.is_err() || request.uri().to_string() != format!("/{}", token.unwrap().claims.id) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
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

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_ref(), &salt)
        .unwrap()
        .to_string()
}

fn check_password_hash(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();
    Argon2::default()
        .verify_password(password.as_ref(), &parsed_hash)
        .is_ok()
}

async fn get_timer_from_redis(
    id: String,
    redis: &mut ConnectionManager,
) -> Result<Timer, StatusCode> {
    let timer = &redis.get::<String, String>(id).await;

    if timer.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let timer: Timer = serde_json::from_str(timer.as_ref().unwrap()).unwrap();
    Ok(timer)
}

fn create_jwt(id: String, key: &str) -> String {
    let claims = Claims {
        id: id,
        exp: 0,
        iss: "de:itsblue:synced-timer".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_ref()),
    )
    .unwrap()
}

async fn create_token(
    State(state): State<SharedState>,
    Json(request): Json<TokenRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {
    let mut redis = state.as_ref().redis.clone();

    let timer = get_timer_from_redis(request.id.clone(), &mut redis).await?;

    if !check_password_hash(&request.password, &timer.password) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_jwt(request.id, &state.jwt_key);

    Ok(Json(TokenResponse { token }))
}

async fn create_timer(
    State(state): State<SharedState>,
    Json(request): Json<TimerRequest>,
) -> Result<Json<TimerCreationResponse>, StatusCode> {
    // Timer already exists
    let mut redis = state.as_ref().redis.clone();
    let id = generate_id(request.name.clone());
    if redis.exists::<String, bool>(id.clone()).await.unwrap() {
        return Err(StatusCode::CONFLICT);
    }

    let password = hash_password(&request.password);

    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        password,
        id,
    };

    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .await
        .unwrap();

    let token = create_jwt(timer.id.clone(), &state.jwt_key);

    Ok(Json(TimerCreationResponse {
        timer: timer.into(),
        token: token,
    }))
}

async fn get_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<TimerResponse>, StatusCode> {
    let mut redis = state.as_ref().redis.clone();
    let timer = get_timer_from_redis(id, &mut redis).await?;
    Ok(Json(timer.into()))
}

async fn update_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(request): Json<TimerRequest>,
) -> Result<Json<TimerResponse>, StatusCode> {
    let mut redis = state.as_ref().redis.clone();

    let old_timer: Timer = get_timer_from_redis(id, &mut redis).await?;

    let timer = Timer {
        segments: request.segments,
        name: request.name,
        repeat: request.repeat,
        start_at: request.start_at,
        ..old_timer
    };

    redis
        .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(&timer).unwrap())
        .await
        .unwrap();

    Ok(Json(timer.into()))
}

async fn delete_timer(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut redis = state.as_ref().redis.clone();
    if redis.del::<String, ()>(id).await.is_err() {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::OK
    }
}

pub fn routes(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/:id", put(update_timer).delete(delete_timer))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
        .route("/token", post(create_token))
        .route("/", post(create_timer))
        .route("/:id", get(get_timer))
}
