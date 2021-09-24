use specs::{System, Write, ReadStorage, Join};
use specs::shred::DynamicSystemData;
use std::collections::HashMap;
use crate::components::{Position, LiveCell, DeadCell};
use crate::resources::next_iteration::NextIteration;
use crate::systems::rules::utils;
use crate::systems::rules::rule::Rule;

pub struct OverPopulation;

impl Rule for OverPopulation {
    fn perform_rule(live_cells: &HashMap<(u128, u128), &LiveCell>, next_iteration_dead_cells: &mut Vec<(u128, u128)>) {
        for (pos, _live_cell) in live_cells {
            let dies = utils::num_adjacent_cells((pos.0, pos.1), &live_cells) > 3;

            if dies {
                next_iteration_dead_cells.push((pos.0, pos.1));
            }
        }
    }
}

impl<'a> System<'a> for OverPopulation {
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

        self::OverPopulation::perform_rule(&live_cells_map, next_iteration.dead_cells.as_mut());
    }
}
