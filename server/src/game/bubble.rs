use crate::util::Vec2;

#[derive(Clone)]
pub struct Bubble {
    pub position: Vec2
}

impl Bubble {
    pub fn new(point: Vec2) -> Self {
        Self { position: point }
    }
}