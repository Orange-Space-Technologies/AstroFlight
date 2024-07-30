use std::sync::Mutex;
use std::time::Duration;
use bno055::mint::Vector3;
use queues::{IsQueue, Queue};
use rand;

use crate::models::state::State;
use crate::models::delay::Delay;

use crate::models::sensors_reading::SensorsReading;
use crate::config::SENSOR_READING_THREAD_HZ;
use crate::utils::time_loop;

use bno055;

use rppal::i2c::I2c;

// Simulation
use crate::models::simulation_entry::SimulationEntry;
use crate::utils::parse_simlation_csv;
use std::{env, fs};

fn update_speed_position(acceleration: &Vector3<f32>, sensors_reading: &mut SensorsReading, last_reading: &SensorsReading, target_loop_duration: &Duration) {
    sensors_reading.acc_x = acceleration.x;
    sensors_reading.acc_y = acceleration.y;
    sensors_reading.acc_z = acceleration.z;

    sensors_reading.vel_x = last_reading.vel_x + (acceleration.x * target_loop_duration.as_secs_f32());
    sensors_reading.vel_y = last_reading.vel_y + (acceleration.y * target_loop_duration.as_secs_f32());
    sensors_reading.vel_z = last_reading.vel_z + (acceleration.z * target_loop_duration.as_secs_f32());

    sensors_reading.pos_x = last_reading.pos_x + (sensors_reading.vel_x * target_loop_duration.as_secs_f32());
    sensors_reading.pos_y = last_reading.pos_y + (sensors_reading.vel_y * target_loop_duration.as_secs_f32());
    sensors_reading.pos_z = last_reading.pos_z + (sensors_reading.vel_z * target_loop_duration.as_secs_f32());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sensor_reading_thread(state: &Mutex<State>, flag_continue_running: &Mutex<bool>, latest_sensors_reading: &Mutex<SensorsReading>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) -> Result<String, String> {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / SENSOR_READING_THREAD_HZ as f32);

    let i2c = I2c::new().unwrap();
    let delay = &mut Delay::new();

    let mut imu = bno055::Bno055::new(i2c);
    if let Err(e) = imu.init(delay) {
        println!("[SENSORS] Error initializing BNO055: {:?}", e);
        return Err(e.to_string());
    }

    let acc_config = imu.get_acc_config();
    if let Ok(mut acc_config) =  acc_config{
        acc_config.set_g_range(bno055::AccGRange::G16);
        acc_config.set_bandwidth(bno055::AccBandwidth::Hz125);
        acc_config.set_operation_mode(bno055::AccOperationMode::Normal);
        if let Err(e) = imu.set_acc_config(&acc_config) {
            println!("[SENSORS] Error setting acc config: {:?}", e);
            return Err(e.to_string());
        }
    } else {
        println!("[SENSORS] Error getting acc config");
        return Err(acc_config.unwrap_err().to_string());
    }

    // Wait for calibration to complete
    loop {
        let is_calibrated_result = imu.is_fully_calibrated();
        if let Ok(is_calibrated) = is_calibrated_result {
            if is_calibrated {
                println!("Calibrated!");
                break;
            } else {
                println!("[SENSORS] Calibrating...");
            }
        } else {
            let err = is_calibrated_result.unwrap_err();
            println!("[SENSORS] Error checking calibration: {:?}", &err);
            return Err(err.to_string());
        }
    }

    // Set IMU mode to NDOF (9 degrees of freedom)
    if let Err(e) = imu.set_mode(bno055::BNO055OperationMode::NDOF, delay) {
        println!("[SENSORS] Error setting mode: {:?}", e);
        return Err(e.to_string());
    }

    let mut index: i32 = 0;
    loop {
        let loop_start = std::time::Instant::now();

        let last_reading = (*latest_sensors_reading.lock().unwrap()).clone();

        // Read sensor data
        let mut sensors_reading = SensorsReading::null();

        let acceleration = imu.linear_acceleration();
        if let Ok(acceleration) = acceleration { // Update acceleration, velocity, and position
            update_speed_position(&acceleration, &mut sensors_reading, &last_reading, &target_loop_duration)
            // sensors_reading.acc_x = acceleration.x;
            // sensors_reading.acc_y = acceleration.y;
            // sensors_reading.acc_z = acceleration.z;

            // sensors_reading.vel_x = last_reading.vel_x + (acceleration.x * target_loop_duration.as_secs_f32());
            // sensors_reading.vel_y = last_reading.vel_y + (acceleration.y * target_loop_duration.as_secs_f32());
            // sensors_reading.vel_z = last_reading.vel_z + (acceleration.z * target_loop_duration.as_secs_f32());

            // sensors_reading.pos_x = last_reading.pos_x + (sensors_reading.vel_x * target_loop_duration.as_secs_f32());
            // sensors_reading.pos_y = last_reading.pos_y + (sensors_reading.vel_y * target_loop_duration.as_secs_f32());
            // sensors_reading.pos_z = last_reading.pos_z + (sensors_reading.vel_z * target_loop_duration.as_secs_f32());
        } else { // Something went wrong, but dont exit
            println!("[SENSORS] Error reading acceleration: {:?}", acceleration.unwrap_err());
        }

        // Randomize data
        sensors_reading.pressure = index as f32;
        sensors_reading.altitude = 100.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.temperature = 20.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.gps_latitude = 37.7749 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_longitude = -122.4194 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_altitude = 100.0 + (rand::random::<f32>() * 10.0);

        
        // Update latest sensor reading
        if let Ok(mut latest_sensors_reading) = latest_sensors_reading.lock() {
            *latest_sensors_reading = sensors_reading.clone();
        } else {
            println!("[SENSORS] Error updating latest sensor reading");
        }
        
        // Log sensor data
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Err(err) = lock.add(sensors_reading.clone()) {
                println!("[SENSORS] Error adding to queue: {:?}", err);
            }
        }
        
        index += 1;

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[SENSORS] Exiting...");
                break;
            }
        } else {
            println!("[SENSORS] Error checking flag");
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    };
    return Ok("ok".to_string());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn software_in_the_loop_sensor_reading_thread(state: &Mutex<State>, flag_continue_running: &Mutex<bool>, latest_sensors_reading: &Mutex<SensorsReading>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) -> Result<String, String> {
    // Timing setup
    // let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / SENSOR_READING_THREAD_HZ as f32);

    // Get FILENAME from env
    #[allow(non_snake_case)]
    let SIM_FILENAME: String;
    if env::var("FILENAME").is_ok() {
        SIM_FILENAME = env::var("FILENAME").unwrap();
    } else {
        println!("[SENSORS SIM] FILENAME not set");
        *flag_continue_running.lock().unwrap() = false;
        return Err("FILENAME not set".to_string());
    }

    println!("[SENSORS SIM] Reading simulation data from: {}", SIM_FILENAME);
    let sim_reader = fs::read_to_string(SIM_FILENAME).unwrap();
    let sim_data = sim_reader.as_str();
    let sim_data = parse_simlation_csv(sim_data);

    if let Err(e) = sim_data {
        println!("[SENSORS SIM] Error parsing simulation data: {:?}", e);
        return Err("Error parsing simulation data".to_string());
    }
    let sim_data = sim_data.unwrap();

    let mut index: i32 = 0;
    for i in 0..sim_data.len() {
        let entry: SimulationEntry = sim_data[i];

        let loop_start = std::time::Instant::now();

        let last_reading = (*latest_sensors_reading.lock().unwrap()).clone();
        
        // Read sensor data
        let mut sensors_reading = SensorsReading::null();

        let velocity: Vector3<f32> = Vector3 {
            x: 0.0,
            y: 0.0,
            z: entry.vertical_acceleration as f32,
        };

        let time_from_reading: Duration;
        if i == 0 {
            time_from_reading = Duration::from_secs_f64(entry.time);
        } else {
            time_from_reading = Duration::from_secs_f64(entry.time - sim_data[i-1].time);
        }
        update_speed_position(&velocity, &mut sensors_reading, &last_reading, &time_from_reading);

        // Randomize data
        sensors_reading.pressure = index as f32;

        
        // Update latest sensor reading
        *latest_sensors_reading.lock().unwrap() = sensors_reading.clone();
        
        // Log sensor data
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Err(err) = lock.add(sensors_reading.clone()) {
                println!("[SENSORS SIM] Error adding to queue: {:?}", err);
            }
        }
        
        println!("Calculated altitude: {}, Simulation altitude: {}", sensors_reading.pos_z, entry.altitude);

        index += 1;

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[SENSORS SIM] Exiting...");
                break;
            }
        } else {
            println!("[SENSORS SIM] Error checking flag");
        }

        // Time loop
        if i == sim_data.len()-1 {
            *flag_continue_running.lock().unwrap() = false;
            break;
        }
        let time_to_sleep = Duration::from_secs_f64(sim_data[i+1].time - entry.time);
        time_loop(time_to_sleep, loop_start);
    }
    Ok("ok".to_string())
}