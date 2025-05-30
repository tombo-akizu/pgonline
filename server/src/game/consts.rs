use crate::vec2::Vec2;
use super::{bubble::BubbleColor, bubble_schedule::{BubbleSchedule, BubbleScheduleUnit}};

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

pub const BUBBLE_NUM: usize = 10;
pub const BUBBLE_SCHEDULE: BubbleSchedule = [
    BubbleScheduleUnit::new(100,  Some(BubbleColor::Red  )),
    BubbleScheduleUnit::new(200,  Some(BubbleColor::White)),
    BubbleScheduleUnit::new(300,  Some(BubbleColor::White)),
    BubbleScheduleUnit::new(400,  Some(BubbleColor::Red  )),
    BubbleScheduleUnit::new(500,  Some(BubbleColor::White)),
    BubbleScheduleUnit::new(600,  Some(BubbleColor::White)),
    BubbleScheduleUnit::new(700,  Some(BubbleColor::Red  )),
    BubbleScheduleUnit::new(800,  Some(BubbleColor::Red  )),
    BubbleScheduleUnit::new(900,  Some(BubbleColor::Red  )),
    BubbleScheduleUnit::new(1000, Some(BubbleColor::White)),
    BubbleScheduleUnit::new(1200, None)
];

pub const BUBBLE_SPAWN_POSITION: Vec2 = Vec2::new(0., 15.);
pub const INERTIA: f32 = 0.96;

pub const BUBBLE_GRAVITY: Vec2 = Vec2::new(0., -0.01);
pub const DELETE_BORDER: f32 = 0.;

pub const RED_CONTAINER_MIN: f32 = -3.;
pub const WHITE_CONTAINER_MIN: f32 = 1.;
pub const CONTAINER_WIDTH: f32 = 2.0;