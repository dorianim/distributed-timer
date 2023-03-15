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

use futures::StreamExt;
use redis::AsyncCommands;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

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
    redis_client: redis::Client,
    jwt_key: String,
    instance_properties: InstanceProperties,
    redis_task_rx: broadcast::Receiver<Timer>,
}

fn spawn_global_redis_listener_task(
    mut redis: redis::aio::ConnectionManager,
    redis_client: redis::Client,
    redis_task_tx: broadcast::Sender<Timer>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut connection = redis_client.get_async_connection().await.unwrap();
        let _: () = redis::cmd("CONFIG")
            .arg("SET")
            .arg("notify-keyspace-events")
            .arg("KEA")
            .query_async(&mut connection)
            .await
            .unwrap();

        let mut pubsub = connection.into_pubsub();

        pubsub
            .psubscribe("__keyspace@0__:*")
            .await
            .expect("Failed to subscribe to redis channel");

        let mut pubsub = pubsub.into_on_message();

        while let Some(msg) = pubsub.next().await {
            println!("Updated! {:?}", msg);
            let timer_id = msg.get_channel_name().split(":").last().unwrap();

            let timer_str = &redis
                .get::<String, String>(String::from(timer_id))
                .await
                .unwrap();
            let timer: Timer = serde_json::from_str(timer_str).unwrap();

            // Broadcast to all listeners
            redis_task_tx.send(timer).unwrap();
        }
    })
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

    let (redis_task_tx, mut redis_task_rx) = broadcast::channel::<Timer>(10);

    spawn_global_redis_listener_task(manager.clone(), client.clone(), redis_task_tx);

    let state: SharedState = Arc::new(AppState {
        redis: manager,
        redis_client: client,
        jwt_key,
        instance_properties,
        redis_task_rx,
    });

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
