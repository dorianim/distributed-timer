use serde::Deserialize;

use crate::repository::DisplayOptions;

use super::pre_start_behaviour::RedisPreStartBehaviour;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisDisplayOptions {
    V0(DisplayOptionsV0),
}

impl Into<DisplayOptions> for RedisDisplayOptions {
    fn into(self) -> DisplayOptions {
        match self {
            RedisDisplayOptions::V0(v0) => v0.into(),
        }
    }
}

impl Into<DisplayOptions> for Option<RedisDisplayOptions> {
    fn into(self) -> DisplayOptions {
        self.map(|o| o.into()).unwrap_or(DisplayOptions::default())
    }
}

#[derive(Deserialize, Clone)]
pub struct DisplayOptionsV0 {
    #[serde(default)]
    clock: bool,
    #[serde(default)]
    pre_start_behaviour: RedisPreStartBehaviour,
}

impl Into<DisplayOptions> for DisplayOptionsV0 {
    fn into(self) -> DisplayOptions {
        DisplayOptions {
            clock: self.clock,
            pre_start_behaviour: self.pre_start_behaviour.into(),
        }
    }
}

impl Into<DisplayOptions> for Option<DisplayOptionsV0> {
    fn into(self) -> DisplayOptions {
        self.map(|o| o.into()).unwrap_or(DisplayOptions::default())
    }
}
