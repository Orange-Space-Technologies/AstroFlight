use crate::models::stage::Stage;

#[derive(Debug, Clone, Copy)]
pub struct State {
    stage: Stage,
}

impl State {
    pub fn new() -> State {
        State {
            stage: Stage::Init,
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
}