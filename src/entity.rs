use crate::physics::Bounds;

pub trait Entity {
    fn get_bounds(&self) -> &Bounds;
    fn get_colour(&self) -> [f32; 4];
    fn tick(&mut self);
} 