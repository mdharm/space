/*

The force on this mass is the sum of the forces of adjacent masses and the clouds around that.

To build the tree, we need the center of mass of each cloud.

1. build the tree.
2. calculate the forces on each mass
3. update the velocity and position
4. repeat.

*/
pub mod joe;
pub mod matt;
pub mod no_gravity;
pub mod point;
use point::*;
use rand::Rng;
use std::fmt::*;
use std::iter::*;

#[derive(Debug, Copy, Clone)]
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

impl Mass {
    pub fn new_random() -> Mass {
        Mass {
            position: Point::new_random(),
            velocity: Point::new_random(),
            mass: rand::thread_rng().gen::<Float>() / 1000.0,
        }
    }
}

pub trait SimFactory {
    fn new(&self, count: usize) -> Box<dyn Simulator>;
    fn name(&self) -> String;
}

pub trait Simulator: Debug + Send + Sync {
    fn step(&mut self);
    fn mass_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Mass> + 'a>;
}
