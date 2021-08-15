use specs::{World, WorldExt};
use crate::components::{Position, Renderable, LiveCell, DeadCell};

pub fn build() -> World {
    let mut world = World::new();

    register_components(&mut world);

    world
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<LiveCell>();
    world.register::<DeadCell>();
}
