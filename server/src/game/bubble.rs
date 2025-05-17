use crate::vec2::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum BubbleColor {
    Red,
    White
}

impl BubbleColor {
    pub fn to_le_bytes_vec(&self) -> Vec<u8> {
        let mut output = vec![];
        match self {
            BubbleColor::Red => {
                output.extend((0 as u8).to_le_bytes().to_vec());
            },
            BubbleColor::White => {
                output.extend((1 as u8).to_le_bytes().to_vec());
            }
        }
        output
    }
}

#[derive(Clone)]
pub struct Bubble {
    pub position: Vec2,
    pub previous_velocity: Vec2,
    pub color: BubbleColor,
}

impl Bubble {
    pub fn new(point: Vec2, color: BubbleColor) -> Self {
        Self { 
            position: point,
            previous_velocity: Vec2::new(0., 0.),
            color
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn move_physically(&mut self, position: Vec2) {
        self.previous_velocity = position - self.position;
        self.position = position;
    }
}