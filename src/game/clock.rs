extern crate wasm_bindgen;

use super::wasm_timer::SystemTime;

pub struct Clock {
    ms_per_frame: u128,
    last_time: SystemTime
}

impl Clock {

    pub fn new(ms_per_frame:u128) -> Clock {
        let last_time = SystemTime::now();
        Clock {
            ms_per_frame, last_time
        }
    }
    
    pub fn is_waiting(&self) -> bool {
        return SystemTime::now().duration_since(self.last_time).unwrap().as_millis()
            < self.ms_per_frame;
    }

    pub fn wait(&mut self) {
        while self.is_waiting() {}
        self.last_time = SystemTime::now();
    }
}
