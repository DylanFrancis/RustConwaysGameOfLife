mod systems;
mod components;
mod entities;
mod core;

use specs::World;
use ggez::{event, Context, GameResult};

use crate::core::{world_builder, context_builder};

fn main() {
    let mut world = world_builder::build();
    let (mut context, mut events_loop) = context_builder::build();

    let game = &mut Game { world };

    event::run(&mut context, &mut events_loop, game);
}

struct Game {
    world: World
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
