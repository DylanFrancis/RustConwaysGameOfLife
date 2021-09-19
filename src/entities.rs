use specs::{World, WorldExt, Builder};
use crate::components::{Position, Renderable, LiveCell, DeadCell};

pub fn create_live_cell(world: &mut World, pos: Position) {
    println!("creating live cell");
    world.create_entity()
        .with(Position {..pos})
        .with(Renderable {dir: "/assets/peepo_32.jpg".to_string()})
        .with(LiveCell {})
        .build();
}

pub fn create_dead_cell(world: &mut World, pos: Position) {
    world.create_entity()
        .with(Position{..pos})
        .with(DeadCell {})
        .build();
}
