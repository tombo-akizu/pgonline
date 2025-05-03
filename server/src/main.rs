use std::{env, io::Error};

use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{protocol::Message, Bytes};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:3000".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    // read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
    //     .forward(write)
    //     .await
    //     .expect("Failed to forward messages")

    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(Message::Binary(data)) => {
                info!("Received binary: {:?}", data);

                let response = match data.get(0) {
                    Some(0x00) => vec![0x00],
                    Some(0x01) => vec![0x01],
                    Some(0x02) => vec![0x02],
                    Some(0x03) => vec![0x00],
                    _ => {
                        info!("unknown byte...");
                        vec![0x00]
                    }
                };

                write.send(Message::Binary(Bytes::from(response))).await.unwrap();
            }

            Ok(Message::Text(text)) => {
                info!("Received text: {}", text);
                write.send(Message::Text("Binary only!".into())).await.unwrap();
            }

            Ok(Message::Close(_)) => {
                info!("Connection closed");
                break;
            }

            _ => {}
        }
    }
}