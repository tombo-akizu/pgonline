use crate::util::Vec2;

#[derive(Clone)]
pub struct Bubble {
    pub position: Vec2,
    pub previous_velocity: Vec2
}

impl Bubble {
    pub fn new(point: Vec2) -> Self {
        Self { 
            position: point,
            previous_velocity: Vec2::new(0., 0.)
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