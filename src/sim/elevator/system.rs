
pub struct System {
    current_floor: usize,
    num_floors: usize,
}


impl System {
    pub fn new(num_floors: usize) -> Self {
        Self {
            num_floors,
            current_floor: 0,
        }
    }

    // Elevator is going only up or from the last floor directly to the ground floor
    pub fn next_step(&mut self, _ts: i32) {
        self.current_floor = (self.current_floor + 1) % self.num_floors;
    }
}