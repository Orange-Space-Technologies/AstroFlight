use crate::models::sensors_reading::SensorsReading;
use crate::models::state::State;
use crate::config;

use std::time::Instant;

pub struct ControlChecks {
    launch_check: Option<Instant>,
    burnout_check: Option<Instant>,
    apogee_check_accel: Option<Instant>,
    apogee_check_altitude: Option<Instant>,
    parachute_check: Option<Instant>,
    landed_check: Option<Instant>,

    last_altitude: f32,
}

// Checks that also check that the statement is true for a certain amount of times
impl ControlChecks {

    pub fn new() -> ControlChecks {
        ControlChecks {
            launch_check: None,
            burnout_check: None,
            apogee_check_accel: None,
            apogee_check_altitude: None,
            parachute_check: None,
            landed_check: None,
            last_altitude: 0.0,
        }
    }

    pub fn check_launch(&mut self, reading: &SensorsReading, state: &State) -> bool {
        if reading.acc_z > config::LAUNCH_TRESHOLD {
            if self.launch_check.is_none() {
                self.launch_check = Some(Instant::now());
                return false;
            }

            if self.launch_check.unwrap().elapsed().as_millis() > config::LAUNCH_DETECT_TIME {
                return true;
            }
            return false;        
        } else {
            self.launch_check = None;
            return false;
        }
    }

    pub fn check_burnout(&mut self, reading: &SensorsReading, state: &State) -> bool {
        if reading.acc_z < config::BURNOUT_TRESHOLD {
            if self.burnout_check.is_none() {
                self.burnout_check = Some(Instant::now());
                return false;
            }

            if self.burnout_check.unwrap().elapsed().as_millis() > config::BURNOUT_DETECT_TIME {
                return true;
            }
            return false;        
        } else {
            self.burnout_check = None;
            return false;
        }
    }

    pub fn check_apogee_accel(&mut self, reading: &SensorsReading, state: &State) -> bool {
        if reading.vel_z.abs() < config::APOGEE_TRESHOLD {
            if self.apogee_check_accel.is_none() {
                self.apogee_check_accel = Some(Instant::now());
                return false;
            }

            if self.apogee_check_accel.unwrap().elapsed().as_millis() > config::APOGEE_DETECT_TIME {
                return true;
            }
            return false;        
        } else {
            self.apogee_check_accel = None;
            return false;
        }
    }

    pub fn check_apogee_altitude(&mut self, reading: &SensorsReading, state: &State) -> bool {
        // if reading.pos_z < max_altitude {
        //     if self.apogee_check.is_none() {
        //         self.apogee_check = Some(Instant::now());
        //         return false;
        //     }

        //     if self.apogee_check.unwrap().elapsed().as_millis() > config::APOGEE_DETECT_TIME {
        //         return true;
        //     }
        //     return false;        
        // } else {
        //     self.apogee_check = None;
        //     return false;
        // }
        if self.apogee_check_altitude.is_none() {
            self.apogee_check_altitude = Some(Instant::now());
            self.last_altitude = reading.pos_z;
            return false;
        }
        if self.apogee_check_altitude.unwrap().elapsed().as_millis() > config::APOGEE_DETECT_TIME {
            self.apogee_check_altitude = Some(Instant::now());
            println!("\n\n[CHECKS] time period elapsed");
            if reading.pos_z < self.last_altitude {
                println!("\n\n[CHECKS] Apogee detected!");
                return true;
            }
            self.last_altitude = reading.pos_z;
            return false;
        }
        return false;
    }

    pub fn check_parachute(&mut self, reading: &SensorsReading, state: &State) -> bool {
        if reading.pos_z < config::PARACHUTE_ALTITUDE {
            if self.parachute_check.is_none() {
                self.parachute_check = Some(Instant::now());
                return false;
            }

            if self.parachute_check.unwrap().elapsed().as_millis() > config::PARACHUTE_DETECT_TIME {
                return true;
            }
            return false;        
        } else {
            self.parachute_check = None;
            return false;
        }
    }

    pub fn check_landed(&mut self, reading: &SensorsReading, state: &State) -> bool {
        if reading.vel_z < config::LANDING_SPEED_TRESHOLD {
            if self.landed_check.is_none() {
                self.landed_check = Some(Instant::now());
                return false;
            }

            if self.landed_check.unwrap().elapsed().as_millis() > config::LANDING_DETECT_TIME {
                return true;
            }
            return false;        
        } else {
            self.landed_check = None;
            return false;
        }
    }
}