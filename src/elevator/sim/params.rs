use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Params {
    pub seed: i32,
    pub sim_steps: usize,
    pub num_floors: usize,
    pub floor_dist: usize,
    pub load_time: usize,
}

impl Params {
    pub fn new(num_floors: usize, floor_dist: usize, load_time: usize) -> Self {
        Self { 
            seed: 12345678,
            sim_steps: 100,
            num_floors,
            floor_dist,
            load_time 
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self { 
            seed: 12345678,
            sim_steps: 100,
            num_floors: 8, 
            floor_dist: 10, 
            load_time: 20 
        }
    }
}