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

use game::{InputMemory, GameStateMemory};

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
    let game_state_memory = Arc::new(Mutex::new(GameStateMemory::GameStart));


    // start game thread
    tokio::spawn(game::game(input_memory.clone(), game_state_memory.clone()));

    // spawn writing to the thread
    tokio::spawn(update_input(input_memory.clone(), read1, 0));
    tokio::spawn(update_input(input_memory.clone(), read2, 1));

    // spawn reading from the thread
    tokio::spawn(send_output(game_state_memory.clone(), write1, 0));
    tokio::spawn(send_output(game_state_memory.clone(), write2, 1));
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

// 送信機として働く非同期タスク。
async fn send_output(
    game_state_memory: Arc<Mutex<GameStateMemory>>,
    mut write: SplitSink<WebSocketStream<TcpStream>, Message>,
    index: usize
) {
    loop {
        // `send`を待つ間ロックを持ち続けないようにキャッシュする。
        // `is_game_end`を使用するのは`send`の後だが、
        // 処理順に依存しないよう`bytes`と同時にキャッシュする。
        let bytes: Option<Bytes>;
        let mut is_game_end = false;

        {
            let mut game_state_memory = game_state_memory.lock().await;
            bytes = Some(Bytes::from(game_state_memory.encode(index)));

            match *game_state_memory {
                GameStateMemory::GameStart => {
                    *game_state_memory = GameStateMemory::new_game_state();
                },
                GameStateMemory::GameEnd => {
                    is_game_end = true;
                },
                _ => {}
            }
        }

        let send_result = write
            .send(Message::Binary(bytes.unwrap()))
            .await;

        if is_game_end {
            break;
        }

        match send_result {
            Err(tokio_tungstenite::tungstenite::Error::Protocol(tokio_tungstenite::tungstenite::error::ProtocolError::SendAfterClosing)) => {
                break;
            }
            _ => {}
        }
    }
}
