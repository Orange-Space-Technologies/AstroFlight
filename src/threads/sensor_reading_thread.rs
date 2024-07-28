use std::sync::Mutex;
use queues::{IsQueue, Queue};
use rand;

use crate::models::sensors_reading::SensorsReading;

pub fn sensor_reading_thread(
    latest_sensors_reading: &Mutex<SensorsReading>,
    sensors_logging_queue: &Mutex<Queue<SensorsReading>>
) {
    let mut index = 0;
    loop {
        // Read sensor data
        let mut sensors_reading = SensorsReading::null();

        // Randomize data
        sensors_reading.pressure = index as f32;
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
        
        index += 1;
        
        // Sleep for 10 milliseconds
        std::thread::sleep(std::time::Duration::from_millis(10));
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