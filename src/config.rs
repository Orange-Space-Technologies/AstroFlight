use std::any;

pub(crate) const SENSOR_READING_THREAD_HZ: u8 = 100;
pub(crate) const TELEMETRY_THREAD_HZ: u8 = 10;
pub(crate) const LOGGING_THREAD_HZ: u8 = 100;
pub(crate) const CONTROL_THREAD_HZ: u8 = 100;

pub(crate) const LOGGING_FILENAME: &str = "data/data_{}.csv";

pub(crate) const LAUNCH_TRESHOLD: f32 = 5.0; // m/s^2
pub(crate) const BURNOUT_TRESHOLD: f32 = 0.0; // m/s^2
pub(crate) const APOGEE_TRESHOLD: f32 = 2.0; // m/s
pub(crate) const PARACHUTE_ALTITUDE: f32 = 150.0; // m
pub(crate) const LANDING_SPEED_TRESHOLD: f32 = 3.0; // m/s