use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Params {
    pub width: usize,
    pub height: usize,
    pub population: usize,
    pub steps_per_generation: usize,
}

impl Params {
}

impl Default for Params {
    fn default() -> Self {
        Self { 
            width: 100,
            height: 100,
            population: 1000,
            steps_per_generation: 200,
        }
    }
}