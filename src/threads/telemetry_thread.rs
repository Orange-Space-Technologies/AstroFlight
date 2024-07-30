use std::time::{Duration, Instant};
use std::sync::Mutex;

use crate::models::state::State;

use crate::models::sensors_reading::SensorsReading;
use crate::config::TELEMETRY_THREAD_HZ;
use crate::utils::time_loop;

use rppal::uart::{Parity, Uart};

#[allow(unused_variables)]
pub fn telemetry_thread(state: &Mutex<State>, flag_continue_running: &Mutex<bool>, sensors_reading: &Mutex<SensorsReading>) -> Result<String, rppal::uart::Error> {
    // Timing setup
    let target_loop_duration: Duration = std::time::Duration::from_secs_f32(1.0 / TELEMETRY_THREAD_HZ as f32);

    // Setup UART
    let uart = Uart::new(19200, Parity::None, 8, 1);
    if let Err(e) = uart {
        println!("[TELEMETRY] Error initializing UART: {:?}", e);
        return Err(e);
    }
    let mut uart = uart.unwrap();

    if let Err(e) = uart.set_write_mode(true) {
        println!("[TELEMETRY] Error setting write mode: {:?}", e);
        return Err(e);
    }

    loop {
        let loop_start = Instant::now();

        let mutex_lock = sensors_reading.lock();
        if let Ok(reading) = mutex_lock {
            println!("Telemetry: {:?}", *reading);

            // Send to UART
            let buffer: Vec<u8> = (*reading).to_be_bytes();
            if let Err(e) = uart.write(&buffer) {
                println!("Error writing to UART: {:?}", e);
            }
        }

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[TELEMETRY] Exiting...");
                break;
            }
        } else {
            println!("[TELEMETRY] Error checking flag");
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
    return Ok("ok".to_string());
}

