use specs::{System, WriteStorage, Join, ReadStorage, Write};
use specs::shred::DynamicSystemData;
use crate::components::{Position, LiveCell, DeadCell};
use crate::resources::next_iteration::NextIteration;
use std::collections::HashMap;
use crate::systems::rules::utils;

pub struct NextGeneration {}

impl <'a> System<'a> for NextGeneration {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, LiveCell>,
        ReadStorage<'a, DeadCell>,
        Write<'a, NextIteration>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions,  live_cells,
            dead_cells, mut next_iteration) = data;

        let live_cells_map: HashMap<(u128, u128), &LiveCell> = (&positions, &live_cells)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect();

        for (pos, live_cell) in (&positions, &live_cells).join() {
            let adjacent = utils::num_adjacent_cells((pos.x, pos.y), &live_cells_map);
            let survives = adjacent >= 2 && adjacent <= 3;

            if survives {
                next_iteration.live_cells.push((pos.x, pos.y));
            }
        }
    }
}
