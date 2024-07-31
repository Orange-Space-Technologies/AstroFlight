// Config file, contains all the constants used in the program

// Threads timing settings
pub(crate) const SENSOR_READING_THREAD_HZ: u8 = 100;
pub(crate) const TELEMETRY_THREAD_HZ: u8 = 10;
pub(crate) const LOGGING_THREAD_HZ: u8 = 100;
pub(crate) const CONTROL_THREAD_HZ: u8 = 100;

// Logging filename, `{}` is replaced with a timestamp
pub(crate) const LOGGING_FILENAME: &str = "data/data_{}.csv";

// Launch check settings
pub(crate) const LAUNCH_TRESHOLD: f32 = 5.0; // m/s^2
pub(crate) const LAUNCH_DETECT_TIME: u128 = 100; // ms

// Burnout check settings
pub(crate) const BURNOUT_TRESHOLD: f32 = 0.0; // m/s^2
pub(crate) const BURNOUT_DETECT_TIME: u128 = 100; // ms

// Apogee check settings
pub(crate) const APOGEE_TRESHOLD: f32 = 2.0; // m/s
pub(crate) const APOGEE_DETECT_TIME: u128 = 100; // ms

// Parachute check settings
pub(crate) const PARACHUTE_ALTITUDE: f32 = 150.0; // m
pub(crate) const PARACHUTE_DETECT_TIME: u128 = 100; // ms

// Landing check settings
pub(crate) const LANDING_SPEED_TRESHOLD: f32 = 3.0; // m/s
pub(crate) const LANDING_DETECT_TIME: u128 = 1000; // ms