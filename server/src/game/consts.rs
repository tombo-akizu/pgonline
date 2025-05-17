use crate::vec2::Vec2;
use super::bubble::BubbleColor;

pub const BAR_LAYOUT: [Vec2; 6] = [
    Vec2::new(0., 10.6),
    Vec2::new(-2.6, 8.1),
    Vec2::new(2.8, 7.4),
    Vec2::new(-1.3, 5.4),
    Vec2::new(2.7, 4.3),
    Vec2::new(0.1, 0.8)
];

pub const BAR_HALF_LEN: [f32; 6] = [
    0.97,
    0.78,
    0.97,
    0.97,
    0.97,
    1.36
];

pub const BUBBLE_SPAWN_FRAME: [i32; 10] = [
    100,
    200,
    300,
    400,
    500,
    600,
    700,
    800,
    900,
    1000
];

pub const BUBBLE_COLORS: [BubbleColor; 10] = [
    BubbleColor::Red,
    BubbleColor::White,
    BubbleColor::White,
    BubbleColor::Red,
    BubbleColor::White,
    BubbleColor::White,
    BubbleColor::Red,
    BubbleColor::Red,
    BubbleColor::Red,
    BubbleColor::White
];

pub const BUBBLE_SPAWN_POSITION: Vec2 = Vec2::new(0., 15.);
pub const INERTIA: f32 = 0.96;

pub const BUBBLE_GRAVITY: Vec2 = Vec2::new(0., -0.01);
pub const DELETE_BORDER: f32 = 0.;

pub const RED_CONTAINER_MIN: f32 = -3.;
pub const WHITE_CONTAINER_MIN: f32 = 1.;
pub const CONTAINER_WIDTH: f32 = 2.0;