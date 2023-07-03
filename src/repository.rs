use std::sync::Arc;

use crate::{color::Color, redis_migrations::RedisTimer};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use redis::AsyncCommands;
use tokio::{
    sync::broadcast::{self, Receiver},
    task::JoinHandle,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    pub label: String,
    pub time: u32,
    pub color: Option<Color>,
    pub count_to: u32,
    pub sounds: Vec<Sound>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sound {
    pub filename: String,
    pub trigger_time: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum PreStartBehaviour {
    ShowFirstSegment,
    ShowLastSegment,
    RunNormally,
}

impl Default for PreStartBehaviour {
    fn default() -> Self {
        PreStartBehaviour::ShowFirstSegment
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DisplayOptions {
    pub clock: bool,
    pub pre_start_behaviour: PreStartBehaviour,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct TimerMetadata {
    pub delay_start_stop: u32,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Timer {
    pub segments: Vec<Segment>,
    pub repeat: bool,
    pub display_options: DisplayOptions,
    pub start_at: u64,
    pub stop_at: Option<u64>,
    pub password: String,
    pub id: String,
    pub metadata: TimerMetadata,
}

#[derive(Clone)]
pub struct Repository {
    redis: redis::aio::ConnectionManager,
    pub updates_rx: Arc<Receiver<Timer>>,
}

impl Repository {
    pub async fn new(redis_string: String) -> Self {
        let client =
            redis::Client::open(redis_string.to_owned()).expect("Could not connect to redis");
        let manager = redis::aio::ConnectionManager::new(client.clone())
            .await
            .unwrap();

        let (redis_task_tx, redis_task_rx) = broadcast::channel::<Timer>(10);
        spawn_global_redis_listener_task(manager.clone(), client, redis_task_tx);

        Repository {
            redis: manager,
            updates_rx: Arc::new(redis_task_rx),
        }
    }

    pub async fn get_timer(&self, id: String) -> Option<Timer> {
        let mut redis = self.redis.clone();
        let timer = &redis.get::<String, String>(id).await;

        if timer.is_err() {
            return None;
        }

        let timer: RedisTimer = serde_json::from_str(timer.as_ref().unwrap()).unwrap();
        Some(timer.into())
    }

    pub async fn create_timer(&self, timer: &Timer) -> Result<(), ()> {
        let mut redis = self.redis.clone();
        if redis
            .exists::<String, bool>(timer.id.clone())
            .await
            .unwrap()
        {
            return Err(());
        }

        redis
            .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(timer).unwrap())
            .await
            .unwrap();

        return Ok(());
    }

    pub async fn update_timer(&self, timer: &Timer) {
        let mut redis = self.redis.clone();
        redis
            .set::<String, String, ()>(timer.id.clone(), serde_json::to_string(timer).unwrap())
            .await
            .unwrap();
    }

    pub async fn delete_timer(&self, id: String) -> Result<(), ()> {
        self.redis
            .clone()
            .del::<String, ()>(id)
            .await
            .map_err(|_| ())
    }
}

pub fn spawn_global_redis_listener_task(
    mut redis: redis::aio::ConnectionManager,
    redis_client: redis::Client,
    redis_task_tx: broadcast::Sender<Timer>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut connection = redis_client.get_async_connection().await.unwrap();
        let _: () = redis::cmd("CONFIG")
            .arg("SET")
            .arg("notify-keyspace-events")
            .arg("KEA")
            .query_async(&mut connection)
            .await
            .unwrap();

        let mut pubsub = connection.into_pubsub();

        pubsub
            .psubscribe("__keyspace@*__:*")
            .await
            .expect("Failed to subscribe to redis channel");

        let mut pubsub = pubsub.into_on_message();

        while let Some(msg) = pubsub.next().await {
            println!("Updated! {:?}", msg);
            let timer_id = msg.get_channel_name().split(":").last().unwrap();

            let timer_str = &redis
                .get::<String, String>(String::from(timer_id))
                .await
                .expect("Did not find timer in redis");
            let timer: Timer = serde_json::from_str(timer_str).unwrap();

            // Broadcast to all listeners
            redis_task_tx.send(timer).unwrap();
        }
    })
}
