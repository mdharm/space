use super::*;

#[derive(Debug)]
pub struct MattFactory;

impl SimFactory for MattFactory {
    fn new(&self, count: usize) -> Box<dyn Simulator> {
        let mut masses: Vec<Mass> = Vec::new();
        let mut cm_numerator = Point::ZERO;
        let mut cm_denominator = 0.0;
        for _i in 0..count {
            let tmp = Mass::new_random();
            cm_numerator += tmp.position * tmp.mass;
            cm_denominator += tmp.mass;
            masses.push(Mass::new_random());
        }
        Box::new(MattSimulator {
            masses,
            cm_numerator,
            cm_denominator,
        })
    }

    fn name(&self) -> String {
        String::from("Matt's Simulator")
    }
}

#[derive(Debug)]
struct MattSimulator {
    masses: Vec<Mass>,
    cm_numerator: Point,
    cm_denominator: Float,
}

impl MattSimulator {}

impl Simulator for MattSimulator {
    fn step(&mut self) {
        for x in self.masses.iter_mut() {
            // update position based on current velocity
            x.position += x.velocity;

            // update velocity based on gravity effect
            // center of mass updated to exclude this particular mass
            let cm = (self.cm_numerator - (x.position * x.mass)) / (self.cm_denominator - x.mass);

            // force felt by this mass = this_mass * other_mass / distance**2
            let force =
                (x.mass * (self.cm_denominator - x.mass)) / (x.position - cm).magnitude_squared();

            // acceleration (change in velocity) is force / mass along the vector between the mass
            // and the center of mass of the cloud
            x.velocity += (cm - x.position) * (force / x.mass);
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
