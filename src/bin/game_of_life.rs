use std::fs;

use anyhow::Result;

use sim_sandbox::sim::gol::{System, Params};


fn main() -> Result<()> {
    let param_file = std::env::args().last().expect("No param file provided");
    let contents = fs::read_to_string(&param_file)?;
    let params: Params = toml::from_str(&contents)?;

    let mut system = System::new(params);

    for _ts in 0..system.params.num_steps {
        system.next_step();
    }

    println!("Simulation finished!");
    Ok(())
}
