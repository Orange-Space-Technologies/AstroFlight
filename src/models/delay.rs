extern crate embedded_hal;
use embedded_hal::delay::DelayNs;

pub struct Delay {

}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(std::time::Duration::from_nanos(ns as u64));
    }
    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
    fn delay_us(&mut self, us: u32) {
        std::thread::sleep(std::time::Duration::from_micros(us as u64));
    }
}

impl Delay {
    pub fn new() -> Delay {
        Delay {}
    }
}