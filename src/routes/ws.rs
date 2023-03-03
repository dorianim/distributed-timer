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
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::SharedState;
use crate::Timer;

use std::time::{SystemTime, UNIX_EPOCH};

use super::timer::TimerResponse;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum WSMessage {
    Hello(String),
    GetTime,
    Timer(TimerResponse),
    Timestamp(u128),
    Error(String),
}

struct WsConnection {}

impl WsConnection {
    async fn new(state: SharedState, socket: WebSocket) {
        let mut connection = state.redis_client.get_async_connection().await.unwrap();
        let _: () = redis::cmd("CONFIG")
            .arg("SET")
            .arg("notify-keyspace-events")
            .arg("KEA")
            .query_async(&mut connection)
            .await
            .unwrap();
        let pubsub = connection.into_pubsub();

        let (ws_sender, ws_receiver) = socket.split();
        let (ws_message_tx, ws_message_rx) = tokio::sync::mpsc::channel::<WSMessage>(32);
        let (redis_listen_id_tx, redis_listen_id_rx) = tokio::sync::mpsc::channel::<String>(32);

        let ws_sender_task = WsConnection::spawn_ws_sender_task(ws_sender, ws_message_rx);
        let ws_receiver_task = WsConnection::spawn_ws_receiver_task(
            state.redis.clone(),
            ws_message_tx.clone(),
            redis_listen_id_tx,
            ws_receiver,
        );
        let redis_listener_task = WsConnection::spawn_redis_listener_task(
            state.redis.clone(),
            ws_message_tx,
            redis_listen_id_rx,
            pubsub,
        );

        ws_sender_task.await.unwrap();
        ws_receiver_task.await.unwrap();
        redis_listener_task.await.unwrap();
    }

    fn spawn_redis_listener_task(
        mut redis: redis::aio::ConnectionManager,
        ws_message_tx: Sender<WSMessage>,
        mut redis_listen_id_rx: Receiver<String>,
        mut pubsub: redis::aio::PubSub,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut msg = None;
            while msg.is_none() {
                msg = redis_listen_id_rx.recv().await;
            }

            let timer_id = msg.unwrap();
            pubsub
                .psubscribe(format!("__keyspace@*__:{}", &timer_id))
                .await
                .unwrap();
            println!("Redis listening!");
            redis_listen_id_rx.close();

            let mut pubsub = pubsub.into_on_message();

            while let Some(msg) = pubsub.next().await {
                println!("Updated! {:?}", msg);

                let timer: Timer = serde_json::from_str(
                    &redis.get::<String, String>(timer_id.clone()).await.unwrap(),
                )
                .unwrap();

                let response = WSMessage::Timer(timer.into());
                ws_message_tx.send(response).await.unwrap();
            }
        })
    }

    fn spawn_ws_sender_task(
        mut sender: SplitSink<WebSocket, Message>,
        mut rx: Receiver<WSMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                sender
                    .send(serde_json::to_string(&msg).unwrap().into())
                    .await
                    .unwrap();
            }
        })
    }

    fn spawn_ws_receiver_task(
        redis: redis::aio::ConnectionManager,
        ws_message_tx: Sender<WSMessage>,
        redis_listen_id_tx: Sender<String>,
        ws_receiver: SplitStream<WebSocket>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut message_handler =
                WsMessageHandler::new(redis, ws_message_tx, redis_listen_id_tx, ws_receiver);
            message_handler.listen().await;
        })
    }
}

struct WsMessageHandler {
    redis: redis::aio::ConnectionManager,
    ws_message_tx: Sender<WSMessage>,
    redis_listen_id_tx: Sender<String>,
    ws_receiver: SplitStream<WebSocket>,
}

impl WsMessageHandler {
    fn new(
        redis: redis::aio::ConnectionManager,
        ws_message_tx: Sender<WSMessage>,
        redis_listen_id_tx: Sender<String>,
        ws_receiver: SplitStream<WebSocket>,
    ) -> Self {
        WsMessageHandler {
            redis,
            ws_message_tx,
            redis_listen_id_tx,
            ws_receiver,
        }
    }

    async fn listen(&mut self) {
        loop {
            let msg = self.ws_receiver.next().await;
            if let Some(Ok(Message::Text(msg))) = msg {
                println!("Received message: {:?}", msg);
                let message: WSMessage = serde_json::from_str(&msg).unwrap();
                let response = self.handle_message(message).await;
                self.ws_message_tx.send(response).await.unwrap();
            }
        }
    }

    async fn handle_message(&mut self, message: WSMessage) -> WSMessage {
        match message {
            WSMessage::GetTime => self.handle_message_gettime().await,
            WSMessage::Hello(id) => self.handle_message_hello(id).await,
            _ => WSMessage::Error("Invalid message type".to_owned()),
        }
    }

    async fn handle_message_gettime(&self) -> WSMessage {
        let start = SystemTime::now();
        let current_time = start
            .duration_since(UNIX_EPOCH)
            .expect("Back to the Past lul");

        WSMessage::Timestamp(current_time.as_millis())
    }

    async fn handle_message_hello(&mut self, id: String) -> WSMessage {
        if self.redis_listen_id_tx.is_closed() {
            return WSMessage::Error("Already said hello!".to_owned());
        }

        self.redis_listen_id_tx.send(id.clone()).await.unwrap();

        let timer: Timer =
            serde_json::from_str(&self.redis.get::<String, String>(id).await.unwrap()).unwrap();

        WSMessage::Timer(timer.into())
    }
}

pub async fn ws_handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| WsConnection::new(state, socket))
}

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(ws_handler))
}
