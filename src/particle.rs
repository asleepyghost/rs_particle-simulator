use rand::Rng;
use crate::entity::Entity;
use crate::physics::{Bounds, Vector2D};

#[derive(Debug, PartialEq)]
pub enum ParticleType {
    BLUE = 0,
    RED = 1,
    GREEN = 2,
    ORANGE = 3,
    BROWN = 4,
    WHITE = 5,
    PURPLE = 6,
    PINK = 7
}

pub struct Particle {
    bounds: Bounds,
    velocity: Vector2D,
    pub particle_type: ParticleType,
    id: usize,
}

impl Particle {
    pub fn new(position : Vector2D, width : f64, height : f64, particle_type: ParticleType, id: usize) -> Self {
        let bounds = Bounds::new(position, (width, height));

        Self {
            bounds,
            particle_type,
            velocity: get_initial_velocity(),
            id,
        }
    }

    pub fn handle_movement(&mut self) {
        let new_position: Vector2D = Vector2D::add(self.bounds.get_position(), &self.velocity);

        self.bounds.set_position(new_position);
    }

    pub fn handle_wall_collision(&mut self) {
        let position = self.bounds.get_position();

        if position.x() < 0.0 {
            self.velocity.set_x(self.velocity.x() * -1.0);
        }

        if position.x() > 800.0 {
            self.velocity.set_x(self.velocity.x() * -1.0);
        }

        if position.y() < 0.0 {
            self.velocity.set_y(self.velocity.y() * -1.0);
        }

        if position.y() > 600.0 {
            self.velocity.set_y(self.velocity.y() * -1.0);
        }
    }
}

impl Entity for Particle {
    fn get_bounds(&self) -> &Bounds {
        &self.bounds
    }

    fn get_colour(&self) -> [f32;4] {
        match &self.particle_type {
            ParticleType::BLUE => [0.0, 0.0, 1.0, 1.0],
            ParticleType::RED => [1.0, 0.0, 0.0, 1.0],
            ParticleType::GREEN => [0.0, 1.0, 0.0, 1.0],
            ParticleType::ORANGE => [1.0, 0.95, 0.0, 1.0],
            ParticleType::BROWN => [0.64, 0.16, 0.16, 1.0],
            ParticleType::WHITE => [1.0, 1.0, 1.0, 1.0],
            ParticleType::PURPLE => [0.9, 0.9, 0.98, 1.0],
            ParticleType::PINK => [0.62, 0.16, 0.4, 1.0]
        }
    }

    fn tick(&mut self) {
        self.handle_movement();
        self.handle_wall_collision();
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_velocity(&self) -> &Vector2D {
        &self.velocity
    }

    fn set_velocity(&mut self, velocity: Vector2D) {
        self.velocity = velocity;
    }

    fn as_particle(&self) -> &Particle {
        &self
    }
}

pub fn get_random_particle_type() -> ParticleType {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<f32>();

    match num {
       num if num < 0.125 => ParticleType::BLUE,
       num if num < 0.25 => ParticleType::RED,
       num if num < 0.375 => ParticleType::GREEN,
       num if num < 0.5 => ParticleType::ORANGE,
       num if num < 0.625 => ParticleType::BROWN,
       num if num < 0.75 => ParticleType::WHITE,
       num if num < 0.875 => ParticleType::PURPLE,
       _ => ParticleType::PINK
    }
}

pub fn get_initial_velocity() -> Vector2D {
    let mut rng = rand::thread_rng();

    let reverse_x = rng.gen::<f64>() > 0.5;
    let reverse_y = rng.gen::<f64>() > 0.5;

    let multiplier = rng.gen::<f64>() * 5.0;

    let mut initial_velocity = Vector2D::new(rng.gen::<f64>() * multiplier, rng.gen::<f64>() * multiplier);

    if reverse_x {
        initial_velocity.set_x(initial_velocity.x() * -1.0);
    }

    if reverse_y {
        initial_velocity.set_y(initial_velocity.y() * -1.0);
    }

    initial_velocity
}