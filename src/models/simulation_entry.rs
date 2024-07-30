use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SimulationEntry {
    pub time: f64, // Time (s)
    pub altitude: f64, // Altitude (m)
    pub vertical_velocity: f64, // Vertical velocity (m/s)
    pub vertical_acceleration: f64, // Vertical acceleration (m/s^2)
}

impl SimulationEntry {
    pub const fn null() -> SimulationEntry {
        return SimulationEntry {
            time: 0.0,
            altitude: 0.0,
            vertical_velocity: 0.0,
            vertical_acceleration: 0.0,
        };
    }
}