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
use Tree::*;

static ZERO: Point = Point { x: 0.0, y: 0.0 };

pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

struct TreeNode<'a> {
    center: Point,
    mass: Float,
    left: Tree<'a>,
    right: Tree<'a>,
}

enum Tree<'a> {
    Node(Box<TreeNode<'a>>),
    Leaf(&'a mut Mass),
}

impl Mass {
    pub fn new_random() -> Mass {
        Mass {
            position: Point::new_random(),
            velocity: Point::new_random(),
            mass: 1.0,
        }
    }
}

pub fn step(masses: &mut Vec<Mass>) {
    let mut i = masses.iter_mut();
    let tree = Tree::new_leaf(i.next().unwrap());
    for m in i {
        tree.add(m);
    }
    tree.updateWith(ZERO);
}

impl<'a> Tree<'a> {
    pub fn new_leaf(m: &'a mut Mass) -> Tree<'a> {
        Leaf(m)
    }
    pub fn new_node(l: &'a mut Mass, r: &'a mut Mass) -> Tree<'a> {
        Node(Box::new(TreeNode {
            center: l
                .position
                .scale(l.mass)
                .add(r.position.scale(r.mass))
                .scale(1.0 / (l.mass + r.mass)),
            mass: l.mass + r.mass,
            left: Leaf(l),
            right: Leaf(r),
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

    pub fn add(mut self, mass: &'a mut Mass) -> Self {
        match self {
            Leaf(mut m) => Tree::new_node(m, mass),
            Node(mut n) => {
                let leftForce = (n.left.mass() + mass.mass)
                    / n.left.center().minus(mass.position).magnitudeSquared();
                let rightForce = (n.right.mass() + mass.mass)
                    / n.right.center().minus(mass.position).magnitudeSquared();
                // can Boxes be reused?
                if rightForce > leftForce {
                    n.right = n.right.add(mass);
                } else {
                    n.left = n.left.add(mass);
                }
                self
            }
        }
    }

    fn updateWith(&self, force: Point) {
        match self {
            Node(i) => {
                let diff = i.left.center().minus(i.right.center());
                let f = diff
                    .unitVector()
                    .scale((i.left.mass() + i.right.mass()) / diff.magnitudeSquared().sqrt());
                i.left.updateWith(force.add(f));
                i.right.updateWith(force.add(f.inverse()));
            }
            Leaf(mut mass) => {
                mass.velocity.add(force.scale(1.0 / mass.mass));
                mass.position = mass.position.add(mass.velocity);
            }
        }
    }
}
