use graphics::{Context, Graphics, rectangle};
use rand::Rng;

use crate::entity::Entity;
use crate::particle::{Particle, get_random_particle_type};
use crate::vector2::Vector2D;

pub struct Game {
    entities: Vec<Box<dyn Entity>>,
}

fn generate_particle(rng: &mut rand::prelude::ThreadRng) -> Box<dyn Entity> {
    let position = Vector2D::new(rng.gen::<f64>() * 800.0, rng.gen::<f64>() * 600.0);
    let (width, height) = (5.0, 5.0);
    
    Box::new(Particle::new(position, width, height, get_random_particle_type()))
}

impl Game {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }

    pub fn load(&mut self) {
        let mut rng = rand::thread_rng();

        self.entities = [0; 100].iter().map(|_| generate_particle(&mut rng)).collect();
    }

    pub fn tick(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.tick();
        }
    }

    pub fn draw<G: Graphics> (&self, c: Context, g: &mut G) {
        for entity in &self.entities {
            rectangle(entity.get_colour(),
                      entity.get_bounds().get_bounding_rect(),
                      c.transform,
                      g);
        }
    }
}