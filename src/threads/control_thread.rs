use std::sync::Mutex;
use std::time::Instant;

use crate::models::stage::Stage;
use crate::models::state::State;

use crate::config::CONTROL_THREAD_HZ;
use crate::models::sensors_reading::SensorsReading;
use crate::utils::time_loop;

use crate::config;

#[allow(unused_variables)]
pub fn control_thread(
    state: &Mutex<State>,
    flag_continue_running: &Mutex<bool>,
    sensors_reading: &Mutex<SensorsReading>,
) -> Result<String, String> {
    // Timing setup
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / CONTROL_THREAD_HZ as f32);

    // let mut last_reading: SensorsReading = SensorsReading::null();
    let mut max_altitude: f32 = 0.0;

    let control_start: Instant = Instant::now();
    // let mut launch_time: Instant = Instant::now();

    loop {
        let loop_start = std::time::Instant::now();

        if let Ok(reading) = sensors_reading.lock() {
            if reading.pos_z > max_altitude {
                max_altitude = reading.pos_z;
            }
            if let Ok(mut state) = state.lock() {
                let stage = (*state).get_stage();

                // println!("[CONTROL] Current stage: {:?}", stage);
                if stage == Stage::Init {
                    // if control_start.elapsed().as_secs() > 10 {
                    state.set_stage(Stage::PadIdle);
                    // }
                } else if stage == Stage::PadIdle {
                    if reading.acc_z > config::LAUNCH_TRESHOLD {
                        state.launch_time = Instant::now();
                        println!("\n\n\n[CONTROL] Launch detected!");
                        println!("[CONTROL] Time: {:?}", control_start.elapsed());
                        state.set_stage(Stage::Launch);
                    }
                } else if stage == Stage::Launch {
                    if reading.acc_z < config::BURNOUT_TRESHOLD {
                        state.burnout_time = Instant::now();
                        println!("\n\n\n[CONTROL] Burnout detected!");
                        println!("[CONTROL] Time: {:?}", state.launch_time.elapsed());
                        state.set_stage(Stage::Coast);
                    }
                } else if stage == Stage::Coast {
                    if reading.vel_z.abs() < config::APOGEE_TRESHOLD {
                        state.apogee_time = Instant::now();
                        println!("\n\n\n[CONTROL] Apogee detected!");
                        println!("[CONTROL] Time: {:?}", state.launch_time.elapsed());
                        state.set_stage(Stage::Apogee);
                    }
                    if reading.pos_z < max_altitude {
                        state.apogee_time = Instant::now();
                        println!("\n\n\n[CONTROL] Apogee detected!");
                        println!("[CONTROL] Time: {:?}", state.launch_time.elapsed());
                        state.set_stage(Stage::Apogee);
                    }
                } else if stage == Stage::Apogee {
                    if reading.pos_z < config::PARACHUTE_ALTITUDE {
                        state.parachute_time = Instant::now();
                        println!("\n\n\n[CONTROL] Parachute altitude detected!");
                        println!("[CONTROL] Time: {:?}", state.launch_time.elapsed());
                        state.set_stage(Stage::Parachute);
                    }
                } else if stage == Stage::Parachute {
                    if reading.vel_z < config::LANDING_SPEED_TRESHOLD {
                        state.landed_time = Instant::now();
                        println!("\n\n\n[CONTROL] Landed detected!");
                        println!("[CONTROL] Time: {:?}", state.launch_time.elapsed());
                        state.set_stage(Stage::Landed);
                    }
                } else if stage == Stage::Landed {
                } else {
                    println!("[CONTROL] Unknown stage: {:?}", stage);
                }
            // last_reading = reading.clone();
            } else {
                println!("[CONTROL] Error accessing state");
            }
        } else {
            println!("[CONTROL] Error accessing sensors reading");
        }

        if let Ok(flag_continue_running) = flag_continue_running.lock() {
            if !(*flag_continue_running) {
                println!("[CONTROL] Exiting...");
                break Ok("ok".to_string());
            }
        } else {
            println!("[CONTROL] Error checking flag");
        }

        // Time loop
        time_loop(target_loop_duration, loop_start)
    }
}
