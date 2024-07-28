use std::sync::Mutex;

use crate::models::sensors_reading::SensorsReading;
use crate::config::TELEMETRY_THREAD_HZ;
use crate::utils::time_loop;

pub fn telemetry_thread(sensors_reading: &Mutex<SensorsReading>) {
    time_loop(TELEMETRY_THREAD_HZ, &|| {
        let mutex_lock = sensors_reading.lock();
        if let Ok(reading) = mutex_lock {
            println!("Telemetry: {:?}", reading);
        }
    });
}

