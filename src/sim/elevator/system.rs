use super::Params;


pub struct Order {
    destination_floor: usize,
}

#[derive(PartialEq, Debug)]
pub enum ElevatorState {
    Moving(usize), // position
    Loading(usize, usize), // Floor, time left to close door
    Waiting(usize), // Floor
}

// Distance betweeen floors is 10. 
// So ground floor is at pos 0 and 1st floor is at pos 10.
pub struct Elevator {
    pub state: ElevatorState,
    orders: Vec<Order>,
}

pub struct System {
    pub params: Params,
    pub elevator: Elevator,
}


impl Order {
    pub fn new(destination_floor: usize) -> Self {
        Self { destination_floor }
    }
}


impl Elevator {
    fn new() -> Self {
        Self { 
            state: ElevatorState::Waiting(0),
            orders: vec![],
         }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
    
    // Update elevator state
    pub fn  next_step(&mut self, params: &Params) {
        if let ElevatorState::Loading(floor, time_left) = self.state {
            if time_left > 1 {
                self.state = ElevatorState::Loading(floor, time_left - 1);
            } else {
                self.state = ElevatorState::Waiting(floor);
                self.execute_order(params);
            }
        } else {
            self.execute_order(params);
        }
    }
    
    fn execute_order(&mut self, params: &Params) {
        if let Some(order) = self.orders.first() {
            let dest = order.destination_floor * params.floor_dist;
            let pos = match self.state {
                ElevatorState::Moving(pos) => pos,
                ElevatorState::Waiting(floor) => floor * params.floor_dist,
                ElevatorState::Loading(floor, _) => floor * params.floor_dist,
            };

            self.state = if pos > dest { 
                ElevatorState::Moving(pos-1)
            } else if pos < dest { 
                ElevatorState::Moving(pos+1)
            } else { 
                ElevatorState::Loading(order.destination_floor, params.load_time)
            };

            if let ElevatorState::Loading(_, _) = self.state {
                self.orders.remove(0);
            }
        }
    }
}

impl System {
    pub fn new(params: Params) -> Self {
        Self {
            params,
            elevator: Elevator::new(),
        }
    }

    pub fn next_step(&mut self, _ts: i32) {
        // 1. Generate new people at each floor
        // 2. Move lift to the next floor
        self.elevator.next_step(&self.params);
        // 3. People going out and into the lift if the lift stops.
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    // use super::*;

    use crate::sim::elevator::{ElevatorState, Params};

    use super::{Elevator, Order};

    #[test]
    fn test_floor_1() {
        let params = Params::new(8, 10, 20);
        let mut elevator = Elevator::new();
        elevator.add_order(Order::new(8));
        
        for _ in 0..10 {
            elevator.next_step(&params);
        }

        assert_eq!(elevator.state, ElevatorState::Moving(10));
    }

    #[test]
    fn test_unload() {
        let params = Params::new(8, 10, 20);
        let mut elevator = Elevator::new();
        elevator.add_order(Order::new(1));
        
        for _t in 0..11 {
            elevator.next_step(&params);
        }

        assert_eq!(elevator.state, ElevatorState::Loading(1, 20));
    }

    #[test]
    fn test_wait() {
        let params = Params::new(8, 10, 20);
        let mut elevator = Elevator::new();
        elevator.add_order(Order::new(1));
        
        for _t in 0..31 {
            elevator.next_step(&params);
        }

        assert_eq!(elevator.state, ElevatorState::Waiting(1));
    }

    #[test]
    fn test_2_orders() {
        let params = Params::new(8, 2, 1);
        let mut elevator = Elevator::new();
        elevator.add_order(Order::new(1));
        elevator.add_order(Order::new(2));
        
        for _t in 0..4 {
            elevator.next_step(&params);
        }

        assert_eq!(elevator.state, ElevatorState::Moving(3));
    }
}