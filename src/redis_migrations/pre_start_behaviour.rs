use serde::Deserialize;

use crate::repository::PreStartBehaviour;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum RedisPreStartBehaviour {
    V0(PreStartBehaviourV0),
}

impl Into<PreStartBehaviour> for RedisPreStartBehaviour {
    fn into(self) -> PreStartBehaviour {
        match self {
            RedisPreStartBehaviour::V0(v0) => v0.into(),
        }
    }
}

impl Default for RedisPreStartBehaviour {
    fn default() -> Self {
        Self::V0(PreStartBehaviourV0::default())
    }
}

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
            PreStartBehaviourV0::ShowZero => PreStartBehaviour::ShowZero,
        }
    }
}
