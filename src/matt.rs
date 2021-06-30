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
            masses.push(tmp);
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
    fn test_velocity() {
        let test_mass = Mass {
            position: Point::ZERO,
            velocity: Point(1.0, 1.0),
            mass: 1.0,
        };
        let mut sim = MattSimulator {
            masses: vec![test_mass],
            cm_numerator: test_mass.position * test_mass.mass,
            cm_denominator: test_mass.mass,
        };

        sim.step();
        assert!((sim.masses[0].position - Point(1.0, 1.0)).magnitude_squared() < Point::EPSILON);
        assert!(sim.masses[0].position == Point(1.0, 1.0));
    }

    #[test]
    fn test_gravity_vector() {
        // two masses, on either side of the coordinate origin
        let test_mass1 = Mass {
            position: Point(-1.0, 0.0),
            velocity: Point::ZERO,
            mass: 1.0,
        };
        let test_mass2 = Mass {
            position: Point(1.0, 0.0),
            ..test_mass1
        };
        let mut sim = MattSimulator {
            masses: vec![test_mass1, test_mass2],
            cm_numerator: (test_mass1.position * test_mass1.mass)
                + (test_mass2.position * test_mass2.mass),
            cm_denominator: test_mass1.mass + test_mass2.mass,
        };

        sim.step();

        // masses should be moving towards each other
        assert!(sim.masses[0].velocity.0 > 0.0);
        assert!(sim.masses[0].velocity.1 == 0.0);
        assert!(sim.masses[1].velocity.0 < 0.0);
        assert!(sim.masses[1].velocity.1 == 0.0);

        // travel should be along the axis
        assert!(sim.masses[0].position.1 == 0.0);
        assert!(sim.masses[1].position.1 == 0.0);

        sim.step();

        // travel should be along the axis
        assert!(sim.masses[0].position.1 == 0.0);
        assert!(sim.masses[1].position.1 == 0.0);
    }
}
