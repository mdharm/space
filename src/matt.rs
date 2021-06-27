use super::*;

#[derive(Debug)]
pub struct MattFactory;

impl SimFactory for MattFactory {
    fn new(&self, count: usize) -> Box<dyn Simulator> {
        let mut masses: Vec<Mass> = Vec::new();
        for _i in 0..count {
            masses.push(Mass::new_random());
        }
        Box::new(MattSimulator { masses })
    }

    fn name(&self) -> String {
        String::from("Matt's Simulator")
    }
}

#[derive(Debug)]
struct MattSimulator {
    masses: Vec<Mass>,
}

impl MattSimulator {}

impl Simulator for MattSimulator {
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
    fn test_step() {
        let test_mass = Mass {
            position: Point::ZERO,
            velocity: Point(1.0, 1.0),
            mass: 1.0,
        };
        let mut sim = MattSimulator {
            masses: vec![test_mass],
        };

        sim.step();
        assert!((sim.masses[0].position - Point(1.0, 1.0)).magnitude_squared() < Point::EPSILON);
        assert!(sim.masses[0].position == Point(1.0, 1.0));

        sim.step();
        assert!((sim.masses[0].position - Point(2.0, 2.0)).magnitude_squared() < Point::EPSILON);
        assert!(sim.masses[0].position == Point(2.0, 2.0));

        sim.step();
        assert!((sim.masses[0].position - Point(3.0, 3.0)).magnitude_squared() < Point::EPSILON);
        assert!(sim.masses[0].position == Point(3.0, 3.0));
    }
}
