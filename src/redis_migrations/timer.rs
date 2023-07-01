use serde::Deserialize;

use crate::repository::Timer;

use super::display_options::RedisDisplayOptions;
use super::segment::RedisSegment;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisTimer {
    V0(TimerV0),
}

impl Into<Timer> for RedisTimer {
    fn into(self) -> Timer {
        match self {
            RedisTimer::V0(t) => t.into(),
        }
    }
}

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
        }
    }
}
