type Float = f64;

pub struct Point {
    pub x: Float,
    pub y: Float,
}
impl Point {
    pub fn scale(&self, s: Float) -> &Point {
        self.x *= s;
        self.y *= s;
        return self;
    }
}
pub struct Node {
    mass: Option<Mass>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
pub struct Mass {
    pub position: Point,
    pub velocity: Point,
    pub mass: Float,
}
impl Mass {
    pub fn new_random() -> Mass {
        let m: Mass;
        return m;
    }
}
impl Node {
    pub fn new_leaf() -> Node {
        let n: Node;
        return n;
    }
    pub fn new(l: Node, r: Node) -> Node {
        let n: Node;
        return n;
    }
    pub fn add(&self, mass: Mass) -> &Node {
        let n = self;
        if self.left.is_none() {
            let n = Node::new_leaf();
            self.left = Option::from(Box::new(n));
        } else if self.right.is_none() {
            let n = Node::new_leaf();
            self.right = Option::from(Box::new(n));
        } else {
        }
        return n;
    }
    pub fn recalc(&self) {}
}
