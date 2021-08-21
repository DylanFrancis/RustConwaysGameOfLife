
#[derive(Default)]
pub struct Timer {
    pub time: u128,
    pub updated: bool
}

impl Timer {
    pub fn reset(&mut self) {
        self.time = 0;
    }

    pub fn add(&mut self, t: u128) {
        self.time += t;
    }
}