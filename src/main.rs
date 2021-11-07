mod vector2;
mod game;
mod particle;
mod entity;
mod physics;

use piston_window::{PistonWindow, WindowSettings, clear};
use game::Game;

// TODO: If a particle meets a particle of the same type, it joins with that particle to become a ParticleCluster
// TODO: If a ParticleCluser is at least 4 times bigger than a particle of a different type, and they meet, then the ParticleCluster destroys the particle of the other type

fn main() {
    let mut window : PistonWindow = create_window();
    let mut game = Game::new();

    game.load();

    while let Some(e) = window.next() {
        game.tick();

        window.draw_2d(&e, |ctx, gfx, _| {
            clear([0.0, 0.0, 0.0, 1.0], gfx);
            game.draw(ctx, gfx);
        });
    }
}

fn create_window() -> PistonWindow {
    WindowSettings::new("Particles", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap()
}