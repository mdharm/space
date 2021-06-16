/*

The force on this mass is the sum of the forces of adjacent masses and the clouds around that.

To build the tree, we need the center of mass of each cloud.

1. build the tree.
2. calculate the forces on each mass
3. update the velocity and position
4. repeat.

*/
pub mod point;
use point::{Float, Point};
use rand::Rng;
use std::cell::RefCell;
use Tree::*;

static ZERO: Point = Point { x: 0.0, y: 0.0 };

#[derive(Debug)]
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    center: Point,
    mass: Float,
    left: Tree<'a>,
    right: Tree<'a>,
}

#[derive(Debug)]
pub enum Tree<'a> {
    Node(Box<TreeNode<'a>>),
    Leaf(RefCell<&'a mut Mass>),
}

impl<'a> Tree<'a> {
    fn new_node(left_tree: Tree<'a>, right_tree: Tree<'a>) -> Tree<'a> {
        let new_mass = left_tree.mass() + right_tree.mass();
        Node(Box::new(TreeNode {
            center: left_tree
                .center()
                .scale(left_tree.mass())
                .add(right_tree.center().scale(right_tree.mass()))
                .scale(1.0 / new_mass),
            mass: new_mass,
            left: left_tree,
            right: right_tree,
        }))
    }

    fn center(&self) -> Point {
        match self {
            Leaf(m) => m.borrow().position,
            Node(n) => n.center,
        }
    }

    fn mass(&self) -> Float {
        match self {
            Leaf(m) => m.borrow().mass,
            Node(n) => n.mass,
        }
    }

    fn add_mass(self, mass_ref: RefCell<&'a mut Mass>) -> Self {
        match self {
            Leaf(mass) => Tree::new_node(Leaf(mass), Leaf(mass_ref)),
            Node(node) => {
                let center = mass_ref.borrow().position;
                // ignore the effect of node.mass, because it would be same for left and right
                let left_force =
                    (node.left.mass()) / node.left.center().minus(center).magnitude_squared();
                let right_force =
                    (node.right.mass()) / node.right.center().minus(center).magnitude_squared();

                if right_force > left_force {
                    Tree::new_node(node.left, node.right.add_mass(mass_ref))
                } else {
                    Tree::new_node(node.left.add_mass(mass_ref), node.right)
                }
            }
        }
    }

    fn update_with(&mut self, force: Point) {
        match self {
            Node(i) => {
                let diff = i.left.center().minus(i.right.center());
                let f = diff
                    .unit_vector()
                    .scale((i.left.mass() * i.right.mass()) / diff.magnitude_squared().sqrt());
                i.left.update_with(force.add(f));
                i.right.update_with(force.add(f.inverse()));
            }
            Leaf(mass) => {
                let mut point_mass = mass.borrow_mut();
                point_mass.velocity = point_mass.velocity.add(force.scale(1.0 / point_mass.mass));
                point_mass.position = point_mass.position.add(point_mass.velocity);
            }
        }
    }
}

impl Mass {
    pub fn new_random() -> Mass {
        let mut rng = rand::thread_rng();
        Mass {
            position: Point::new_random(),
            velocity: Point::new_random(),
            mass: rng.gen::<Float>(),
        }
    }
}

#[derive(Debug)]
pub struct Simulator {
    pub masses: Vec<Mass>,
}

impl Simulator {
    pub fn new(count: usize) -> Self {
        let mut simulator = Simulator {
            masses: Vec::with_capacity(count),
        };
        for _i in 1..count {
            simulator.masses.push(Mass::new_random());
        }
        simulator
    }
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.tree().update_with(ZERO);
    }
    pub fn tree(&mut self) -> Tree {
        let mut iter = self.masses.iter_mut().map(RefCell::new);
        let mut tree = Leaf(iter.next().unwrap());
        for mass in iter {
            tree = tree.add_mass(mass);
        }
        tree
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_with() {
        let mut test_mass = Mass {
            position: ZERO,
            velocity: Point { x: 1.0, y: 1.0 },
            mass: 1.0,
        };
        let mut test_node = Tree::Leaf(RefCell::new(&mut test_mass));

        test_node.update_with(ZERO);
        println!("test_node is {:?}", test_node);
    }
}
