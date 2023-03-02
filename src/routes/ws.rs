use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::SinkExt;
use futures::StreamExt;

use serde::{Deserialize, Serialize};
use serde_json;

use redis::Commands;

use crate::SharedState;
use crate::{Timer};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum MessageData {
    Timestamp(u128),
    Timer(Timer),
    TimerId(String),
}

#[derive(Serialize, Deserialize)]
struct WSMessage {
    message_type: String, // command, reply, push
    command: String,      // get_time, hello
    data: MessageData,    // Timestamp, Timer
}

pub async fn ws_handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(State(state), socket))
}

async fn handle_socket(State(state): State<SharedState>, socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();

    let mut last_update_nr: i32 = -1;
    let mut timer_id = String::new();
    loop {
        // check if message is available
        let msg = receiver.next().await;
        if !msg.is_none() {
            if let Some(Ok(Message::Text(msg))) = msg {
                println!("Received message: {:?}", msg);
                let message: WSMessage = serde_json::from_str(&msg).unwrap();

                match message.message_type.as_str() {
                    "command" => {
                        match message.command.as_str() {
                            "get_time" => {
                                let start = SystemTime::now();
                                let current_time = start
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Back to the Past lul");

                                let respone = WSMessage {
                                    message_type: "reply".into(),
                                    command: "get_time".into(),
                                    data: MessageData::Timestamp(current_time.as_millis()),
                                };

                                sender
                                    .send(serde_json::to_string(&respone).unwrap().into())
                                    .await
                                    .unwrap();
                            }
                            "hello" => {
                                // Reply with Timer from timer_id
                                {
                                    let mut redis = state.as_ref().redis.write().await;
                                    let id: String = match message.data {
                                        MessageData::TimerId(id) => id,
                                        _ => "".into(),
                                    };
                                    timer_id = id.clone();
                                    let timer: Timer = serde_json::from_str(&redis.get::<String, String>(id.clone()).unwrap()).unwrap();
        
                                    last_update_nr = redis.get(format!("updated:{}", id)).unwrap();

                                    let respone = WSMessage {
                                        message_type: "reply".into(),
                                        command: "hello".into(),
                                        data: MessageData::Timer(
                                            timer.into()
                                        ),
                                    };

                                    sender
                                        .send(serde_json::to_string(&respone).unwrap().into())
                                        .await
                                        .unwrap();
                                }
                            }
                            _ => {
                                println!("Invalid command");
                            }
                        }
                    }
                    _ => {
                        sender.send("Invalid message type".into()).await.unwrap();
                    }
                }
            }
        }

        // sleep for 10ms
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Check if timer has been updated
        if !timer_id.is_empty() {
            let mut redis = state.as_ref().redis.write().await;
            let update_nr: i32 = redis.get(format!("updated:{}", timer_id)).unwrap();
            if update_nr > last_update_nr {
                println!("Timer has been updated");
                last_update_nr = update_nr;
                let timer: Timer = serde_json::from_str(&redis.get::<String, String>(timer_id.clone()).unwrap()).unwrap();
                let respone = WSMessage {
                    message_type: "push".into(),
                    command: "update".into(),
                    data: MessageData::Timer(timer.into()),
                };

                sender
                    .send(serde_json::to_string(&respone).unwrap().into())
                    .await
                    .unwrap();
            }
        }
    }
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(ws_handler))
}
