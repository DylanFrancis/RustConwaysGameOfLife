mod systems;
mod components;
mod entities;
mod core;
mod resources;

use specs::{World, RunNow, WorldExt};
use ggez::{event, Context, GameResult, timer, graphics};
use ggez::event::{quit, KeyMods};
use ggez::input::keyboard::KeyCode;

use crate::core::{world_builder, context_builder, map_loader, tester};
use crate::systems::render::rendering::RenderingSystem;
use crate::systems::input::input_system::{InputQueue, InputSystem};
use crate::systems::rules::nextgeneration::NextGeneration;
use std::time::Duration;
use crate::resources::timer::Timer;
use std::borrow::BorrowMut;
use crate::systems::rules::apply_rules::ApplyRules;
use crate::systems::rules::overpopulation::OverPopulation;
use crate::systems::rules::reproduction::Reproduction;

const _RENDER_WIDTH: u128 = 1200;
const _RENDER_HEIGHT: u128 = 800;
const _RENDER_X: u128 = u128::max_value() / 2;
const _RENDER_Y: u128 = u128::max_value() / 2;

fn main() {
    let mut world = world_builder::build((_RENDER_X, _RENDER_Y));
    let (mut context, mut events_loop) = context_builder::build();

    map_loader::load_map(&mut world, test());
    // tester::test(&mut world);

    let game = &mut Game { world };

    event::run(&mut context, &mut events_loop, game);
}

struct Game {
    world: World
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // println!(
        //     "[update] ticks: {}\tfps: {}\tdelta: {:?}",
        //     timer::ticks(_ctx),
        //     timer::fps(_ctx),
        //     timer::delta(_ctx)
        // );

        let mut update_timer = self.world.fetch_mut::<Timer>();

        if update_timer.time > 1000 {
            println!("Updating");

            let mut next_gen = NextGeneration { };
            next_gen.run_now(&self.world);

            let mut over_pop = OverPopulation { };
            over_pop.run_now(&self.world);

            let mut repro = Reproduction { };
            repro.run_now(&self.world);


            let mut apply = ApplyRules { };
            apply.run_now(&self.world);

            update_timer.reset();
        }

        update_timer.add(timer::delta(_ctx).as_millis());

        let mut input = InputSystem { };
        input.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        // let fps = timer::fps(_ctx);
        // let fps_display = graphics::Text::new(format!("FPS: {}", fps));
        // println!(
        //     "[draw] ticks: {}\tfps: {}\tdelta: {:?}",
        //     timer::ticks(_ctx),
        //     fps,
        //     timer::delta(_ctx)
        // );

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
    "............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ............................
    ..................xx........
    .................xx.........
    ..x...x...........x.........
    ............................
    ............................
    ............................
    ............................
    ".to_string()
}
