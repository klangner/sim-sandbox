use sim_sandbox::sim::elevator::{System, Params};


fn main() {
    let params = Params::default();
    let mut system = System::new(params);

    for ts in 0..100 {
        system.next_step(ts);
        let elevator = &system.elevator;
        // println!("Step {}: Floor {}", ts, elevator.current_pos);
    }

    println!("Simulation finished!");
}
