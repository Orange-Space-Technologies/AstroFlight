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

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let buff = [
            &self.pressure.to_be_bytes()[..],
            &self.altitude.to_be_bytes()[..],
            &self.temperature.to_be_bytes()[..],
            &self.pos_x.to_be_bytes()[..],
            &self.pos_y.to_be_bytes()[..],
            &self.pos_z.to_be_bytes()[..],
            &self.vel_x.to_be_bytes()[..],
            &self.vel_y.to_be_bytes()[..],
            &self.vel_z.to_be_bytes()[..],
            &self.acc_x.to_be_bytes()[..],
            &self.acc_y.to_be_bytes()[..],
            &self.acc_z.to_be_bytes()[..],
            &self.gps_latitude.to_be_bytes()[..],
            &self.gps_longitude.to_be_bytes()[..],
            &self.gps_altitude.to_be_bytes()[..],
        ].concat();
        return buff
    }

    pub fn to_csv(&self) -> String {
        return format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.pressure,
            self.altitude,
            self.temperature,
            self.pos_x,
            self.pos_y,
            self.pos_z,
            self.vel_x,
            self.vel_y,
            self.vel_z,
            self.acc_x,
            self.acc_y,
            self.acc_z,
            self.gps_latitude,
            self.gps_longitude,
            self.gps_altitude
        );
    }
}