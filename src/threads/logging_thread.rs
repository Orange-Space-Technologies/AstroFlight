use std::io::Write;
use std::sync::Mutex;
use queues::{IsQueue, Queue};

use chrono;

use crate::config;

use crate::models::state::State;

use crate::models::sensors_reading::SensorsReading;
use crate::config::LOGGING_THREAD_HZ;
use crate::utils::time_loop;

#[allow(unused_variables)]
pub fn logging_thread(state: &Mutex<State>, flag_continue_running: &Mutex<bool>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) -> Result<String, String> {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / LOGGING_THREAD_HZ as f32);

    let logging_filename = config::LOGGING_FILENAME.replace("{}", &format!("{}", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")));
    let mut file = match std::fs::File::create(&logging_filename) {
        Ok(file) => file,
        Err(e) => {
            println!("[LOGGING] Error creating file: {}", e);
            return Err("Error creating file".to_string());
        }
    };
    // Write header
    file.write_all(b"# Time (ms), Stage, Pressure, Altitude (m), Temperature (C), Position X, Position Y, Position Z, Velocity X, Velocity Y, Velocity Z, Acceleration X, Acceleration Y, Acceleration Z, GPS Latitude, GPS Longitude, GPS Altitude\n");

    loop {
        let loop_start = std::time::Instant::now();

        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Ok(reading) = lock.remove() {
                if let Ok(state) = state.lock() {
                    let state_csv = state.to_csv();
                    let reading_csv = reading.to_csv();
                    let logging_string = format!("{},{}\n", state_csv, reading_csv);
                    match file.write_all(logging_string.as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("[LOGGING] Error writing to file: {}", e);
                        }
                    }
                }
            }
        }

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[LOGGING] Exiting...");
                file.sync_all().unwrap();
                file.flush().unwrap();
                break Ok("ok".to_string());
            }
        } else {
            println!("[LOGGING] Error checking flag");
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}