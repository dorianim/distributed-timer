use axum::{extract::State, routing::get, Json, Router};

use crate::{InstanceProperties, SharedState};

async fn get_instance_properties(State(state): State<SharedState>) -> Json<InstanceProperties> {
    Json(state.instance_properties.clone())
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(get_instance_properties))
}
