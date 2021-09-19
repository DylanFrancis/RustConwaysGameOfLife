use specs::{System, Write, WriteStorage, Join, Storage, World, WorldExt, Entities, Builder};
use specs::shred::{DynamicSystemData};
use crate::resources::next_iteration::NextIteration;
use crate::components::{Position, Renderable, LiveCell, DeadCell};
use std::borrow::{Borrow, BorrowMut};

pub struct ApplyRules {

}

impl<'a> System<'a> for ApplyRules {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, LiveCell>,
        WriteStorage<'a, DeadCell>,
        Write<'a, NextIteration>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut renderables,
            mut live_cells, mut dead_cells,
            mut next_iteration, mut entities) = data;

        Storage::clear(&mut live_cells);
        Storage::clear(&mut positions);
        Storage::clear(&mut renderables);

        for live_cell in &next_iteration.live_cells {
            let new_cell = entities.create();

            live_cells.insert(new_cell, LiveCell{});
            positions.insert(new_cell, Position{x: live_cell.0, y: live_cell.1});
            renderables.insert(new_cell, Renderable{dir: "/assets/peepo_32.jpg".to_string()});
        }

        for dead_cell in &next_iteration.dead_cells {
            let new_cell = entities.create();

            dead_cells.insert(new_cell, DeadCell{});
            positions.insert(new_cell, Position{x: dead_cell.0, y: dead_cell.1});
        }

        next_iteration.live_cells.clear();
        next_iteration.dead_cells.clear();

        World::maintain;
    }
}
