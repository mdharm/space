use rand::Rng;

pub type Float = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

impl Point {
    pub fn scale(self, s: Float) -> Point {
        Point {
            x: self.x * s,
            y: self.y * s,
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, that: Point) -> Point {
        Point {
            x: self.x + that.x,
            y: self.y + that.y,
        }
    }
    pub fn minus(self, that: Point) -> Point {
        Point {
            x: self.x - that.x,
            y: self.y - that.y,
        }
    }
    pub fn inverse(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn magnitude_squared(self) -> Float {
        self.x * self.x + self.y * self.y
    }
    pub fn magnitude(self) -> Float {
        self.magnitude_squared().sqrt()
    }
    pub fn unit_vector(self) -> Point {
        self.scale(1.0 / self.magnitude())
    }
    pub fn new_random() -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen::<Float>() - 0.5,
            y: rng.gen::<Float>() - 0.5,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!();
            }
        };
    }
    macro_rules! assert_def {
        ($x:expr, $y:expr) => {
            assert_delta!($x, $y, 0.001)
        };
    }
    #[test]
    fn test_add() {
        let one = Point { x: 1.0, y: 1.0 };
        let zero = Point { x: 0.0, y: 0.0 };
        assert_eq!(one, one.add(zero));
        assert_eq!(one, one.add(one).minus(one));
    }
    #[test]
    fn test_magnitude() {
        let one = Point { x: 1.0, y: 1.0 };
        assert_def!(2.0, one.magnitude_squared());
        assert_def!(2.0, one.magnitude() * one.magnitude());
    }
    #[test]
    fn test_unitvector() {
        let one = Point { x: 2.0, y: 2.0 };
        assert_def!((8.0 as f64).sqrt(), one.unit_vector().magnitude());
        assert_def!(one.unit_vector().x, one.unit_vector().y)
    }
}
