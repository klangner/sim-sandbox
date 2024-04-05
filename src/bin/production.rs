use anyhow::Result;


const SIM_TIME_SEC: u32 = 100;


struct Resource {
    id: u32,
    name: String,
    amount: u32,
}


impl Resource {
    fn new(id: u32, name: String, amount: u32) -> Self {
        Self { id, name, amount }
    }
}

fn main() -> Result<()> {
    let iron = Resource::new( 1, "iron".to_owned(), 1_000);

    for ts in 0..SIM_TIME_SEC {
    }

    println!("World state after {} seconds!", SIM_TIME_SEC);
    println!("Resources");
    println!(" * Resource id: {}, Ore type: {}, amount left: {}", iron.id, iron.name, iron.amount);

    Ok(())
}
