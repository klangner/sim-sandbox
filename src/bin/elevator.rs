use std::fs;

use anyhow::Result;

use sim_sandbox::sim::elevator::{System, Params};


fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: Params = toml::from_str(&contents)?;

    let mut system = System::new(params);

    for ts in 0..100 {
        system.next_step(ts);
        let elevator = &system.elevator;
        println!("Step {}: state {:?}", ts, elevator.state);
    }

    println!("Simulation finished!");
    Ok(())
}
