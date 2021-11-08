use graphics::{Context, Graphics, rectangle};
use rand::Rng;

use crate::entity::Entity;
use crate::particle::{Particle, get_random_particle_type};
use crate::physics::{Vector2D, Bounds};

pub struct Game {
    entities: Vec<Box<dyn Entity>>
}

fn generate_particle(rng: &mut rand::prelude::ThreadRng, idx: usize) -> Box<dyn Entity> {
    let position = Vector2D::new(rng.gen::<f64>() * 800.0, rng.gen::<f64>() * 600.0);
    let (width, height) = (5.0, 5.0);
    
    Box::new(Particle::new(position, width, height, get_random_particle_type(), idx))
}

impl Game {
    pub fn new() -> Self {
        Self {
            entities: vec![]
        }
    }

    pub fn load(&mut self) {
        let mut rng = rand::thread_rng();

        self.entities = [0; 100]
            .iter()
            .enumerate()
            .map(|(idx, _)| generate_particle(&mut rng, idx + 1))
            .collect();
    }

    pub fn tick(&mut self) {
        for entity in self.entities.iter_mut() {
            entity.tick();
        }
        
        self.check_for_collisions();
    }

     // this method sucks help me
     pub fn check_for_collisions(&mut self) {
        let mut entity_ids_to_reverse : Vec<usize> = vec![];
        let mut entity_ids_to_remove : Vec<usize> = vec![];

        let entities = &mut self.entities;

        for i in 0..entities.len() {
            let other_entity = &entities[i];
            for entity in entities.iter() {
                if Bounds::contains(&entity.get_bounds(), &other_entity.get_bounds()) && entity.get_id() != other_entity.get_id() {
                    if entity.as_particle().particle_type == other_entity.as_particle().particle_type {
                        entity_ids_to_remove.push(entity.get_id());
                        entity_ids_to_remove.push(other_entity.get_id());
                        continue;
                    }

                    entity_ids_to_reverse.push(entity.get_id());
                    entity_ids_to_reverse.push(other_entity.get_id());
                }
            }
        }

        for id in entity_ids_to_remove {
            entities.retain(|e| e.get_id() != id);
        }

        for id in entity_ids_to_reverse {
            for entity in entities.iter_mut() {
                if id == entity.get_id() {
                    entity.set_velocity(Vector2D::reverse(entity.get_velocity()));
                }
            }
        }
    }

    pub fn draw<G: Graphics> (&self, c: Context, g: &mut G) {
        for entity in &self.entities {
            rectangle(entity.get_colour(),
                      entity.get_bounds().get_entity_rect(),
                      c.transform,
                      g);
        }
    }
}