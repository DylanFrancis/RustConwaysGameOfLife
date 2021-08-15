use specs::{World, WorldExt, Builder};
use crate::components::{Position, Renderable};

pub fn createLiveCell(world: &mut World, pos: Position) {
    world.create_entity()
        .with(Position {..pos})
        .with(Renderable {dir: "/resources/assets".to_string()})
        .build();
}

pub fn createDeadCell(world: &mut World, pos: Position) {
}
