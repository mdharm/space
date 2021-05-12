pub type Float = f64;

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
    pub fn magnitudeSquared(self) -> Float {
        return self.x * self.x + self.y * self.y;
    }
    pub fn unitVector(self) -> Point {
        return self.scale(self.magnitudeSquared().sqrt());
    }
}
