
pub struct Params {
    pub num_floors: usize,
    pub floor_dist: usize,
    pub load_time: usize,
}

impl Params {
    pub fn new(num_floors: usize, floor_dist: usize, load_time: usize) -> Self {
        Self { 
            num_floors,
            floor_dist,
            load_time 
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self { 
            num_floors: 8, 
            floor_dist: 10, 
            load_time: 20 
        }
    }
}