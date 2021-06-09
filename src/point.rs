use rand::Rng;

pub type Float = f64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(pub Float, pub Float);

impl Point {
    pub const ZERO: Point = Point(0.0, 0.0);
    pub const EPSILON: Float = 0.0000001;

    pub fn scale(self, s: Float) -> Point {
        Point(self.0 * s, self.1 * s)
    }

    pub fn add(self, that: Point) -> Point {
        Point(self.0 + that.0, self.1 + that.1)
    }

    pub fn minus(self, that: Point) -> Point {
        self.add(that.inverse())
    }

    pub fn inverse(self) -> Point {
        self.scale(-1.0)
    }

    pub fn magnitude_squared(self) -> Float {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn magnitude(self) -> Float {
        self.magnitude_squared().sqrt()
    }

    pub fn unit_vector(self) -> Point {
        self.scale(1.0 / self.magnitude_squared().sqrt())
    }

    pub fn new_random() -> Point {
        let mut rng = rand::thread_rng();
        Point(rng.gen::<f64>(), rng.gen::<f64>())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale() {
        assert!(Point(1.0, 1.0).scale(2.0) == Point(2.0, 2.0));
        assert!(Point(1.0, 1.0).scale(-3.0) == Point(-3.0, -3.0));
        assert!(Point(1.0, 2.0).scale(-3.0) == Point(-3.0, -6.0));
        assert!(Point(1.0, 2.0).scale(0.5) == Point(0.5, 1.0));
    }

    #[test]
    fn test_add() {
        assert!(Point(4.0, 5.0).add(Point(1.0, 2.0)) == Point(5.0, 7.0));
    }

    #[test]
    fn test_minus() {
        assert!(Point(4.0, 5.0).minus(Point(1.0, 2.0)) == Point(3.0, 3.0));
    }

    #[test]
    fn test_inverse() {
        assert!(Point(1.0, 2.0).inverse() == Point(-1.0, -2.0));
    }

    #[test]
    fn test_magnitude_squared() {
        assert!(Point(3.0, 4.0).magnitude_squared() == 25.0);
    }

    #[test]
    fn test_magnitude() {
        assert!(Point(3.0, 4.0).magnitude() == 5.0);
    }

    #[test]
    fn test_unit_vector() {
        assert!((Point::new_random().unit_vector().magnitude_squared() - 1.0) < Point::EPSILON);
    }

    #[test]
    fn test_random() {
        assert!(Point::new_random() != Point::new_random());
    }
}
