use crate::vec2::Vec2;
use super::bubble::BubbleColor;

// クライアントの操作を書き込み、ゲームから読み取るための共有メモリ。
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

// ゲームの状態を書き込み、クライアントに送信するための共有メモリ。
pub enum GameStateMemory {
    OnGoing {
        angles: [f32; 2],
        scores: [i8; 2],
        bubble_positions: Vec<Vec<Vec2>>,
        bubble_colors: Vec<Vec<BubbleColor>>,
    },
    GameStart,
    GameEnd
}

impl GameStateMemory {
    pub fn new_game_state() -> Self {
        Self::OnGoing {
            angles: [0., 0.],
            scores: [0, 0],
            bubble_positions: vec![vec![], vec![]],
            bubble_colors: vec![vec![], vec![]]
        }
    }

    pub fn encode(&self, index: usize) -> Vec<u8> {
        match self {
            Self::GameStart => vec![0x01],
            Self::GameEnd => vec![0x02],
            Self::OnGoing { 
                angles,
                scores,
                bubble_positions,
                bubble_colors
            } => {
                Self::encode_game_state(
                    index, 
                    angles, 
                    scores, 
                    bubble_positions, 
                    bubble_colors
                )
            }
        }
    }

    fn encode_game_state(
        index: usize,
        angles: &[f32; 2],
        scores: &[i8; 2],
        bubble_positions: &Vec<Vec<Vec2>>,
        bubble_colors: &Vec<Vec<BubbleColor>>
    ) -> Vec<u8> {
        let mut outputs = [vec![], vec![]];

        for i in 0..2 {
            outputs[i].extend(angles[i].to_le_bytes().to_vec());
            outputs[i].extend(scores[i].to_le_bytes().to_vec());
            let len: u8 = bubble_positions[i].len().try_into().unwrap();
            outputs[i].extend(len.to_le_bytes().to_vec());

            for (position, color) in bubble_positions[i].iter().zip(bubble_colors[i].iter()) {
                outputs[i].extend(position.to_le_bytes_vec());
                outputs[i].extend(color.to_le_bytes_vec());
            }
        }

        let [output1, output2] = outputs;
        let mut output = vec![0x00];
        match index {
            0 => {
                output.extend(output1);
                output.extend(output2);
                output
            },
            1 => {
                output.extend(output2);
                output.extend(output1);
                output
            },
            _ => {
                panic!();
            }
        }
    }
}