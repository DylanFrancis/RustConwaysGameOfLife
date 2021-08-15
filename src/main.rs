use specs::World;
use ggez::{event, Context, GameResult};

fn main() {
}

struct Game {
    world: World
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        todo!()
    }
}
