use super::{bubble::BubbleColor, consts::BUBBLE_NUM};


pub type BubbleSchedule = [BubbleScheduleUnit; BUBBLE_NUM + 1];

pub struct BubbleScheduleUnit {
    pub spawn_frame: i32,
    pub bubble_color: Option<BubbleColor>
}

impl BubbleScheduleUnit {
    pub const fn new(spawn_frame: i32, bubble_color: Option<BubbleColor>) -> Self {
        Self {
            spawn_frame,
            bubble_color
        }
    }
}