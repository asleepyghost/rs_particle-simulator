#[derive(Debug)]
pub struct Bounds {
    position: Vector2D,
    dimensions: (f64, f64)
}

impl Bounds {
    pub fn new(position : Vector2D, dimensions : (f64, f64)) -> Self {
        Self {
            position,
            dimensions
        }
    }

    pub fn x(&self) -> f64 {
        self.position.x()
    }

    pub fn y(&self) -> f64 {
        self.position.y()
    }

    pub fn width(&self) -> f64 {
        self.dimensions.0
    }

    pub fn height(&self) -> f64 {
        self.dimensions.1
    }

    pub fn x_bounds(&self) -> f64 {
        self.position.x() + self.width()
    }

    pub fn y_bounds(&self) -> f64 {
        self.position.y() + self.height()
    }

    pub fn get_entity_rect(&self) -> [f64; 4] {
        [self.x(), self.y(), self.width(), self.height()]
    }

    pub fn get_position(&mut self) -> &Vector2D {
        &self.position
    }

    pub fn set_position(&mut self, new_position: Vector2D) {
        self.position = new_position;
    }

    // Other is a 4 item array like: [x, y, x+width, y+height]
    pub fn contains(me: &Bounds, other: &Bounds) -> bool {
        me.position.x() > other.x()
            && me.position.x() < other.x_bounds()
            && me.position.y() > other.y()
            && me.position.y() < other.y_bounds()
    }
}

#[derive(Debug)]
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

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn reverse(vector: &Vector2D) -> Vector2D {
        let x = vector.x * -1.0;
        let y = vector.y * -1.0;

        Vector2D::new(x, y)
    }
}

impl std::fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector2 Position: ({}, {})", self.x, self.y)
    }
}