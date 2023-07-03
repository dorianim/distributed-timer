use serde::Deserialize;

use crate::repository::TimerMetadata;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisTimerMetadata {
    V0(TimerMetadataV0),
}

impl Into<TimerMetadata> for RedisTimerMetadata {
    fn into(self) -> TimerMetadata {
        match self {
            RedisTimerMetadata::V0(v0) => v0.into(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct TimerMetadataV0 {
    pub delay_start_stop: u32,
}

impl Into<TimerMetadata> for TimerMetadataV0 {
    fn into(self) -> TimerMetadata {
        TimerMetadata {
            delay_start_stop: self.delay_start_stop,
        }
    }
}
