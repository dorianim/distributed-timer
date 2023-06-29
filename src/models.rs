use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

//main.rs
fn default_zero() -> u32 {
    0
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    label: String,
    pub time: u32,
    sound: bool,
    color: Option<Color>,
    #[serde(default = "default_zero")]
    pub count_to: u32,
    pub sounds: Vec<Sound>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sound {
    pub filename: String,
    pub trigger_time: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PreStartBehaviour {
    ShowZero,
    RunNormally,
}

impl Default for PreStartBehaviour {
    fn default() -> Self {
        PreStartBehaviour::ShowZero
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DisplayOptions {
    #[serde(default)]
    clock: bool,
    #[serde(default)]
    pre_start_behaviour: PreStartBehaviour,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Timer {
    // Return after TimerRequest
    pub segments: Vec<Segment>,
    pub repeat: bool,
    pub display_options: Option<DisplayOptions>,
    pub start_at: u64,
    pub stop_at: Option<u64>,
    pub password: String,
    pub id: String, // 5 random chars
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum DonationMethod {
    PayPal(String),
}

#[derive(Serialize, Clone)]
pub struct InstanceProperties {
    pub demo: bool,
    pub donation: Option<Vec<DonationMethod>>,
}

pub type SharedState = Arc<AppState>;
pub struct AppState {
    pub redis: redis::aio::ConnectionManager,
    pub jwt_key: String,
    pub instance_properties: InstanceProperties,
    pub redis_task_rx: broadcast::Receiver<Timer>,
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
