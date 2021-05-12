extern crate either;
use either::*;
use std::slice::*;
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

static ZERO: Point = Point { x: 0.0, y: 0.0 };

pub struct Node {
    mass: &Mass,
    tree: Option<Box<(Node, Node)>>,
}
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}
impl Mass {
    pub fn new_random() -> Mass {
        let m: Mass;
        return m;
    }
}
pub fn step(masses: Vec<Mass>) {
    let i = masses.iter();
    let n = Node::new_leaf(*i.next().unwrap());
    for m in i {
        n.add(m);
    }
    n.updateWith(ZERO);
    return;
}

impl Node {
    pub fn new_leaf(m: Mass) -> Node {
        let n: Node;
        n.mass = m;
        n.tree = None;
        return n;
    }
    pub fn new(l: Mass, r: Mass) -> Node {
        let n: Node;
        n.tree = Some(Box::new((Node::new_leaf(l), Node::new_leaf(r))));
        n.mass.mass = l.mass + r.mass;
        n.mass.position = l
            .position
            .scale(l.mass)
            .add(r.position.scale(r.mass))
            .scale(1.0 / (l.mass + r.mass));
        n.mass.velocity = ZERO;
        return n;
    }

    pub fn add(&self, mass: &Mass) {
        if self.tree.is_none() {
            self.tree = Some(Box::new((Node::new_leaf(mass), Node::new_leaf(self.mass))));
        } else {
            let (left, right) = *self.tree.unwrap();

            let leftDistance2 = left
                .mass
                .position
                .minus(self.mass.position)
                .magnitudeSquared();
            let rightDistance2 = right
                .mass
                .position
                .minus(self.mass.position)
                .magnitudeSquared();
            if leftDistance2 < rightDistance2 {
                left.add(mass);
            } else {
                right.add(mass);
            }
        }
        let (left, right) = *self.tree.unwrap();
        mass.mass = left.mass.mass + right.mass.mass;
        mass.position = left
            .mass
            .position
            .scale(left.mass.mass)
            .add(right.mass.position.scale(right.mass.mass))
            .scale(1.0 / mass.mass);
    }

    fn updateWith(&self, force: Point) {
        if self.tree.is_some() {
            let (left, right) = *self.tree.unwrap();
            let diff = left.mass.position.minus(right.mass.position);
            let f = diff
                .unitVector()
                .scale((left.mass.mass + right.mass.mass) / diff.magnitudeSquared().sqrt());
            left.updateWith(force.add(f));
            right.updateWith(force.add(f.inverse()));
        } else {
            self.mass.velocity.add(force.scale(1.0 / self.mass.mass));
            self.mass.position = self.mass.position.add(self.mass.velocity);
        }
    }
}
