use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Params {
    pub num_steps: usize,
    pub map: Map,
}


impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}


impl Params {
    pub fn new(map: Map) -> Self {
        Self { 
            num_steps: 100,
            map,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self { 
            num_steps: 100,
            map: Map::new(100, 100),
        }
    }
}

