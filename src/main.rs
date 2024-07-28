use std::sync::{Arc, Mutex};
use std::thread;

mod models;
use models::sensors_reading::SensorsReading;

mod threads;
use threads::sensor_reading_thread::sensor_reading_thread;

extern crate queues;
use queues::Queue;

// Create mutex for shared data (current sensor reading)

fn main() {
    let latest_sensors_reading: Arc<Mutex<SensorsReading>> = Arc::new(Mutex::new(SensorsReading::null()));
    let sensors_logging_queue: Arc<Mutex<Queue<SensorsReading>>> = Arc::new(Mutex::new(Queue::new()));
    // let latest_sensors_reading: Mutex<SensorsReading> = Mutex::new(SensorsReading::null());
    // let mut sensors_logging_queue: Mutex<Queue<SensorsReading>> = Mutex::new(Queue::new());

    let sensors_reading_clone_1 = latest_sensors_reading.clone();
    let sensors_queue_clone_1 = sensors_logging_queue.clone();
    let sensor_reading_thread_handle = thread::spawn(move ||{
        sensor_reading_thread(&sensors_reading_clone_1, &sensors_queue_clone_1);
    });

    let sensors_queue_clone_2 = sensors_logging_queue.clone();
    let logging_thread_handle = thread::spawn(move ||{
        threads::logging_thread::logging_thread(&sensors_queue_clone_2);
    });
    

    sensor_reading_thread_handle.join().unwrap();
    logging_thread_handle.join().unwrap();
}
