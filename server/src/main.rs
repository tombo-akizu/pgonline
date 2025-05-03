mod game;

use std::collections::VecDeque;
use std::{env, io::Error};
use std::sync::Arc;

use futures_util::stream::{SplitStream, SplitSink};
use futures_util::{SinkExt, StreamExt};
use log::info;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::{protocol::Message, Bytes};

type Ws = WebSocketStream<tokio::net::TcpStream>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:3000".to_string());

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

        // tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

pub struct InputMemory {
    pub right_inputs: [bool; 2],
    pub left_inputs: [bool; 2],
}

impl InputMemory {
    pub fn new() -> Self {
        Self {
            right_inputs: [false, false],
            left_inputs: [false, false],
        }
    }

    pub fn update(&mut self, byte: u8, index: usize) {
        self.right_inputs[index] = byte == 0x01;
        self.left_inputs[index] = byte == 0x02;
    }
}

pub struct OutputMemory {
    pub xs: [f32; 2],
    pub zs: [f32; 2],
}

impl OutputMemory {
    pub fn new() -> Self {
        Self {
            xs: [0., 0.],
            zs: [0., 0.],
        }
    }

    pub fn encode(&self, index: usize) -> Vec<u8> {
        let mut output = vec![];
        let myx = self.xs[index].to_le_bytes().to_vec();
        let myz = self.zs[index].to_le_bytes().to_vec();
        let (other_x, other_z) = match index {
            0 => {
                (
                    self.xs[1].to_le_bytes().to_vec(),
                    self.zs[1].to_le_bytes().to_vec()
                )
            },
            1 => {
                (
                    self.xs[0].to_le_bytes().to_vec(),
                    self.zs[0].to_le_bytes().to_vec()
                )
            },
            _ => panic!()
        };
        output.extend(myx);
        output.extend(myz);
        output.extend(other_x);
        output.extend(other_z);
        output
    }
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
        write.send(Message::Binary(Bytes::from(output_memory.lock().await.encode(index)))).await.unwrap();
    }
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