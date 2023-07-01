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
               },
               {
                  "label":"Boulder",
                  "time":11000,
                  "sound":true,
                  "color":"#A51D2D",
                  "count_to":0
               },
               {
                  "label":"Change",
                  "time":14000,
                  "sound":true,
                  "color":"#E66100",
                  "count_to":1000
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
    assert_eq!(timer.segments.len(), 3);
    assert_eq!(timer.segments[0].label, "Boulder");
}
