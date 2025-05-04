use std::vec;
use std::{thread, time::Duration};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{InputMemory, OutputMemory};
use super::bar::Bar;
use super::bubble::Bubble;
use super::consts::{BAR_HALF_LEN, BAR_LAYOUT, BUBBLE_GRAVITY, BUBBLE_SPAWN_FRAME, BUBBLE_SPAWN_POSITION, DELETE_BORDER, INERTIA};

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
            for frame in BUBBLE_SPAWN_FRAME {
                if frame == count {
                    bubbles[i].push(Bubble::new(BUBBLE_SPAWN_POSITION));
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
                    if let Some(corrected_position) = bar.corrected_point(bubble.position, BUBBLE_GRAVITY + bubble.previous_velocity * INERTIA) {
                        bubble.move_physically(corrected_position);
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    bubble.move_physically(bubble.position + BUBBLE_GRAVITY + bubble.previous_velocity * INERTIA);
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
