use crate::vector2::Vector2D;

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
        self.position.x() + self.dimensions.0
    }

    pub fn y_bounds(&self) -> f64 {
        self.position.y() + self.dimensions.1
    }

    pub fn get_bounding_rect(&self) -> [f64; 4] {
        [self.x(), self.y(), self.width(), self.height()]
    }

    pub fn get_position(&mut self) -> &Vector2D {
        &self.position
    }

    pub fn set_position(&mut self, new_position: Vector2D) {
        self.position = new_position;
    }

    pub fn contains(&self, other: &Bounds) -> bool {
        self.position.x() > other.position.x()
            && self.position.x() < other.x_bounds()
            && self.position.y() > other.position.y()
            && self.position.y() < other.y_bounds()
    }
}