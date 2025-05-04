use std::vec;
use std::{thread, time::Duration};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{InputMemory, OutputMemory};
use crate::util::Vec2;
use super::bar::Bar;
use super::bubble::Bubble;

const BAR_LAYOUT: [Vec2; 6] = [
    Vec2::new(0., 10.6),
    Vec2::new(-2.6, 8.1),
    Vec2::new(2.8, 7.4),
    Vec2::new(-1.3, 5.4),
    Vec2::new(2.7, 4.3),
    Vec2::new(0.1, 0.8)
];

const BAR_HALF_LEN: [f32; 6] = [
    0.97,
    0.78,
    0.97,
    0.97,
    0.97,
    1.36
];

const BUBBLE_FRAME: [i32; 10] = [
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

const BUBBLE_DELTA: Vec2 = Vec2::new(0., -0.01);
const DELETE_BORDER: f32 = 0.;

pub async fn game(
    input_memory: Arc<Mutex<InputMemory>>,
    output_memory: Arc<Mutex<OutputMemory>>,
) {
    let tick_duration = Duration::from_millis(32);

    let mut count = 0;

    let mut bars: Vec<Vec<Bar>> = vec![vec![], vec![]];
    for i in 0..2 {
        for j in 0..(BAR_LAYOUT.len()) {
            bars[i].push(Bar::new(BAR_LAYOUT[j], BAR_HALF_LEN[j]));
        }
    }

    let mut bubbles: Vec<Vec<Bubble>> = vec![vec![], vec![]];

    loop {
        let start = std::time::Instant::now();

        count += 1;

        {
            let input = input_memory.lock().await;
            for i in 0..2 {
            
                if input.right_inputs[i] {
                    for bar in &mut bars[i] {
                        bar.rotate_positive();
                    }
                }
                if input.left_inputs[i] {
                    for bar in &mut bars[i] {
                        bar.rotate_negative();
                    }
                }
    
            }
        }

        for i in 0..2 {
            for frame in BUBBLE_FRAME {
                if frame == count {
                    bubbles[i].push(Bubble::new(Vec2::new(0., 15.)));
                }
            }

            for bubble in &mut bubbles[i] {
                for bar in &bars[i] {
                    if bar.has_pushed_up(bubble.position) {
                        bubble.set_position(bar.pushed_up_point(bubble.position));
                    }
                }
            }

            for bubble in &mut bubbles[i] {
                let mut flag = false;
                for bar in &bars[i] {
                    if let Some(corrected_position) = bar.corrected_point(bubble.position, BUBBLE_DELTA + bubble.previous_velocity * 0.96) {
                        bubble.move_physically(corrected_position);
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    bubble.move_physically(bubble.position + BUBBLE_DELTA + bubble.previous_velocity * 0.96);
                }
            }

            bubbles[i] = bubbles[i]
                .clone()
                .into_iter()
                .filter(|bubble| bubble.position.y >= DELETE_BORDER)
                .collect();
        }

        for i in 0..2 {
            let mut output = output_memory.lock().await;
            output.angles[i] = bars[i][0].angle;
            output.bubble_positions[i] = bubbles[i]
                .iter()
                .map(|bubble| bubble.position)
                .collect();
        }

        let elapsed = start.elapsed();
        if let Some(sleep_time) = tick_duration.checked_sub(elapsed) {
            thread::sleep(sleep_time);
        }
    }
}
