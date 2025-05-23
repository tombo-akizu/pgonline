mod game;
mod consts;
mod vec2;

use std::collections::VecDeque;
use std::io::Error;
use std::sync::Arc;

use futures_util::stream::{SplitStream, SplitSink};
use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::{protocol::Message, Bytes};

use game::{InputMemory, OutputMemory};

type Ws = WebSocketStream<tokio::net::TcpStream>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = "0.0.0.0:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let waiting_clients = Arc::new(Mutex::new(VecDeque::<Ws>::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during the websocket handshake occurred");

        waiting_clients.lock().await.push_back(ws_stream);
        info!("{}", waiting_clients.lock().await.len());
        if waiting_clients.lock().await.len() >= 2 {
            let p1 = waiting_clients.lock().await.pop_front().unwrap();
            let p2 = waiting_clients.lock().await.pop_front().unwrap();
            let players = [p1, p2];
            tokio::spawn(game_2p(players));
        }
    }

    Ok(())
}

async fn game_2p(players: [Ws; 2]) {
    let [(write1, read1), (write2, read2)] = players.map(|ws| ws.split());

    let input_memory = Arc::new(Mutex::new(InputMemory::new()));
    let output_memory = Arc::new(Mutex::new(OutputMemory::new()));


    // start game thread
    tokio::spawn(game::game(input_memory.clone(), output_memory.clone()));

    // spawn writing to the thread
    tokio::spawn(update_input(input_memory.clone(), read1, 0));
    tokio::spawn(update_input(input_memory.clone(), read2, 1));

    // spawn reading from the thread
    tokio::spawn(send_output(output_memory.clone(), write1, 0));
    tokio::spawn(send_output(output_memory.clone(), write2, 1));
}


async fn update_input(
    input_memory: Arc<Mutex<InputMemory>>,
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    index: usize
) {
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(Message::Binary(data)) => {
                input_memory.lock().await.update(*data.get(0).unwrap(), index);
            }

            Ok(Message::Text(text)) => {
                info!("Received text: {}", text);
            }

            Ok(Message::Close(_)) => {
                info!("Connection closed");
                break;
            }

            _ => {}
        }
    }
}

async fn send_output(
    output_memory: Arc<Mutex<OutputMemory>>,
    mut write: SplitSink<WebSocketStream<TcpStream>, Message>,
    index: usize
) {
    loop {
        let send_result = write
            .send(Message::Binary(Bytes::from(output_memory.lock().await.encode(index))))
            .await;
        match send_result {
            Err(tokio_tungstenite::tungstenite::Error::Protocol(tokio_tungstenite::tungstenite::error::ProtocolError::SendAfterClosing)) => {
                break;
            }
            _ => {}
        }
    }
}
