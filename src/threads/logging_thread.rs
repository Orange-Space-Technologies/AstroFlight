use std::sync::Mutex;
use queues::{IsQueue, Queue};

use crate::models::sensors_reading::SensorsReading;
use crate::config::LOGGING_THREAD_HZ;
use crate::utils::time_loop;

#[allow(unused_variables)]
pub fn logging_thread(flag_continue_running: &Mutex<bool>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) -> Result<String, String> {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / LOGGING_THREAD_HZ as f32);

    loop {
        let loop_start = std::time::Instant::now();

        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Ok(reading) = lock.remove() {
                println!("Logging: {:?}", reading);
            }
        }

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[LOGGING] Exiting...");
                break Ok("ok".to_string());
            }
        } else {
            println!("[LOGGING] Error checking flag");
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}