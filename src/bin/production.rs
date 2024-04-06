use std::fmt;

use anyhow::Result;


#[derive(Clone, Copy, Debug)]
enum Resource {
    Copper,
    Iron,
    Limestone,

    // IronIngot,
    // IronPlate,
    // IronRod,
}

struct ResourceNode {
    resource: Resource,
    amount_available: u32,
}

struct Miner {
    internal_storage: u32,
    resource_node_id: usize,
    output_belt_id: Option<usize>,
}

struct StorageContainer {
    capacity: usize,
    resources: Vec<Resource>,
    input_belt_id: Option<usize>,
}

struct Belt {
    length: usize,
    resources: Vec<Option<Resource>>,
}

// struct Recipie { 
//     input_resources: Vec<Resource>,
//     output_resource: Resource,
// }

// struct Smelter {

// }

// struct Constructor {

// }

// struct Assembler {

// }

struct World {
    resources: Vec<ResourceNode>,
    miners: Vec<Miner>,
    containers: Vec<StorageContainer>,
    belts: Vec<Belt>,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResourceNode {
    pub fn new(resource: Resource) -> Self {
        Self { resource, amount_available: 1_000_000 }
    }
}


impl Miner {
    const STORAGE_CAPACITY: u32 = 100;

    pub fn new(resource_node_id: usize) -> Self {
        Self { internal_storage: 0, resource_node_id, output_belt_id: None }
    }
}


impl StorageContainer {
    const STORAGE_CAPACITY: usize =  7;//5_000;

    pub fn new() -> Self {
        Self { capacity: Self::STORAGE_CAPACITY, resources: vec![], input_belt_id: None }
    }

    pub fn has_space(&self) -> bool {
        self.resources.len() < Self::STORAGE_CAPACITY
    }

    pub fn insert(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}


impl Belt {
    pub fn new(length: usize) -> Self {
        assert!(length > 0);
        Self { 
            length, 
            resources: (0..length).map(|_| None).collect(),
        }
    }

    // Try to add item to the belt. If there is no place then return false. 
    // Otherwise add item
    pub fn try_push(&mut self, resource: Resource) -> bool {
        if self.resources[0].is_none() {
            self.resources[0] = Some(resource);
            true
        } else {
            false
        }
    }

    pub fn fetch(&mut self) -> Option<Resource> {
        let res = self.resources.pop().flatten();
        self.resources.push(None);
        res
    }

    // Update internall state
    pub fn update(&mut self) {
        if self.resources[self.resources.len()-1].is_none() {
            self.resources.pop();
            self.resources.insert(0, None);
        }
    }
}


impl World {
    pub fn new() -> Self {
        Self { 
            resources: vec![],
            miners: vec![], 
            containers: vec![],
            belts: vec![],
        }
    }

    // Create a new resource node and return its id
    pub fn spawn_resource(&mut self, resource: Resource) -> usize {
        self.resources.push(ResourceNode::new(resource));
        self.resources.len() - 1
    }

    // Create new miner
    pub fn spawn_miner(&mut self, resource_id: usize) -> usize {
        self.miners.push(Miner::new(resource_id));
        self.miners.len() - 1
    }

    pub fn spawn_container(&mut self) -> usize {
        self.containers.push(StorageContainer::new());
        self.containers.len() - 1
    }

    // Connect with belt. Return belt id
    pub fn connect(&mut self, miner_id: usize, container_id: usize, length: usize) -> usize {
        self.belts.push(Belt::new(length));
        let belt_id = self.belts.len() - 1;
        if miner_id < self.miners.len() {
            self.miners[miner_id].output_belt_id = Some(belt_id);
        }
        if container_id < self.containers.len() {
            self.containers[container_id].input_belt_id = Some(belt_id);
        }

        belt_id
    }

    pub fn update(&mut self, _ts: u32) {
        // Update all miners and resources
        for miner in self.miners.iter_mut() {
            if let Some(res) = self.resources.get_mut(miner.resource_node_id) {
                // get resource
                if miner.internal_storage < Miner::STORAGE_CAPACITY  && res.amount_available > 0 {
                    res.amount_available -= 1;
                    miner.internal_storage += 1;
                }
                // If connected then put it on the belt
                if let Some(belt_id) = miner.output_belt_id {
                    let belt = &mut self.belts[belt_id];
                    if belt.try_push(res.resource) {
                        miner.internal_storage -= 1;
                    }
                }
            }
        }

        // Update storage containers
        for sc in self.containers.iter_mut().filter(|c| c.has_space()) {
            if let Some(belt_id) = sc.input_belt_id {
                if let Some(res) = self.belts[belt_id].fetch() {
                    sc.insert(res)
                }
            }
        }

        // Update belts
        for belt in self.belts.iter_mut() {
            belt.update();
        }
    }
}


trait Collector {
    fn save_state(&self, ts: u32, world: &World);
}

struct ConsoleCollector {}

impl ConsoleCollector {
    pub fn new() -> Self {
        Self {}
    }
}


impl Collector for ConsoleCollector {
    fn save_state(&self, ts: u32, world: &World) {
        println!("World state after {} seconds!", ts);
        
        println!("Resources:");
        for (i, res) in world.resources.iter().enumerate() {
            println!(" * Id: {}, Ore: {}, amount_left: {} resources", i, res.resource, res.amount_available);
        }
        
        println!("Miners:");
        for (i, miner) in world.miners.iter().enumerate() {
            let res = &world.resources[miner.resource_node_id].resource;
            println!(" * Id: {}, Ore: {}, internal_storage: {} resources", i, res, miner.internal_storage);
        }
        
        println!("Containers:");
        for (i, sc) in world.containers.iter().enumerate() {
            println!(" * Id: {}, amount: {}", i, sc.resources.len());
        }
        
        println!("Belts:");
        for (i, belt) in world.belts.iter().enumerate() {
            println!(" * Id: {}, contains: {:?}", i, belt.resources);
        }
    }
}

fn init_demo_world() -> World {
    let mut world = World::new();
    let r1 = world.spawn_resource(Resource::Iron);
    let r2 = world.spawn_resource(Resource::Copper);
    world.spawn_resource(Resource::Iron);
    world.spawn_resource(Resource::Limestone);

    let m1 = world.spawn_miner(r1);
    world.spawn_miner(r2);
    
    let c1 = world.spawn_container();

    world.connect(m1, c1, 5);

    world
}


fn main() -> Result<()> {
    let mut world =  init_demo_world();
    let collector = ConsoleCollector::new();

    let sim_len_seconds = 150;
    for ts in 0..sim_len_seconds {
        world.update(ts);
    }

    collector.save_state( sim_len_seconds, &world);

    Ok(())
}
