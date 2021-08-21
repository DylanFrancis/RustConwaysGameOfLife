mod systems;
mod components;
mod entities;
mod core;
mod resources;

use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult};
use ggez::event::{quit, KeyMods};
use ggez::input::keyboard::KeyCode;

use crate::core::{world_builder, context_builder, map_loader};
use crate::systems::render::rendering::RenderingSystem;
use crate::systems::input::input_system::{InputQueue, InputSystem};

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
        let mut input = InputSystem { };
        input.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let mut renderer = RenderingSystem {
            context: _ctx,
            size: (_RENDER_WIDTH, _RENDER_HEIGHT)
        };

        renderer.run_now(&self.world);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode)
    }
}

fn test() -> String {
    "
    .......\n
    ..xx...\n
    ..x...x\n
    ".to_string()
}
