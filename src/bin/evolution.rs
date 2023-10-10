use std::fs;
use anyhow::Result;
use sim_sandbox::evolution::{Params, Universe};


 

fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: Params = toml::from_str(&contents)?;
    let mut universe = Universe::random(params);

    for _ in 0..1000 {
        universe.tick();
    }

    println!("Simulation finisheed!");

    Ok(())
}