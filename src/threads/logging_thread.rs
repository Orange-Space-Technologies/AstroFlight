use std::sync::Mutex;
use queues::{IsQueue, Queue};

use crate::models::sensors_reading::SensorsReading;

pub fn logging_thread(
    sensors_logging_queue: &Mutex<Queue<SensorsReading>>
) {
    let mut index = 0;
    let mut last_index = -1;
    loop {
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Ok(reading) = lock.remove() {
                index = reading.pressure as i32;
                if index != last_index + 1 && index != 0{
                    panic!("Missing reading: {} -> {}", last_index, index);
                }
                println!("Logging: {:?}", reading);
            }
            
        }
        
        last_index = index.clone();
        // Sleep for 10 milliseconds
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}