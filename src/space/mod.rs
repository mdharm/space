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

struct Leaf<'a> {
    mass: &'a Mass,
}

struct Node<'a> {
    center: Point,
    mass: Float,
    left: Box<Tree<'a>>,
    right: Box<Tree<'a>>,
}

struct Tree<'a> {
    node: Option<Box<Node<'a>>>,
    leaf: Option<Leaf<'a>>,
}

pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}

impl Mass {
    pub fn new_random() -> Mass {
        return Mass {
            position: Point::new_random(),
            velocity: Point::new_random(),
            mass: 1.0,
        };
    }
}
pub fn step(masses: Vec<Mass>) {
    let mut i = masses.iter();
    let n = Node::new_leaf(i.next().unwrap());
    for m in i {
        n.add(m);
    }
    n.updateWith(ZERO);
    return;
}

impl<'a> Node<'a> {
    pub fn new_leaf(m: &'a Mass) -> Node<'a> {
        return Node {
            mass: m,
            tree: None,
        };
    }
    pub fn new(l: &'a Mass, r: &'a Mass) -> Node<'a> {
        let m: &Mass = Mass {
            position: l
                .position
                .scale(l.mass)
                .add(r.position.scale(r.mass))
                .scale(1.0 / (l.mass + r.mass)),
            velocity: ZERO,
            mass: l.mass + r.mass,
        };
        return Node {
            mass: m,
            tree: Some(Box::new((Node::new_leaf(l), Node::new_leaf(r)))),
        };
    }

    pub fn add(&self, mass: &'a Mass) {
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
