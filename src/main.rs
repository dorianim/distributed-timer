use axum::{http::Request, response::Response, Router};
use tower_http::catch_panic::CatchPanicLayer;

use std::sync::Arc;
use std::{env, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Span;
mod color;
mod models;
mod repository;
mod routes;

use models::*;

use crate::repository::Repository;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let instance_properties = InstanceProperties {
        demo: env::var("INSTANCE_DEMO").unwrap_or("false".to_owned()) == "true",
        donation: env::var("INSTANCE_DONATION_PAYPAL")
            .map(|id| Some(vec![DonationMethod::PayPal(id)]))
            .unwrap_or(None),
        s3_host: env::var("S3_HOST").unwrap_or("".to_owned()),
    };
    let jwt_key = env::var("JWT_KEY").expect("JWT_KEY is not set");
    let redis_string = env::var("REDIS_STRING").expect("REDIS_STRING is not set");

    let repository = Repository::new(redis_string).await;

    let state: SharedState = Arc::new(AppState {
        repository,
        jwt_key,
        instance_properties,
    });

    let app = Router::new()
        .nest("/api/ws", routes::ws::routes())
        .nest("/api/timer", routes::timer::routes(state.clone()))
        .nest("/api/instance", routes::instance::routes())
        .fallback(routes::web::web_assets)
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
