use std::sync::{Arc, Mutex};
use std::thread;

mod models;
use models::sensors_reading::SensorsReading;

mod threads;
use threads::sensor_reading_thread::{sensor_reading_thread, software_in_the_loop_sensor_reading_thread};

extern crate queues;
use queues::Queue;

pub mod config;
pub mod utils;

// Create mutex for shared data (current sensor reading)

fn main() {
    let latest_sensors_reading: Arc<Mutex<SensorsReading>> = Arc::new(Mutex::new(SensorsReading::null()));
    let sensors_logging_queue: Arc<Mutex<Queue<SensorsReading>>> = Arc::new(Mutex::new(Queue::new()));

    let flag_continue_running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    // let latest_sensors_reading: Mutex<SensorsReading> = Mutex::new(SensorsReading::null());
    // let mut sensors_logging_queue: Mutex<Queue<SensorsReading>> = Mutex::new(Queue::new());

    // SENSOR READING THREAD
    let flag_continue_running_clone1 = flag_continue_running.clone();
    let latest_sensors_reading_clone1 = latest_sensors_reading.clone();
    let sensors_logging_queue_clone1 = sensors_logging_queue.clone();
    let sensor_reading_thread_handle = thread::spawn(move ||{
        // return sensor_reading_thread(&flag_continue_running_clone1, &latest_sensors_reading_clone1, &sensors_logging_queue_clone1);
        return software_in_the_loop_sensor_reading_thread(&flag_continue_running_clone1, &latest_sensors_reading_clone1, &sensors_logging_queue_clone1);
    });

    // LOGGING THREAD
    let flag_continue_running_clone2 = flag_continue_running.clone();
    let sensors_logging_queue_clone2 = sensors_logging_queue.clone();
    let logging_thread_handle = thread::spawn(move ||{
        return threads::logging_thread::logging_thread(&flag_continue_running_clone2, &sensors_logging_queue_clone2);
    });

    // TELEMETRY THREAD
    let flag_continue_running_clone3 = flag_continue_running.clone();
    let latest_sensors_reading_clone2 = latest_sensors_reading.clone();

    let telemetry_thread_handle = thread::spawn(move ||{
        return threads::telemetry_thread::telemetry_thread(&flag_continue_running_clone3, &latest_sensors_reading_clone2);
    });
    

    sensor_reading_thread_handle.join().unwrap();
    logging_thread_handle.join().unwrap();
    let _ = telemetry_thread_handle.join().unwrap();
}
