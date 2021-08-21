use specs::{World, WorldExt};
use crate::components::{Position, Renderable, LiveCell, DeadCell};
use crate::resources::next_iteration::NextIteration;
use crate::resources::render_position::RenderPosition;

pub fn build(render_size: (u128, u128)) -> World {
    let mut world = World::new();

    register_components(&mut world);
    register_resources(&mut world, render_size);

    world
}

fn register_resources(world: &mut World, render_size: (u128, u128)) {
    world.insert(NextIteration::default());
    world.insert(RenderPosition {pos: render_size});
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<LiveCell>();
    world.register::<DeadCell>();
}
