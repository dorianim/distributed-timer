use crate::repository::{DisplayOptions, Repository, Segment, Timer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum DonationMethod {
    PayPal(String),
}

#[derive(Serialize, Clone)]
pub struct InstanceProperties {
    pub demo: bool,
    pub donation: Option<Vec<DonationMethod>>,
    pub s3_host: String,
}

pub type SharedState = Arc<AppState>;
pub struct AppState {
    pub repository: Repository,
    pub jwt_key: String,
    pub instance_properties: InstanceProperties,
}

//timer.rs

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerResponse {
    pub segments: Vec<Segment>,
    pub id: String,
    pub repeat: bool,
    pub display_options: DisplayOptions,
    pub start_at: u64,
    pub stop_at: Option<u64>,
}

impl Into<TimerResponse> for Timer {
    fn into(self) -> TimerResponse {
        TimerResponse {
            segments: self.segments,
            id: self.id,
            repeat: self.repeat,
            display_options: self.display_options.unwrap_or(DisplayOptions::default()),
            start_at: self.start_at,
            stop_at: self.stop_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerCreationResponse {
    pub timer: TimerResponse,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimerCreationRequest {
    // Get from User
    pub segments: Vec<Segment>,
    pub id: String,
    pub password: String,
    pub repeat: bool,
    pub start_at: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TimerUpdateRequest {
    // Get from User
    pub segments: Vec<Segment>,
    pub repeat: bool,
    pub display_options: Option<DisplayOptions>,
    pub start_at: u64,
    pub stop_at: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub exp: usize,
    pub iss: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub token: String,
}
