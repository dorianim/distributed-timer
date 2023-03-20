use axum::{http::Request, response::Response, Router};
use color::Color;
use serde::{Deserialize, Serialize};
use tower_http::catch_panic::CatchPanicLayer;

use std::sync::Arc;
use std::{env, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
mod color;
mod routes;

use tokio::sync::broadcast;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    label: String,
    time: u32,
    sound: bool,
    color: Option<Color>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Timer {
    // Return after TimerRequest
    pub segments: Vec<Segment>,
    pub repeat: bool,
    pub start_at: u64,
    pub password: String,
    pub id: String, // 5 random chars
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "data")]
enum DonationMethod {
    PayPal(String),
}

#[derive(Serialize, Clone)]
struct InstanceProperties {
    demo: bool,
    donation: Option<Vec<DonationMethod>>,
}

type SharedState = Arc<AppState>;
pub struct AppState {
    redis: redis::aio::ConnectionManager,
    jwt_key: String,
    instance_properties: InstanceProperties,
    redis_task_rx: broadcast::Receiver<Timer>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let redis_string = env::var("REDIS_STRING").expect("REDIS_STRING is not set");
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY is not set");
    let client = redis::Client::open(redis_string.to_owned()).expect("Could not connect to redis");
    let manager = redis::aio::ConnectionManager::new(client.clone())
        .await
        .unwrap();

    let instance_properties = InstanceProperties {
        demo: env::var("INSTANCE_DEMO").unwrap_or("false".to_owned()) == "true",
        donation: env::var("INSTANCE_DONATION_PAYPAL")
            .map(|id| Some(vec![DonationMethod::PayPal(id)]))
            .unwrap_or(None),
    };

    let (redis_task_tx, redis_task_rx) = broadcast::channel::<Timer>(10);

    let state: SharedState = Arc::new(AppState {
        redis: manager.clone(),
        jwt_key,
        instance_properties,
        redis_task_rx,
    });

    routes::ws::spawn_global_redis_listener_task(manager, client, redis_task_tx);

    let app = Router::new()
        .nest("/api/ws", routes::ws::routes())
        .nest("/api/timer", routes::timer::routes(state.clone()))
        .nest("/api/instance", routes::instance::routes())
        .fallback(routes::client::client_assets)
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    println!("Request {} {}", request.method(), request.uri());
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    println!(
                        "Response {}, {}ms",
                        _response.status(),
                        _latency.as_millis()
                    );
                }),
        )
        .layer(CatchPanicLayer::new())
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server started on port 3000");
}
