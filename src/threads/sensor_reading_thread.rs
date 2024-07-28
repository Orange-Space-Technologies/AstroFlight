use std::sync::Mutex;
use queues::{IsQueue, Queue};
use rand;

use crate::models::sensors_reading::SensorsReading;
use crate::config::SENSOR_READING_THREAD_HZ;
use crate::utils::time_loop;

pub fn sensor_reading_thread(latest_sensors_reading: &Mutex<SensorsReading>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / SENSOR_READING_THREAD_HZ as f32);

    let index: Mutex<i32> = Mutex::new(0);
    loop {
        let loop_start = std::time::Instant::now();


        let mut index = index.lock().unwrap();

        // Read sensor data
        let mut sensors_reading = SensorsReading::null();

        // Randomize data
        sensors_reading.pressure = (*index) as f32;
        sensors_reading.altitude = 100.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.temperature = 20.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.pos_x = rand::random::<f32>() * 100.0;
        sensors_reading.pos_y = rand::random::<f32>() * 100.0;
        sensors_reading.pos_z = rand::random::<f32>() * 100.0;
        sensors_reading.vel_x = rand::random::<f32>() * 10.0;
        sensors_reading.vel_y = rand::random::<f32>() * 10.0;
        sensors_reading.vel_z = rand::random::<f32>() * 10.0;
        sensors_reading.acc_x = rand::random::<f32>() * 1.0;
        sensors_reading.acc_y = rand::random::<f32>() * 1.0;
        sensors_reading.acc_z = rand::random::<f32>() * 1.0;
        sensors_reading.gps_latitude = 37.7749 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_longitude = -122.4194 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_altitude = 100.0 + (rand::random::<f32>() * 10.0);

        
        // Update latest sensor reading
        *latest_sensors_reading.lock().unwrap() = sensors_reading.clone();
        
        // Log sensor data
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Err(err) = lock.add(sensors_reading.clone()) {
                println!("Error adding to queue: {:?}", err);
            }
        }
        
        *index += 1;

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}

#[allow(dead_code)]
pub fn software_in_the_loop_sensor_reading_thread(
    #[allow(unused_variables)]
    latest_sensors_reading: &Mutex<SensorsReading>,
    #[allow(unused_variables)]
    sensors_logging_queue: &Mutex<Queue<SensorsReading>>
) {
    loop {
        // Read sensor data from simulation, blah blah blah
    }
}