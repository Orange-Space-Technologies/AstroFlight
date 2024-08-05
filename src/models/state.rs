use std::time::Instant;

use crate::models::stage::Stage;

#[derive(Debug, Clone, Copy)]
pub struct State {
    stage: Stage,
    pub launch_time: Instant,
    pub burnout_time: Instant,
    pub apogee_time: Instant,
    pub parachute_time: Instant,
    pub landed_time: Instant,
}

impl State {
    pub fn new() -> State {
        State {
            stage: Stage::Init,
            launch_time: Instant::now(),
            burnout_time: Instant::now(),
            apogee_time: Instant::now(),
            parachute_time: Instant::now(),
            landed_time: Instant::now(),
        }
    }
    pub fn get_stage(&self) -> Stage {
        self.stage
    }
    pub fn set_stage(&mut self, stage: Stage) {
        if stage > self.stage {
            println!("[STATE] Stage changed from {:?} to {:?}", self.stage, stage);
            self.stage = stage;
        }
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{}", //,{:?},{:?},{:?},{:?},{:?}",
            self.launch_time.elapsed().as_millis(),
            // self.burnout_time.elapsed(),
            // self.apogee_time.elapsed(),
            // self.parachute_time.elapsed(),
            // self.landed_time.elapsed(),
            self.stage as u8
        )
    }
}