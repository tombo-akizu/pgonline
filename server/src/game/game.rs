use std::vec;
use std::{thread, time::Duration};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{InputMemory, OutputMemory};
use crate::util::Vec2;
use super::bar::Bar;
use super::bubble::Bubble;

const BAR_LAYOUT: [Vec2; 3] = [
    Vec2::new(-1., 2.),
    Vec2::new(1., 1.5),
    Vec2::new(0., 1.)
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

const BUBBLE_DELTA: Vec2 = Vec2::new(0., -0.5);
const DELETE_BORDER: f32 = 0.;

pub async fn game(
    input_memory: Arc<Mutex<InputMemory>>,
    output_memory: Arc<Mutex<OutputMemory>>,
) {
    let tick_duration = Duration::from_millis(32);

    let mut count = 0;

    let mut bars: Vec<Vec<Bar>> = vec![vec![], vec![]];
    for i in 0..2 {
        for point in BAR_LAYOUT {
            bars[i].push(Bar::new(point));
        }
    }

    let mut bubbles: Vec<Vec<Bubble>> = vec![vec![], vec![]];

    loop {
        let start = std::time::Instant::now();

        count += 1;

        for i in 0..2 {
            if input_memory.lock().await.right_inputs[i] {
                for bar in &mut bars[i] {
                    bar.rotate(0.1);
                }
            }
            if input_memory.lock().await.left_inputs[i] {
                for bar in &mut bars[i] {
                    bar.rotate(-0.1);
                }
            }

        }

        for i in 0..2 {
            for frame in BUBBLE_FRAME {
                if frame == count {
                    bubbles[i].push(Bubble::new(Vec2::new(1., 3.)));
                }
            }

            for bubble in &mut bubbles[i] {
                for bar in &bars[i] {
                    if bar.has_pushed_up(bubble.position) {
                        bubble.position = bar.pushed_up_point(bubble.position);
                    }
                }
            }

            for bubble in &mut bubbles[i] {
                for bar in &bars[i] {
                    if let Some(corrected_position) = bar.corrected_point(bubble.position, BUBBLE_DELTA) {
                        bubble.position = corrected_position;
                        break;
                    }
                    bubble.position = bubble.position + BUBBLE_DELTA;
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
