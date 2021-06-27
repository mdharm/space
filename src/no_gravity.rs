use super::*;

#[derive(Debug)]
pub struct NoGravityFactory;

impl SimFactory for NoGravityFactory {
    fn new(&self, count: usize) -> Box<dyn Simulator> {
        let mut masses: Vec<Mass> = Vec::new();
        for _i in 0..count {
            masses.push(Mass::new_random());
        }
        Box::new(NoGravitySimulator { masses })
    }

    fn name(&self) -> String {
        String::from("No Gravity Simulator")
    }
}

#[derive(Debug)]
struct NoGravitySimulator {
    masses: Vec<Mass>,
}

impl NoGravitySimulator {}

impl Simulator for NoGravitySimulator {
    fn step(&mut self) {
        for x in self.masses.iter_mut() {
            x.position += x.velocity;
        }
    }

    fn mass_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Mass> + 'a> {
        Box::new(self.masses.iter())
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
