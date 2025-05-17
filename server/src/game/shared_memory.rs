use crate::vec2::Vec2;
use super::bubble::BubbleColor;

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
    pub angles: [f32; 2],
    pub scores: [i8; 2],
    pub bubble_positions: Vec<Vec<Vec2>>,
    pub bubble_colors: Vec<Vec<BubbleColor>>,
}

impl OutputMemory {
    pub fn new() -> Self {
        Self {
            angles: [0., 0.],
            scores: [0, 0],
            bubble_positions: vec![vec![], vec![]],
            bubble_colors: vec![vec![], vec![]]
        }
    }

    pub fn encode(&self, index: usize) -> Vec<u8> {
        let mut outputs = [vec![], vec![]];

        for i in 0..2 {
            outputs[i].extend(self.angles[i].to_le_bytes().to_vec());
            outputs[i].extend(self.scores[i].to_le_bytes().to_vec());
            let len: u8 = self.bubble_positions[i].len().try_into().unwrap();
            outputs[i].extend(len.to_le_bytes().to_vec());

            for (position, color) in self.bubble_positions[i].iter().zip(self.bubble_colors[i].iter()) {
                outputs[i].extend(position.to_le_bytes_vec());
                outputs[i].extend(color.to_le_bytes_vec());
            }
        }

        let [mut output1, mut output2] = outputs;
        match index {
            0 => {
                output1.extend(output2);
                output1
            },
            1 => {
                output2.extend(output1);
                output2
            },
            _ => {
                panic!();
            }
        }
    }
}