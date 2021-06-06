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
use Tree::*;

static ZERO: Point = Point { x: 0.0, y: 0.0 };

#[derive(Debug)]
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

#[derive(Debug)]
struct TreeNode<'a> {
    center: Point,
    mass: Float,
    left: Tree<'a>,
    right: Tree<'a>,
}

#[derive(Debug)]
pub enum Tree<'a> {
    Node(Box<TreeNode<'a>>),
    Leaf(&'a mut Mass),
}

impl<'a> Tree<'a> {
    pub fn new_node(left: Tree<'a>, right: Tree<'a>) -> Tree<'a> {
        let new_mass = left.mass() + right.mass();
        Node(Box::new(TreeNode {
            center: left
                .center()
                .scale(left.mass())
                .add(right.center().scale(right.mass()))
                .scale(1.0 / new_mass),
            mass: new_mass,
            left: left,
            right: right,
        }))
    }

    fn center(&self) -> Point {
        match self {
            Leaf(m) => m.position,
            Node(n) => n.center,
        }
    }

    fn mass(&self) -> Float {
        match self {
            Leaf(m) => m.mass,
            Node(n) => n.mass,
        }
    }

    pub fn add(self, new_mass: &'a mut Mass) -> Self {
        match self {
            Leaf(mass) => Tree::new_node(Leaf(mass), Leaf(new_mass)),
            Node(node) => {
                let left_force = (node.left.mass() * new_mass.mass)
                    / node
                        .left
                        .center()
                        .minus(new_mass.position)
                        .magnitude_squared();
                let right_force = (node.right.mass() * new_mass.mass)
                    / node
                        .right
                        .center()
                        .minus(new_mass.position)
                        .magnitude_squared();
                // can Boxes be reused?
                if right_force > left_force {
                    Tree::new_node(node.left, node.right.add(new_mass))
                } else {
                    Tree::new_node(node.left.add(new_mass), node.right)
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
                mass.velocity.add(force.scale(1.0 / mass.mass));
                mass.position = mass.position.add(mass.velocity);
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
        let mut tree = self.tree();
        tree.update_with(ZERO);
    }
    pub fn tree<'a>(&'a mut self) -> Tree<'a> {
        let mut iter = self.masses.iter_mut();
        let mut tree = Leaf(iter.next().unwrap());
        for mass in iter {
            tree = tree.add(mass);
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
        let mut test_node = Tree::Leaf(&mut test_mass);

        test_node.update_with(ZERO);
        println!("test_node is {:?}", test_node);
    }
}
