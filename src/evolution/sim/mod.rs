/* 

# Elevetor sim spec

* There are fixed number of floors
* 1 step = 1s
* distance between floors is 10 steps
* Loading an unloading takes 20 steps in total
* Elevator can be in the following states
  * Moving between floors
  * Open at given floor (loading/unloading)
  * Waiting for orders at given floor
* Lift has list of orders to follow
* Order consist of destination (floors)

*/ 


pub mod universe;
pub mod params;

pub use universe::*;
pub use params::*;