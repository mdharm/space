use rand::Rng;
use std::ops::*;

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
        self.scale(1.0 / self.magnitude())
    }

    pub fn new_random() -> Point {
        let mut rng = rand::thread_rng();
        Point(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = self.add(other);
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = self.minus(other);
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.minus(other)
    }
}

impl MulAssign<Float> for Point {
    fn mul_assign(&mut self, rhs: Float) {
        *self = self.scale(rhs)
    }
}

impl Mul<Float> for Point {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        self.scale(rhs)
    }
}

impl DivAssign<Float> for Point {
    fn div_assign(&mut self, rhs: Float) {
        *self = self.scale(1.0 / rhs)
    }
}

impl Div<Float> for Point {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        self.scale(1.0 / rhs)
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
        assert!((Point::new_random().unit_vector().magnitude() - 1.0) < Point::EPSILON);
    }

    #[test]
    fn test_random() {
        assert!(Point::new_random() != Point::new_random());
    }

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !(($x - $y).abs() < $d) {
                panic!();
            }
        };
    }

    macro_rules! assert_def {
        ($x:expr, $y:expr) => {
            assert_delta!($x, $y, Point::EPSILON)
        };
    }

    #[test]
    fn test_add2() {
        let one = Point(1.0, 1.0);
        let zero = Point(0.0, 0.0);
        assert_eq!(one, one.add(zero));
        assert_eq!(one, one.add(one).minus(one));
    }

    #[test]
    fn test_magnitude2() {
        let one = Point(1.0, 1.0);
        assert_def!(2.0, one.magnitude_squared());
        assert_def!(2.0, one.magnitude() * one.magnitude());
    }

    #[test]
    fn test_unit_vector2() {
        let one = Point(2.0, 2.0);
        assert_def!(1.0, one.unit_vector().magnitude());
        assert_def!(one.unit_vector().0, one.unit_vector().1)
    }

    #[test]
    fn test_traits() {
        let two = Point(2.0, 2.0);
        let one = Point(1.0, 1.0);
        let zero = Point(0.0, 0.0);

        // operator +=
        let mut result = zero;
        result += one;
        assert_eq!(result, one);
        result += one;
        assert_eq!(result, two);

        // operator +
        assert_eq!(one, one + zero);
        assert_eq!(two, one + one);
        assert!(Point(4.0, 5.0) + Point(1.0, 2.0) == Point(5.0, 7.0));

        // operator -=
        result = two;
        result -= one;
        assert_eq!(result, one);
        result -= one;
        assert_eq!(result, zero);

        // operator -
        assert_eq!(zero, one - one);
        assert_eq!(one, one - zero);
        assert_eq!(one, two - one);
        assert!(Point(4.0, 5.0) - Point(1.0, 2.0) == Point(3.0, 3.0));

        // operator *=
        result = one;
        result *= 2.0;
        assert_eq!(result, two);

        // operator *
        assert_eq!(two, one * 2.0);
        assert_eq!(zero, one * 0.0);
    }
}
