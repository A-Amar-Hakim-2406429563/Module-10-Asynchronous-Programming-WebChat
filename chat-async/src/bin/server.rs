use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast::{channel, Sender}, Mutex};

use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

fn build_users_message(users: Vec<String>) -> String {
    let message = WebSocketMessage {
        message_type: MsgTypes::Users,
        data_array: Some(users),
        data: None,
    };
    serde_json::to_string(&message).unwrap()
}

fn build_chat_message(from: String, message: String) -> String {
    let payload = MessageData { from, message };
    let message = WebSocketMessage {
        message_type: MsgTypes::Message,
        data_array: None,
        data: Some(serde_json::to_string(&payload).unwrap()),
    };
    serde_json::to_string(&message).unwrap()
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Arc<Mutex<HashSet<String>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    let mut username: Option<String> = None;

    loop {
        tokio::select! {

            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            if let Ok(packet) = serde_json::from_str::<WebSocketMessage>(text) {
                                match packet.message_type {
                                    MsgTypes::Register => {
                                        if let Some(name) = packet.data {
                                            println!("Register from {addr}: {name}");
                                            username = Some(name.clone());
                                            let users_list = {
                                                let mut users = users.lock().await;
                                                users.insert(name);
                                                let mut list: Vec<String> = users.iter().cloned().collect();
                                                list.sort();
                                                list
                                            };
                                            let payload = build_users_message(users_list);
                                            let _ = bcast_tx.send(payload);
                                        }
                                    }
                                    MsgTypes::Message => {
                                        if let (Some(message_text), Some(name)) = (packet.data, username.clone()) {
                                            println!("From client {addr}: {message_text}");
                                            let payload = build_chat_message(name, message_text);
                                            let _ = bcast_tx.send(payload);
                                        }
                                    }
                                    MsgTypes::Users => {}
                                }
                            }
                        }
                    }

                    _ => break,
                }
            }

            Ok(msg) = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg)).await?;
            }
        }
    }

    if let Some(name) = username {
        let users_list = {
            let mut users = users.lock().await;
            users.remove(&name);
            let mut list: Vec<String> = users.iter().cloned().collect();
            list.sort();
            list
        };
        let payload = build_users_message(users_list);
        let _ = bcast_tx.send(payload);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let (bcast_tx, _) = channel(16);
    let users = Arc::new(Mutex::new(HashSet::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("listening on port 8080");

    loop {

        let (socket, addr) = listener.accept().await?;

        println!("New connection from Amar's Computer {addr:?}");

        let bcast_tx = bcast_tx.clone();
        let users = users.clone();

        tokio::spawn(async move {

            let (_req, ws_stream) =
                ServerBuilder::new()
                    .accept(socket)
                    .await
                    .unwrap();

            handle_connection(addr, ws_stream, bcast_tx, users)
                .await
                .unwrap();
        });
    }
}