
pub struct Params {
    pub world_width: usize,
    pub world_height: usize,
}

impl Params {
    pub fn new(world_width: usize, world_height: usize) -> Self {
        Self { 
            world_width,
            world_height,
        }
    }
}

impl Default for Params {
    fn default() -> Self {
        Self { 
            world_width: 100,
            world_height: 100,
        }
    }
}