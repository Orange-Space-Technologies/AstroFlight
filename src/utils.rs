use std::time::Instant;

pub fn time_loop(target_hz: u8, f: &dyn Fn() -> ()) {
    let target_loop_duration = std::time::Duration::from_secs_f32(1.0 / target_hz as f32);
    loop {
        let loop_start = Instant::now();
        f();
        let time_since_loop_start = loop_start.elapsed();
        if time_since_loop_start < target_loop_duration {
            std::thread::sleep(target_loop_duration - time_since_loop_start);
        }
    }
}
