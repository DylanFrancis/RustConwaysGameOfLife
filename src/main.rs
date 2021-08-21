mod systems;
mod components;
mod entities;
mod core;
mod resources;

use specs::{World, RunNow};
use ggez::{event, Context, GameResult};

use crate::core::{world_builder, context_builder, map_loader};
use crate::systems::render::rendering::RenderingSystem;

const _RENDER_WIDTH: u128 = 1200;
const _RENDER_HEIGHT: u128 = 800;
const _RENDER_X: u128 = 0;
const _RENDER_Y: u128 = 0;

fn main() {
    let mut world = world_builder::build((_RENDER_X, _RENDER_Y));
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
        let mut renderer = RenderingSystem {
            context: _ctx,
            pos: (_RENDER_X, _RENDER_Y),
            size: (_RENDER_WIDTH, _RENDER_HEIGHT)
        };

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
