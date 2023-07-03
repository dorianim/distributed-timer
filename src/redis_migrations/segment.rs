use serde::Deserialize;

use crate::{color::Color, repository::Segment};

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisSegment {
    V0(SegmentV0),
}

impl Into<Segment> for RedisSegment {
    fn into(self) -> Segment {
        match self {
            RedisSegment::V0(v0) => v0.into(),
        }
    }
}

fn default_zero() -> u32 {
    0
}

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
        Segment {
            label: self.label,
            time: self.time,
            sound: self.sound,
            color: self.color,
            count_to: self.count_to,
        }
    }
}
