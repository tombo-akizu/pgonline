use std::ops;

use crate::consts::DEG2RAD;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f32) -> Self {
        Self {
            x: f32::cos(angle * DEG2RAD),
            y: f32::sin(angle * DEG2RAD)
        }
    }

    pub fn slope_down(angle: f32) -> Self {
        if angle == 0. {
            Self::new(0., 0.)
        } else if angle > 0. {
            Self::new(-f32::cos(angle * DEG2RAD), -f32::sin(angle * DEG2RAD))
        } else {
            Self::new(f32::cos(angle * DEG2RAD), f32::sin(angle * DEG2RAD))
        }
    }

    pub fn inner_product(v0: Self, v1: Self) -> f32 {
        v0.x * v1.x + v0.y * v1.y
    }

    pub fn sqr_distance(&self, other: Vec2) -> f32 {
        (other.x - self.x).powf(2.) + (other.y - self.y).powf(2.)
    }

    pub fn distance(&self, other:Vec2) -> f32 {
        self.sqr_distance(other).powf(0.5)
    }

    pub fn slope(&self) -> f32 {
        self.y / self.x
    }

    pub fn to_le_bytes_vec(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend(self.x.to_le_bytes().to_vec());
        output.extend(self.y.to_le_bytes().to_vec());
        output
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}