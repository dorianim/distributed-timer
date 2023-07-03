use serde::Deserialize;

use crate::repository::PreStartBehaviour;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisPreStartBehaviour {
    V1(PreStartBehaviourV1),
    V0(PreStartBehaviourV0),
}

impl Into<PreStartBehaviour> for RedisPreStartBehaviour {
    fn into(self) -> PreStartBehaviour {
        match self {
            RedisPreStartBehaviour::V0(v0) => v0.into(),
            RedisPreStartBehaviour::V1(v1) => v1.into(),
        }
    }
}

impl Default for RedisPreStartBehaviour {
    fn default() -> Self {
        Self::V1(PreStartBehaviourV1::default())
    }
}

/// === V1 ===
#[derive(Deserialize, Clone)]
pub enum PreStartBehaviourV1 {
    ShowFirstSegment,
    ShowLastSegment,
    RunNormally,
}

impl Default for PreStartBehaviourV1 {
    fn default() -> Self {
        PreStartBehaviourV1::ShowFirstSegment
    }
}

impl Into<PreStartBehaviour> for PreStartBehaviourV1 {
    fn into(self) -> PreStartBehaviour {
        match self {
            PreStartBehaviourV1::RunNormally => PreStartBehaviour::RunNormally,
            PreStartBehaviourV1::ShowFirstSegment => PreStartBehaviour::ShowFirstSegment,
            PreStartBehaviourV1::ShowLastSegment => PreStartBehaviour::ShowLastSegment,
        }
    }
}

/// === V0 ===
#[derive(Deserialize, Clone)]
pub enum PreStartBehaviourV0 {
    ShowZero,
    RunNormally,
}

impl Default for PreStartBehaviourV0 {
    fn default() -> Self {
        PreStartBehaviourV0::ShowZero
    }
}

impl Into<PreStartBehaviour> for PreStartBehaviourV0 {
    fn into(self) -> PreStartBehaviour {
        match self {
            PreStartBehaviourV0::RunNormally => PreStartBehaviour::RunNormally,
            PreStartBehaviourV0::ShowZero => PreStartBehaviour::ShowFirstSegment,
        }
    }
}
