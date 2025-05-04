use core::f32;

use crate::util::{Vec2, DEG2RAD, RAD2DEG};

const MAX_ANGLE: f32 = 30.;
const ROTATE_SPEED: f32 = 1.;

pub struct Bar {
    pub angle: f32,
    half_length: f32,
    previous_angle: f32,
    center: Vec2
}

impl Bar {
    pub fn new(center: Vec2, half_length: f32) -> Self {
        Self {
            angle: 0.,
            half_length,
            previous_angle: 0.,
            center
        }
    }

    pub fn rotate_positive(&mut self) {
        self.rotate(ROTATE_SPEED);
    }

    pub fn rotate_negative(&mut self) {
        self.rotate(-ROTATE_SPEED);
    }

    fn rotate(&mut self, delta: f32) {
        // requires: delta isn't too big or small.
        assert!(delta.abs() < 90.);

        self.previous_angle = self.angle;
        self.angle += delta;
        if self.angle > MAX_ANGLE {
            self.angle = MAX_ANGLE;
        }
        if self.angle < -MAX_ANGLE {
            self.angle = -MAX_ANGLE;
        }
    }

    pub fn has_pushed_up(&self, point: Vec2) -> bool {
        if self.center.sqr_distance(point) > self.half_length { false }
        else if point.x == self.center.x { false }
        else {
            let angle = f32::atan((point - self.center).slope()) * RAD2DEG;
            (point.x > self.center.x && (angle >= self.previous_angle) && (angle < self.angle))
            || (point.x < self.center.x && (angle <= self.previous_angle) && (angle > self.angle))
        }
    }

    pub fn pushed_up_point(&self, point: Vec2) -> Vec2 {
        // require: `point` has been pushed up.

        let distance = self.center.distance(point);
        if point.x > self.center.x {
            self.center + Vec2::from_angle(self.angle) * distance + Vec2::new(0., 0.001)
        } else {
            self.center - Vec2::from_angle(self.angle) * distance + Vec2::new(0., 0.001)
        }
    }

    pub fn corrected_point(&self, departure: Vec2, delta: Vec2) -> Option<Vec2> {
        if delta.x == 0. {
            if let Some(y) = self.on_bar_y(departure.x) {
                if y >= departure.y || y <= departure.y + delta.y {
                    None
                } else {
                    let remain = y - (departure.y + delta.y);
                    Some(Vec2::new(departure.x, y + 0.001) + Vec2::slope_down(self.angle) * remain)
                }
            } else {
                None
            }
        } else {
            let a_delta = delta.y / delta.x;
            let b_delta = departure.y - a_delta * departure.x;

            let a_bar = f32::tan(self.angle * DEG2RAD);
            let b_bar = self.center.y - a_bar * self.center.x;

            let x = (b_bar - b_delta) / (a_delta - a_bar);
            let y = a_delta * x + b_delta;

            if x < departure.x && x < (departure.x + delta.x) {
                None
            } else if x > departure.x && x > (departure.x + delta.x) {
                None
            } else if x < self.center.x - f32::cos(self.angle * DEG2RAD) * self.half_length {
                None
            } else if x > self.center.x + f32::cos(self.angle * DEG2RAD) * self.half_length {
                None
            } else {
                let intersect = Vec2::new(x, y + 0.001);
                let remain = departure + delta - intersect;
                let inner_product = Vec2::inner_product(Vec2::slope_down(self.angle), remain);
                Some(intersect + Vec2::slope_down(self.angle) * inner_product)
            }
        }
    }

    pub fn on_bar_y(&self, x: f32) -> Option<f32> {
        let y = f32::tan(self.angle * DEG2RAD) * (x - self.center.x) + self.center.y;
        if self.center.sqr_distance(Vec2::new(x, y)) > self.half_length.powf(2.) {
            None
        } else {
            Some(y)
        }
    }
}