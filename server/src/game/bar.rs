use core::f32;

use crate::util::{Vec2, DEG2RAD, RAD2DEG};

const MAX_ANGLE: f32 = 30.;
const HALF_LENGTH: f32 = 1.;
const ROTATE_SPEED: f32 = 1.;

pub struct Bar {
    pub angle: f32,
    previous_angle: f32,
    center: Vec2
}

impl Bar {
    pub fn new(center: Vec2) -> Self {
        Self {
            angle: 0.,
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
        if self.center.sqr_distance(point) > HALF_LENGTH { false }
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
            self.center + Vec2::from_angle(self.angle) * distance + Vec2::new(0., 0.01)
        } else {
            self.center - Vec2::from_angle(self.angle) * distance + Vec2::new(0., 0.01)
        }
    }

    pub fn corrected_point(&self, departure: Vec2, delta: Vec2) -> Option<Vec2> {
        // require: `delta` is parallel to the y-axis.
        assert!(delta.x.abs() < f32::EPSILON);
        // require: `delta` is downward.
        assert!(delta.y <= 0.);

        if let Some(y) = self.on_bar_y(departure.x) {
            if y >= departure.y || y <= departure.y + delta.y {
                None
            } else {
                let remain = y - (departure.y + delta.y);
                Some(Vec2::new(departure.x, y + 0.01) + Vec2::slope_down(self.angle) * remain)
            }
        } else {
            None
        }
    }

    pub fn on_bar_y(&self, x: f32) -> Option<f32> {
        let y = f32::tan(self.angle * DEG2RAD) * (x - self.center.x) + self.center.y;
        if self.center.sqr_distance(Vec2::new(x, y)) > HALF_LENGTH.powf(2.) {
            None
        } else {
            Some(y)
        }
    }
}