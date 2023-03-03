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

use redis::AsyncCommands;

use crate::SharedState;
use crate::Timer;

use std::time::{SystemTime, UNIX_EPOCH};

use super::timer::TimerResponse;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum WSMessage {
    Hello(String),
    GetTime,
    Timer(TimerResponse),
    Timestamp(u128),
    Error(String),
}

struct WsConnection {
    sender: SplitSink<WebSocket, Message>,
    receiver: SplitStream<WebSocket>,
    redis: redis::aio::ConnectionManager,
    pubsub: redis::aio::PubSub,
    timer_id: String,
}

impl WsConnection {
    async fn new(state: SharedState, socket: WebSocket) -> Self {
        let (sender, receiver) = socket.split();
        let redis = state.redis.clone();
        let pubsub = redis::Client::open(state.redis_string.to_owned())
            .unwrap()
            .get_async_connection()
            .await
            .unwrap()
            .into_pubsub();

        WsConnection {
            sender,
            receiver,
            redis,
            pubsub,
            timer_id: String::new(),
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
            let msg = self.pubsub.on_message().next().await;
            if let Some(msg) = msg {
                println!("Updated! {:?}", msg);
                let timer: Timer =
                    serde_json::from_str(&msg.get_payload::<String>().unwrap()).unwrap();
                let response = WSMessage::Timer(timer.into());
                self.send_message(&response).await;
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

                let response = WSMessage::Timestamp(current_time.as_millis());
                self.send_message(&response).await;
            }
            WSMessage::Hello(id) => {
                self.timer_id = id.clone();
                self.pubsub.subscribe(self.timer_id.clone()).await.unwrap();
                let timer: Timer = serde_json::from_str(
                    &self.redis.get::<String, String>(id.clone()).await.unwrap(),
                )
                .unwrap();

                let response = WSMessage::Timer(timer.into());
                self.send_message(&response).await;
            }
            _ => {
                let response = WSMessage::Error("Invalid message type".to_owned());
                self.send_message(&response).await;
            }
        }
    }

    async fn send_message(&mut self, message: &WSMessage) {
        self.sender
            .send(serde_json::to_string(message).unwrap().into())
            .await
            .unwrap();
    }
}

async fn handle_socket(State(state): State<SharedState>, socket: WebSocket) {
    let mut connection = WsConnection::new(state, socket).await;

    connection.listen().await;
}

pub async fn ws_handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(State(state), socket))
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(ws_handler))
}
