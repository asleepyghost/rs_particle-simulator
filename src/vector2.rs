pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn add(vec1: &Vector2D, vec2: &Vector2D) -> Vector2D {
        let x = vec1.x() + vec2.x();
        let y = vec1.y() + vec2.y();

        Vector2D::new(x, y)
    }

    pub fn multiply_by_scalar(vec: Vector2D, scalar: f64) -> Vector2D {
        Vector2D::new(vec.x() * scalar, vec.y() * scalar)
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn reverse(&mut self) {
        self.x = self.x * -1.0;
        self.y = self.y * -1.0;
    }
}

impl std::fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2 Position: ({}, {})", self.x, self.y)
    }
}