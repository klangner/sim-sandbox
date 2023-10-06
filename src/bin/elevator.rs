use sim_sandbox::sim::elevator::system::System;



fn main() {
    let mut system = System::new(10);
    system.next_step(1);
    println!("Step 1");
}
