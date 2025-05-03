use std::{thread, time::Duration};
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{InputMemory, OutputMemory};

pub async fn game(
    input_memory: Arc<Mutex<InputMemory>>,
    output_memory: Arc<Mutex<OutputMemory>>,
) {
    let tick_duration = Duration::from_millis(64);
    let mut xs = [0., 0.];
    let zs = [0., 1.];

    loop {
        let start = std::time::Instant::now();

        for i in 0..2 {
            if input_memory.lock().await.right_inputs[i] {
                xs[i] += 0.1;
            }
            if input_memory.lock().await.left_inputs[i] {
                xs[i] += -0.1;
            }
            output_memory.lock().await.xs[i] = xs[i];
            output_memory.lock().await.zs[i] = zs[i];            
        }

        let elapsed = start.elapsed();
        if let Some(sleep_time) = tick_duration.checked_sub(elapsed) {
            thread::sleep(sleep_time);
        }
    }
}
