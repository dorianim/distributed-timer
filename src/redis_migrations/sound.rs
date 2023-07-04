use serde::Deserialize;

use crate::repository::Sound;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisSound {
    V0(SoundV0),
}

impl Into<Sound> for RedisSound {
    fn into(self) -> Sound {
        match self {
            RedisSound::V0(v0) => v0.into(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct SoundV0 {
    pub filename: String,
    pub trigger_time: u32,
}

impl Into<Sound> for SoundV0 {
    fn into(self) -> Sound {
        Sound {
            filename: self.filename,
            trigger_time: self.trigger_time,
        }
    }
}
