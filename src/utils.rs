use std::time::{Duration, Instant};

pub fn time_loop(target_loop_duration: Duration, loop_start: Instant) {
    let time_since_loop_start = loop_start.elapsed();
    if time_since_loop_start < target_loop_duration {
        std::thread::sleep(target_loop_duration - time_since_loop_start);
    }
}
