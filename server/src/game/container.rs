use super::bubble::BubbleColor;
use super::consts::DELETE_BORDER;
use crate::vec2::Vec2;

pub struct Container {
    min_end: f32,
    max_end: f32,
    color: BubbleColor,
}

impl Container {
    pub fn new(min_end: f32, max_end: f32, color: BubbleColor) -> Self {
        Self {
            min_end,
            max_end,
            color,
        }
    }

    pub fn in_container(&self, position: Vec2) -> bool {
        position.y < DELETE_BORDER && position.x > self.min_end && position.x < self.max_end
    }

    pub fn is_correct_container(&self, color: BubbleColor) -> bool {
        self.color == color
    }
}
