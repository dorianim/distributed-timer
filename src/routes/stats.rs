use crate::SharedState;
use axum::extract::State;
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;

#[derive(Serialize)]
struct ServerStats {
    server_up_since: u128,
    uptime: String,
    redis_memory: u64,
    num_keys: u64,
    connections: i64,
}

async fn server_stats(State(state): State<SharedState>) -> Json<ServerStats> {
    let client = state.redis_client.clone();
    let mut con = client.get_async_connection().await.unwrap();

    //Redis Memory
    let res: String = redis::cmd("INFO")
        .arg("MEMORY")
        .query_async(&mut con)
        .await
        .unwrap();
    let mut redis_memory: u64 = 0;

    for r in res.split("\n") {
        if r.contains("used_memory:") {
            redis_memory = r.split(":").collect::<Vec<&str>>()[1]
                .to_string()
                .trim()
                .parse()
                .unwrap();
        }
    }

    //Redis Connections
    let res: String = redis::cmd("INFO")
        .arg("CLIENTS")
        .query_async(&mut con)
        .await
        .unwrap();
    let mut connected_clients: i64 = -1;

    for r in res.split("\n") {
        if r.contains("connected_clients:") {
            let right = r.split(":").collect::<Vec<&str>>()[1].trim();
            connected_clients = right.parse().unwrap();
        }
    }

    //Redis Keys in all Databases
    let res: String = redis::cmd("INFO")
        .arg("KEYSPACE")
        .query_async(&mut con)
        .await
        .unwrap();
    let mut num_keys = 0;

    for r in res.split("\n") {
        if r.contains("keys=") {
            num_keys += r.split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .unwrap();
        }
    }

    // uptime since start
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        - state.server_up_since;

    let uptime_readable = format!(
        "{} days, {} hours, {} minutes, {} seconds",
        uptime / 86400000,
        (uptime % 86400000) / 3600000,
        (uptime % 3600000) / 60000,
        (uptime % 60000) / 1000
    );

    Json(ServerStats {
        server_up_since: state.server_up_since,
        uptime: uptime_readable,
        redis_memory: redis_memory,
        num_keys: num_keys,
        connections: connected_clients,
    })
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(server_stats))
}
