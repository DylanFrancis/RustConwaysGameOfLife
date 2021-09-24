use specs::{System, WriteStorage, Join, ReadStorage, Write};
use specs::shred::DynamicSystemData;
use std::collections::HashMap;
use crate::components::{Position, LiveCell, DeadCell};
use crate::resources::next_iteration::NextIteration;
use crate::systems::rules::utils;
use crate::systems::rules::rule::{Rule};
use specs::join::JoinIter;

pub struct NextGeneration;

impl Rule for NextGeneration {
    fn perform_rule(live_cells: &HashMap<(u128, u128), &LiveCell>, next_iteration_live_cells: &mut Vec<(u128, u128)>){
        for (pos, _live_cell) in live_cells {
            let num_adjacent = utils::num_adjacent_cells((pos.0, pos.1), &live_cells);

            if num_adjacent >= 2 && num_adjacent <= 3 {
                next_iteration_live_cells.push((pos.0, pos.1));
            }
        }
    }
}

impl <'a> System<'a> for NextGeneration where NextGeneration: Rule{
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, LiveCell>,
        Write<'a, NextIteration>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions,  live_cells, mut next_iteration) = data;

        let live_cells_map: HashMap<(u128, u128), &LiveCell> = (&positions, &live_cells)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect();

        self::NextGeneration::perform_rule(&live_cells_map, next_iteration.live_cells.as_mut());
    }
}
