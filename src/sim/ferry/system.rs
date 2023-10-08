use super::Params;


pub struct Cargo {
    pub id: usize,
    pub source_port: usize,
    pub dest_port: usize,
}

pub struct Port {
    pub id: usize,
    pub name: String,
    pub cargos: Vec<usize>, // Cargo waiting for transport
}

pub struct Ship {
    pub id: usize,
    pub line: Vec<usize>, // List of ports id
    pub cargos: Vec<usize>, // Cargo in transport
}

// Route has direction
pub struct Route {
    pub source_port: usize,
    pub dest_port: usize,
    pub distance: usize,
}

// Probably on the water we can connect each ports directly
pub struct Map {
    pub ports: Vec<Port>,
    pub routes: Vec<Route>,
}

pub struct System {
    pub params: Params,
    pub map: Map,
    pub ships: Vec<Ship>,
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_map() {
        // 2 ports with 2 routes
    }
}