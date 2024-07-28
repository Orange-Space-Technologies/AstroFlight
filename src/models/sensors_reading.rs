#[derive(Debug, Clone)]
pub struct SensorsReading {
    // Barometer
    pub pressure: f32,
    pub altitude: f32,
    pub temperature: f32,

    // IMU
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
    pub acc_x: f32,
    pub acc_y: f32,
    pub acc_z: f32,

    // GPS
    pub gps_latitude: f32,
    pub gps_longitude: f32,
    pub gps_altitude: f32,
}

impl SensorsReading {
    pub const fn null() -> SensorsReading{
        return SensorsReading {
            pressure: 0.0,
            altitude: 0.0,
            temperature: 0.0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
            acc_x: 0.0,
            acc_y: 0.0,
            acc_z: 0.0,
            gps_latitude: 0.0,
            gps_longitude: 0.0,
            gps_altitude: 0.0,
        };
    }
}