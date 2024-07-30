use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

mod models;

use models::state::State;
use models::sensors_reading::SensorsReading;

mod threads;
#[allow(unused_imports)]
use threads::sensor_reading_thread::{sensor_reading_thread, software_in_the_loop_sensor_reading_thread};

extern crate queues;
use queues::Queue;

pub mod config;
pub mod utils;

// Create mutex for shared data (current sensor reading)

fn main() {
    #[allow(non_snake_case)]
    let TESTING: bool = env::var("TESTING").is_ok();
    #[allow(non_snake_case)]
    let X86: bool = env::var("X86").is_ok();

    let latest_sensors_reading: Arc<Mutex<SensorsReading>> = Arc::new(Mutex::new(SensorsReading::null()));
    let sensors_logging_queue: Arc<Mutex<Queue<SensorsReading>>> = Arc::new(Mutex::new(Queue::new()));

    let flag_continue_running: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));

    // SENSOR READING THREAD
    let state_clone1 = state.clone();

    let flag_continue_running_clone1 = flag_continue_running.clone();
    let latest_sensors_reading_clone1 = latest_sensors_reading.clone();
    let sensors_logging_queue_clone1 = sensors_logging_queue.clone();
    //spawn
    let sensor_reading_thread_handle = thread::Builder::new().name("sensor_reading".to_string()).spawn(move ||{
        if TESTING || X86 {
            return software_in_the_loop_sensor_reading_thread(&state_clone1, &flag_continue_running_clone1, &latest_sensors_reading_clone1, &sensors_logging_queue_clone1);
        } else {
            return sensor_reading_thread(&state_clone1, &flag_continue_running_clone1, &latest_sensors_reading_clone1, &sensors_logging_queue_clone1);
        }
    }).expect("Cannot create sensor reading thread");

    // LOGGING THREAD
    let state_clone2 = state.clone();

    let flag_continue_running_clone2 = flag_continue_running.clone();
    let sensors_logging_queue_clone2 = sensors_logging_queue.clone();
    //spawn
    let logging_thread_handle = thread::Builder::new().name("logging".to_string()).spawn(move ||{
        return threads::logging_thread::logging_thread(&state_clone2, &flag_continue_running_clone2, &sensors_logging_queue_clone2);
    }).expect("Cannot create logging thread");

    // TELEMETRY THREAD
    let state_clone3 = state.clone();

    let flag_continue_running_clone3 = flag_continue_running.clone();
    let latest_sensors_reading_clone2 = latest_sensors_reading.clone();
    //spawn
    let telemetry_thread_handle = thread::Builder::new().name("telemetry".to_string()).spawn(move ||{
        if X86 {println!("[MAIN] Running on x86, turning telemetry off"); return Ok("X86, turning telemetry off".to_string());}
        return threads::telemetry_thread::telemetry_thread(&state_clone3, &flag_continue_running_clone3, &latest_sensors_reading_clone2);
    }).expect("Cannot create telemetry thread");

    // CONTROL THREAD
    let state_clone4 = state.clone();

    let flag_continue_running_clone4 = flag_continue_running.clone();
    let latest_sensors_reading_clone3 = latest_sensors_reading.clone();
    // spawn
    let control_thread_handle = thread::Builder::new().name("control".to_string()).spawn(move ||{
        return threads::control_thread::control_thread(&state_clone4, &flag_continue_running_clone4, &latest_sensors_reading_clone3);
    }).expect("Cannot create control thread");

    
    loop {
        if sensor_reading_thread_handle.is_finished() {
            println!("[MAIN] [WARN] Sensor reading thread finished");
        }
        if logging_thread_handle.is_finished() {
            println!("[MAIN] [WARN] Logging thread finished");
        }
        if telemetry_thread_handle.is_finished() {
            println!("[MAIN] [WARN] Telemetry thread finished");
        }
        if control_thread_handle.is_finished() {
            println!("[MAIN] [WARN] Control thread finished");
        }
        if sensor_reading_thread_handle.is_finished() && logging_thread_handle.is_finished() && telemetry_thread_handle.is_finished() && control_thread_handle.is_finished() {
            break;
        }
        // sleep for 500ms
        thread::sleep(std::time::Duration::from_millis(500));
    }

    let mut exit_code = 0;

    let sensors_reading_result = sensor_reading_thread_handle.join().unwrap();
    match sensors_reading_result {
        Ok(_) => println!("[MAIN] Sensor reading thread finished successfully"),
        Err(e) => { println!("[MAIN] Sensor reading thread finished with error: {:?}", e); exit_code = 1 }
    }
    
    let logging_result = logging_thread_handle.join().unwrap();
    match logging_result {
        Ok(_) => println!("[MAIN] Logging thread finished successfully"),
        Err(e) => { println!("[MAIN] Logging thread finished with error: {:?}", e); exit_code = 1 }
    }
    
    let telemetry_result = telemetry_thread_handle.join().unwrap();
    match telemetry_result {
        Ok(_) => println!("[MAIN] Telemetry thread finished successfully"),
        Err(e) => { println!("[MAIN] Telemetry thread finished with error: {:?}", e); exit_code = 1 }
    }

    let control_result = control_thread_handle.join().unwrap();
    match control_result {
        Ok(_) => println!("[MAIN] Control thread finished successfully"),
        Err(e) => { println!("[MAIN] Control thread finished with error: {:?}", e); exit_code = 1 }
    }

    std::process::exit(exit_code);
}
