mod systems;
mod components;
mod entities;
mod core;

use specs::{World, RunNow};
use ggez::{event, Context, GameResult};

use crate::core::{world_builder, context_builder, map_loader};
use crate::systems::render::rendering::RenderingSystem;

fn main() {
    let mut world = world_builder::build();
    let (mut context, mut events_loop) = context_builder::build();

    map_loader::load_map(&mut world, test());

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
        let mut renderer = RenderingSystem { context: _ctx };
        renderer.run_now(&self.world);

        Ok(())
    }
}

fn test() -> String {
    "
    .......\n
    ..xx...\n
    ..x...x\n
    ".to_string()
}
