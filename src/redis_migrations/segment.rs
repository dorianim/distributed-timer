use serde::Deserialize;

use crate::{
    color::Color,
    repository::{Segment, Sound},
};

use super::sound::RedisSound;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisSegment {
    V1(SegmentV1),
    V0(SegmentV0),
}

impl Into<Segment> for RedisSegment {
    fn into(self) -> Segment {
        match self {
            RedisSegment::V0(v0) => v0.into(),
            RedisSegment::V1(v1) => v1.into(),
        }
    }
}

fn default_zero() -> u32 {
    0
}

/// === V1 ===

#[derive(Deserialize, Clone)]
pub struct SegmentV1 {
    label: String,
    time: u32,
    color: Option<Color>,
    #[serde(default = "default_zero")]
    count_to: u32,
    sounds: Vec<RedisSound>,
}

impl Into<Segment> for SegmentV1 {
    fn into(self) -> Segment {
        Segment {
            label: self.label,
            time: self.time,
            color: self.color,
            count_to: self.count_to,
            sounds: self.sounds.into_iter().map(|s| s.into()).collect(),
        }
    }
}

/// === V0 ===

#[derive(Deserialize, Clone)]
pub struct SegmentV0 {
    label: String,
    time: u32,
    sound: bool,
    color: Option<Color>,
    #[serde(default = "default_zero")]
    count_to: u32,
}

impl Into<Segment> for SegmentV0 {
    fn into(self) -> Segment {
        let mut sounds: Vec<Sound> = Vec::new();

        if self.sound {
            sounds.push(Sound {
                filename: "beep.mp3".to_string(),
                trigger_time: 60,
            });
            sounds.push(Sound {
                filename: "countdown.mp3".to_string(),
                trigger_time: 5,
            });
        }

        Segment {
            label: self.label,
            time: self.time,
            color: self.color,
            count_to: self.count_to,
            sounds: sounds,
        }
    }
}
