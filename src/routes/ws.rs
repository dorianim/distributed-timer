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

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::{
    repository::{Repository, Timer},
    SharedState,
};

use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
enum WSMessage {
    Hello(String),
    GetTime,
    Timer(WsTimerResponse),
    Timestamp(u128),
    Error((u128, String)),
}

struct WsConnection {}

impl WsConnection {
    async fn new(state: SharedState, socket: WebSocket) {
        let (ws_sender, ws_receiver) = socket.split();
        let (ws_message_tx, ws_message_rx) = tokio::sync::mpsc::channel::<WSMessage>(32);
        let (redis_listen_id_tx, redis_listen_id_rx) = tokio::sync::mpsc::channel::<String>(32);

        let ws_sender_task = WsConnection::spawn_ws_sender_task(ws_sender, ws_message_rx);
        let ws_receiver_task = WsConnection::spawn_ws_receiver_task(
            state.repository.clone(),
            ws_message_tx.clone(),
            redis_listen_id_tx,
            ws_receiver,
        );
        let redis_listener_task = WsConnection::spawn_redis_listener_task(
            ws_message_tx,
            redis_listen_id_rx,
            state.repository.updates_rx.resubscribe(),
        );

        ws_receiver_task.await.unwrap();

        ws_sender_task.abort();
        redis_listener_task.abort();
    }

    fn spawn_redis_listener_task(
        ws_message_tx: Sender<WSMessage>,
        mut redis_listen_id_rx: Receiver<String>,
        mut redis_task_rx: tokio::sync::broadcast::Receiver<Timer>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut msg = None;
            while msg.is_none() {
                msg = redis_listen_id_rx.recv().await;
            }

            let timer_id = msg.unwrap();

            while let Ok(timer) = redis_task_rx.recv().await {
                if timer.id == timer_id {
                    println!("Updated! {:?}", timer);

                    let response = WSMessage::Timer(timer.into());
                    ws_message_tx.send(response).await.unwrap();
                }
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
        repository: Repository,
        ws_message_tx: Sender<WSMessage>,
        redis_listen_id_tx: Sender<String>,
        ws_receiver: SplitStream<WebSocket>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut message_handler =
                WsMessageHandler::new(repository, ws_message_tx, redis_listen_id_tx, ws_receiver);
            message_handler.listen().await;
        })
    }
}

struct WsMessageHandler {
    repository: Repository,
    ws_message_tx: Sender<WSMessage>,
    redis_listen_id_tx: Sender<String>,
    ws_receiver: SplitStream<WebSocket>,
}

impl WsMessageHandler {
    fn new(
        repository: Repository,
        ws_message_tx: Sender<WSMessage>,
        redis_listen_id_tx: Sender<String>,
        ws_receiver: SplitStream<WebSocket>,
    ) -> Self {
        WsMessageHandler {
            repository,
            ws_message_tx,
            redis_listen_id_tx,
            ws_receiver,
        }
    }

    async fn listen(&mut self) {
        while let Some(msg) = self.ws_receiver.next().await {
            if let Ok(Message::Text(msg)) = msg {
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
            _ => WSMessage::Error((400, "Invalid message type".to_owned())),
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
            return WSMessage::Error((400, "Already said hello!".to_owned()));
        }

        self.redis_listen_id_tx.send(id.clone()).await.unwrap();

        self.repository.get_timer(id).await.map_or_else(
            || WSMessage::Error((404, "Timer not found!".to_owned())),
            |t| WSMessage::Timer(t.into()),
        )
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
