use std::sync::Mutex;
use std::time::Duration;
use bno055::mint::Vector3;
use queues::{IsQueue, Queue};
use rand;

use crate::models::sensors_reading::SensorsReading;
use crate::models::delay::Delay;
use crate::config::SENSOR_READING_THREAD_HZ;
use crate::utils::time_loop;

use bno055;

use rppal::i2c::I2c;

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
pub fn sensor_reading_thread(flag_continue_running: &Mutex<bool>, latest_sensors_reading: &Mutex<SensorsReading>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / SENSOR_READING_THREAD_HZ as f32);

    let i2c = I2c::new().unwrap();
    let delay = &mut Delay::new();

    let mut imu = bno055::Bno055::new(i2c);
    if let Err(e) = imu.init(delay) {
        println!("[SENSORS] Error initializing BNO055: {:?}", e);
        // return Err(e);
    }

    let acc_config = imu.get_acc_config();
    if let Ok(mut acc_config) =  acc_config{
        acc_config.set_g_range(bno055::AccGRange::G16);
        acc_config.set_bandwidth(bno055::AccBandwidth::Hz125);
        acc_config.set_operation_mode(bno055::AccOperationMode::Normal);
        if let Err(e) = imu.set_acc_config(&acc_config) {
            println!("[SENSORS] Error setting acc config: {:?}", e);
            // return Err(e);
        }
    } else {
        println!("[SENSORS] Error getting acc config");
        // return Err(acc_config.unwrap_err());
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
            println!("[SENSORS] Error checking calibration: {:?}", is_calibrated_result.unwrap_err());
            // return Err(is_calibrated_result.unwrap_err());
        }
    }

    // Set IMU mode to NDOF (9 degrees of freedom)
    if let Err(e) = imu.set_mode(bno055::BNO055OperationMode::NDOF, delay) {
        println!("[SENSORS] Error setting mode: {:?}", e);
        // return Err(e);
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
        *latest_sensors_reading.lock().unwrap() = sensors_reading.clone();
        
        // Log sensor data
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Err(err) = lock.add(sensors_reading.clone()) {
                println!("[SENSORS] Error adding to queue: {:?}", err);
            }
        }
        
        index += 1;

        if !(*flag_continue_running.lock().unwrap()) {
            println!("[SENSORS] Exiting...");
            break;
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    };
    // return Ok(());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn software_in_the_loop_sensor_reading_thread(flag_continue_running: &Mutex<bool>, latest_sensors_reading: &Mutex<SensorsReading>, sensors_logging_queue: &Mutex<Queue<SensorsReading>>) {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / SENSOR_READING_THREAD_HZ as f32);

    let mut index: i32 = 0;
    loop {
        let loop_start = std::time::Instant::now();

        let last_reading = (*latest_sensors_reading.lock().unwrap()).clone();
        
        // Read sensor data
        let mut sensors_reading = SensorsReading::null();

        let velocity: Vector3<f32> = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.05,
        };

        update_speed_position(&velocity, &mut sensors_reading, &last_reading, &target_loop_duration);

        // Randomize data
        sensors_reading.pressure = index as f32;
        sensors_reading.altitude = 100.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.temperature = 20.0 + (rand::random::<f32>() * 10.0);
        sensors_reading.gps_latitude = 37.7749 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_longitude = -122.4194 + (rand::random::<f32>() * 0.1);
        sensors_reading.gps_altitude = 100.0 + (rand::random::<f32>() * 10.0);

        
        // Update latest sensor reading
        *latest_sensors_reading.lock().unwrap() = sensors_reading.clone();
        
        // Log sensor data
        let queue_lock = sensors_logging_queue.lock();
        if let Ok(mut lock) = queue_lock {
            if let Err(err) = lock.add(sensors_reading.clone()) {
                println!("[SENSORS] Error adding to queue: {:?}", err);
            }
        }
        
        index += 1;

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}