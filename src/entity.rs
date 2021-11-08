use crate::physics::Bounds;
use crate::physics::Vector2D;
use crate::particle::Particle;

pub trait Entity {
    fn get_bounds(&self) -> &Bounds;
    fn get_colour(&self) -> [f32; 4];
    fn tick(&mut self);
    fn get_id(&self) -> usize;
    fn get_velocity(&self) -> &Vector2D;
    fn set_velocity(&mut self, new_velocity: Vector2D);
    
    fn as_particle(&self) -> &Particle;
} 