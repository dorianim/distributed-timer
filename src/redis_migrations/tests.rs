#[allow(unused_imports)]
use crate::repository::PreStartBehaviour;
#[allow(unused_imports)]
use crate::{redis_migrations::timer::RedisTimer, repository::Timer};

#[test]
fn test_v0() {
    let payload = r##"
        {
            "segments":[
               {
                  "label":"Boulder",
                  "time":230000,
                  "sound":true,
                  "color":"#26A269",
                  "count_to":11000
               }
            ],
            "id":"v0",
            "repeat":true,
            "display_options":{
               "clock":false,
               "pre_start_behaviour":"ShowZero"
            },
            "start_at":1688236579108,
            "stop_at":null,
            "password": "test"
         }
        "##;

    let timer: RedisTimer = serde_json::from_str(payload).unwrap();
    let timer: Timer = timer.into();
    assert_eq!(timer.segments.len(), 1);
    assert_eq!(timer.segments[0].label, "Boulder");
    assert_eq!(timer.segments[0].sounds.len(), 2);
    assert_eq!(timer.segments[0].sounds[0].filename, "beep.mp3");
    assert_eq!(timer.segments[0].sounds[0].trigger_time, 60);
    assert_eq!(timer.segments[0].sounds[1].filename, "countdown.mp3");
    assert_eq!(timer.segments[0].sounds[1].trigger_time, 5);
    assert_eq!(timer.metadata.delay_start_stop, 0);
    assert_eq!(
        timer.display_options.pre_start_behaviour,
        PreStartBehaviour::ShowFirstSegment
    );
}

#[test]
fn test_v1() {
    let payload = r##"
        {
            "segments":[
               {
                  "label":"Boulder",
                  "time":230000,
                  "color":"#26A269",
                  "count_to":11000,
                  "sounds": [
                     {
                        "filename": "beep.mp3",
                        "trigger_time": 60
                     }
                  ]
               }
            ],
            "id":"v0",
            "repeat":true,
            "display_options":{
               "clock":false,
               "pre_start_behaviour":"ShowLastSegment"
            },
            "start_at":1688236579108,
            "stop_at":null,
            "password": "test",
            "metadata": {
               "delay_start_stop": 5
            }
         }
        "##;

    let timer: RedisTimer = serde_json::from_str(payload).unwrap();
    let timer: Timer = timer.into();
    assert_eq!(timer.segments.len(), 1);
    assert_eq!(timer.segments[0].label, "Boulder");
    assert_eq!(timer.segments[0].sounds.len(), 1);
    assert_eq!(timer.segments[0].sounds[0].filename, "beep.mp3");
    assert_eq!(timer.segments[0].sounds[0].trigger_time, 60);
    assert_eq!(timer.metadata.delay_start_stop, 5);
    assert_eq!(
        timer.display_options.pre_start_behaviour,
        PreStartBehaviour::ShowLastSegment
    );
}
