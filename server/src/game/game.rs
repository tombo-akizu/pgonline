use std::vec;
use std::{thread, time::Duration};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{InputMemory, OutputMemory};
use super::bar::Bar;
use super::bubble::{Bubble, BubbleColor};
use super::consts;
use super::container::Container;

pub async fn game(
    input_memory: Arc<Mutex<InputMemory>>,
    output_memory: Arc<Mutex<OutputMemory>>,
) {
    let tick_duration = Duration::from_millis(32);

    let mut count = 0;

    let mut bars: Vec<Vec<Bar>> = vec![vec![], vec![]];
    for i in 0..2 {
        for j in 0..(consts::BAR_LAYOUT.len()) {
            bars[i].push(
                Bar::new(consts::BAR_LAYOUT[j], consts::BAR_HALF_LEN[j])
            );
        }
    }

    let mut bubbles: Vec<Vec<Bubble>> = vec![vec![], vec![]];

    let containers = [
        Container::new(
            consts::RED_CONTAINER_MIN,
            consts::RED_CONTAINER_MIN + consts::CONTAINER_WIDTH,
            BubbleColor::Red
        ),
        Container::new(
            consts::WHITE_CONTAINER_MIN,
            consts::WHITE_CONTAINER_MIN + consts::CONTAINER_WIDTH,
            BubbleColor::White
        )
    ];

    let mut scores: [i8; 2] = [0, 0];

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
            for (frame_index, frame) in consts::BUBBLE_SPAWN_FRAME.iter().enumerate() {
                if *frame == count {
                    bubbles[i].push(
                        Bubble::new(
                            consts::BUBBLE_SPAWN_POSITION, 
                            consts::BUBBLE_COLORS[frame_index]
                        )
                    );
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
                    if let Some(corrected_position) = bar.corrected_point(
                            bubble.position, 
                            consts::BUBBLE_GRAVITY + bubble.previous_velocity * consts::INERTIA
                        ) {
                        bubble.move_physically(corrected_position);
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    bubble.move_physically(bubble.position + consts::BUBBLE_GRAVITY + bubble.previous_velocity * consts::INERTIA);
                }
            }

            for bubble in &bubbles[i] {
                for container in &containers {
                    if container.in_container(bubble.position)
                        && container.is_correct_container(bubble.color) {
                        scores[i] += 1;
                    }
                }
            }

            bubbles[i] = bubbles[i]
                .clone()
                .into_iter()
                .filter(|bubble| bubble.position.y >= consts::DELETE_BORDER)
                .collect();
        }

        for i in 0..2 {
            let mut output = output_memory.lock().await;
            output.angles[i] = bars[i][0].angle;
            output.scores[i] = scores[i];
            output.bubble_positions[i] = bubbles[i]
                .iter()
                .map(|bubble| bubble.position)
                .collect();
            output.bubble_colors[i] = bubbles[i]
                .iter()
                .map(|bubble| bubble.color)
                .collect();
        }

        let elapsed = start.elapsed();
        if let Some(sleep_time) = tick_duration.checked_sub(elapsed) {
            thread::sleep(sleep_time);
        }
    }
}
