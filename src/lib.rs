/*

The force on this mass is the sum of the forces of adjacent masses and the clouds around that.

To build the tree, we need the center of mass of each cloud.

1. build the tree.
2. calculate the forces on each mass
3. update the velocity and position
4. repeat.

*/
pub mod point;
use point::*;
use rand::Rng;
use std::iter::*;
use Tree::*;

#[derive(Debug, Copy, Clone)]
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

#[derive(Debug)]
pub struct TreeNode {
    center: Point,
    mass: Float,
    left: Tree,
    right: Tree,
}

#[derive(Debug)]
pub enum Tree {
    Node(Box<TreeNode>),
    Leaf(Mass),
}

impl Tree {
    pub fn size(&self) -> i32 {
        match self {
            Leaf(_) => 1,
            Node(n) => n.left.size() + n.right.size(),
        }
    }
    fn new_node(left: Tree, right: Tree) -> Tree {
        let new_mass = left.mass() + right.mass();
        Node(Box::new(TreeNode {
            center: left
                .center()
                .scale(left.mass())
                .add(right.center().scale(right.mass()))
                .scale(1.0 / new_mass),
            mass: new_mass,
            left,
            right,
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

    fn add_mass(self, mass_ref: Mass) -> Self {
        match self {
            Leaf(lm) => Tree::new_node(Leaf(lm), Leaf(mass_ref)),
            Node(node) => {
                let center = mass_ref.position;
                // ignore the effect of node.mass, because it would be same for left and right
                let mut left_force =
                    (node.left.mass()) / node.left.center().minus(center).magnitude_squared();
                let mut right_force =
                    (node.right.mass()) / node.right.center().minus(center).magnitude_squared();
                if left_force.is_nan() {
                    println!("left NaN");
                    left_force = rand::thread_rng().gen::<Float>();
                }
                if right_force.is_nan() {
                    println!("right NaN");
                    right_force = rand::thread_rng().gen::<Float>();
                }
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
                let mut point_mass = mass;
                point_mass.velocity = point_mass.velocity.add(force.scale(1.0 / point_mass.mass));
                point_mass.position = point_mass.position.add(point_mass.velocity);
            }
        }
    }
    pub fn mass_iter(&self) -> TreeIter {
        TreeIter::new(&self)
    }
}

pub struct TreeIter<'a> {
    stack: Vec<&'a Tree>,
}

impl<'a> TreeIter<'a> {
    fn new(tree: &'a Tree) -> TreeIter<'a> {
        let mut i = TreeIter { stack: Vec::new() };
        i.load(tree);
        i
    }
    fn load(&mut self, tree: &'a Tree) {
        self.stack.push(tree);
        match tree {
            Leaf(_) => {}
            Node(n) => {
                self.load(&n.left);
            }
        }
    }
}
impl<'a> Iterator for TreeIter<'a> {
    type Item = &'a Mass;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(Node(n)) => {
                self.load(&n.right);
                self.next()
            }
            Some(Leaf(m)) => Some(m),
            None => None,
        }
    }
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

#[derive(Debug)]
pub struct Simulator {
    pub tree: Tree,
}

impl Simulator {
    pub fn new(count: usize) -> Self {
        let mut tree = Leaf(Mass::new_random());
        for _i in 1..count {
            tree = tree.add_mass(Mass::new_random());
        }
        Simulator { tree }
    }
    pub fn run(&mut self) {
        loop {
            self.tree = self.new_tree();
        }
    }
    pub fn new_tree(&self) -> Tree {
        let mut iter = self.tree.mass_iter();
        let mut tree = Leaf(*iter.next().unwrap());
        for mass in iter {
            tree = tree.add_mass(*mass);
        }
        tree.update_with(Point::ZERO);
        tree
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_with() {
        let test_mass = Mass {
            position: Point::ZERO,
            velocity: Point(1.0, 1.0),
            mass: 1.0,
        };
        let mut test_node = Tree::Leaf(test_mass);

        test_node.update_with(Point::ZERO);

        if let Leaf(ref x) = test_node {
            assert!(x.position.minus(Point(1.0, 1.0)).magnitude_squared() < Point::EPSILON);
            assert!(x.position == Point(1.0, 1.0));
        } else {
            panic!("Not a Leaf() when that is the only choice!!");
        }

        test_node.update_with(Point(2.0, 3.0));

        if let Leaf(ref x) = test_node {
            assert!(x.position.minus(Point(4.0, 5.0)).magnitude_squared() < Point::EPSILON);
            assert!(x.position == Point(4.0, 5.0));
        } else {
            panic!("Not a Leaf() when that is the only choice!!");
        }
    }
}
