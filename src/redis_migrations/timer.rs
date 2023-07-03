use serde::Deserialize;

use crate::repository::{Timer, TimerMetadata};

use super::display_options::RedisDisplayOptions;
use super::segment::RedisSegment;
use super::timer_metadata::RedisTimerMetadata;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisTimer {
    V1(TimerV1),
    V0(TimerV0),
}

impl Into<Timer> for RedisTimer {
    fn into(self) -> Timer {
        match self {
            RedisTimer::V0(t) => t.into(),
            RedisTimer::V1(t) => t.into(),
        }
    }
}

/// === V1 ===
#[derive(Deserialize, Clone)]
pub struct TimerV1 {
    pub segments: Vec<RedisSegment>,
    pub repeat: bool,
    pub display_options: Option<RedisDisplayOptions>,
    pub start_at: u64,
    pub stop_at: Option<u64>,
    pub password: String,
    pub id: String,
    pub metadata: RedisTimerMetadata,
}

impl Into<Timer> for TimerV1 {
    fn into(self) -> Timer {
        Timer {
            segments: self.segments.into_iter().map(|s| s.into()).collect(),
            repeat: self.repeat,
            display_options: self.display_options.into(),
            start_at: self.start_at,
            stop_at: self.stop_at,
            password: self.password,
            id: self.id,
            metadata: self.metadata.into(),
        }
    }
}

/// === V0 ===
#[derive(Deserialize, Clone)]
pub struct TimerV0 {
    pub segments: Vec<RedisSegment>,
    pub repeat: bool,
    pub display_options: Option<RedisDisplayOptions>,
    pub start_at: u64,
    pub stop_at: Option<u64>,
    pub password: String,
    pub id: String,
}

impl Into<Timer> for TimerV0 {
    fn into(self) -> Timer {
        Timer {
            segments: self.segments.into_iter().map(|s| s.into()).collect(),
            repeat: self.repeat,
            display_options: self.display_options.into(),
            start_at: self.start_at,
            stop_at: self.stop_at,
            password: self.password,
            id: self.id,
            metadata: TimerMetadata::default(),
        }
    }
}
