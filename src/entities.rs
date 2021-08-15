use specs::{World, WorldExt, Builder};
use crate::components::{Position, Renderable};

pub fn create_live_cell(world: &mut World, pos: Position) {
    world.create_entity()
        .with(Position {..pos})
        .with(Renderable {dir: "/assets/pepe_64.png".to_string()})
        .build();
}

pub fn create_dead_cell(world: &mut World, pos: Position) {
}
