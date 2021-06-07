use rand::Rng;

pub type Float = f64;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

impl Point {
    pub fn scale(self, s: Float) -> Point {
        return Point {
            x: self.x * s,
            y: self.y * s,
        };
    }
    pub fn add(self, that: Point) -> Point {
        return Point {
            x: self.x + that.x,
            y: self.y + that.y,
        };
    }
    pub fn minus(self, that: Point) -> Point {
        return Point {
            x: self.x - that.x,
            y: self.y - that.y,
        };
    }
    pub fn inverse(self) -> Point {
        return Point {
            x: -self.x,
            y: -self.y,
        };
    }
    pub fn magnitude_squared(self) -> Float {
        return self.x * self.x + self.y * self.y;
    }
    pub fn magnitude(self) -> Float {
        return self.magnitude_squared().sqrt();
    }
    pub fn unit_vector(self) -> Point {
        return self.scale(self.magnitude());
    }
    pub fn new_random() -> Point {
        let mut rng = rand::thread_rng();
        return Point {
            x: rng.gen::<Float>(),
            y: rng.gen::<Float>(),
        };
    }
}
