use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::StreamExt;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt,
};

use serde::{Deserialize, Serialize};
use serde_json;

use redis::Commands;

use crate::SharedState;
use crate::Timer;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum WSMessage {
    Hello(String),
    GetTime,
    Timer(Timer),
    Timestamp(u128),
}

pub async fn ws_handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(State(state), socket))
}

struct WsConnection {
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
    redis: redis::Connection,
    timer_id: String,
    last_update_nr: i32,
}

impl WsConnection {
    fn new(state: SharedState, socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        let redis = redis::Client::open(state.redis_string.to_owned())
            .unwrap()
            .get_connection()
            .unwrap();

        WsConnection {
            sender,
            receiver,
            redis,
            timer_id: String::new(),
            last_update_nr: -1,
        }
    }

    async fn listen(&mut self) {
        loop {
            // check if message is available
            let msg = self.receiver.next().await;
            if let Some(Ok(Message::Text(msg))) = msg {
                println!("Received message: {:?}", msg);
                let message: WSMessage = serde_json::from_str(&msg).unwrap();
                self.handle_message(message).await;
            }

            // sleep for 10ms
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            // Check if timer has been updated
            if !self.timer_id.is_empty() {
                let update_nr: i32 = self
                    .redis
                    .get(format!("updated:{}", self.timer_id))
                    .unwrap();
                if update_nr > self.last_update_nr {
                    println!("Timer has been updated");
                    self.last_update_nr = update_nr;
                    let timer: Timer = serde_json::from_str(
                        &self
                            .redis
                            .get::<String, String>(self.timer_id.clone())
                            .unwrap(),
                    )
                    .unwrap();
                    let respone = WSMessage::Timer(timer.into());

                    self.sender
                        .send(serde_json::to_string(&respone).unwrap().into())
                        .await
                        .unwrap();
                }
            }
        }
    }

    async fn handle_message(&mut self, message: WSMessage) {
        match message {
            WSMessage::GetTime => {
                let start = SystemTime::now();
                let current_time = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Back to the Past lul");

                let respone = WSMessage::Timestamp(current_time.as_millis());

                self.sender
                    .send(serde_json::to_string(&respone).unwrap().into())
                    .await
                    .unwrap();
            }
            WSMessage::Hello(id) => {
                self.timer_id = id.clone();
                let timer: Timer =
                    serde_json::from_str(&self.redis.get::<String, String>(id.clone()).unwrap())
                        .unwrap();

                let respone = WSMessage::Timer(timer.into());

                self.sender
                    .send(serde_json::to_string(&respone).unwrap().into())
                    .await
                    .unwrap();
            }
            _ => {
                self.sender
                    .send("Invalid message type".into())
                    .await
                    .unwrap();
            }
        }
    }
}

async fn handle_socket(State(state): State<SharedState>, socket: WebSocket) {
    let mut connection = WsConnection::new(state, socket);

    connection.listen().await;
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(ws_handler))
}
