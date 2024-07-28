use std::sync::Mutex;
use queues::{IsQueue, Queue};

use crate::models::sensors_reading::SensorsReading;
use crate::config::LOGGING_THREAD_HZ;
use crate::utils::time_loop;

pub fn logging_thread(sensors_logging_queue: &Mutex<Queue<SensorsReading>>) {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / LOGGING_THREAD_HZ as f32);

    let index: Mutex<i32> = Mutex::new(0);
    let last_index: Mutex<i32> = Mutex::new(-1);
    loop {
        let loop_start = std::time::Instant::now();

        let mut index = index.lock().unwrap();
        let mut last_index = last_index.lock().unwrap();

        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Ok(reading) = lock.remove() {


                *index = reading.pressure as i32;
                if *index != *last_index + 1 && *index != 0{
                    panic!("Missing reading: {} -> {}", *last_index, *index);
                }
                println!("Logging: {:?}", reading);
            }
            
        }
        
        *last_index = (*index).clone();

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}