use specs::{System, Join, ReadStorage, Write};
use specs::shred::DynamicSystemData;
use std::collections::{HashMap, HashSet};
use crate::components::{LiveCell, Position, DeadCell};
use crate::systems::rules::utils;
use crate::resources::next_iteration::NextIteration;
use crate::systems::rules::rule::Rule;

pub struct Reproduction;

impl Rule for Reproduction {
    fn perform_rule(live_cells: &HashMap<(u128, u128), &LiveCell>, next_iteration_live_cells: &mut Vec<(u128, u128)>){
        let mut all_adjacent_dead_cells: HashSet<(u128, u128)> = HashSet::new();

        for (pos, _live_cell) in live_cells {
            let adjacent_dead_cells: Vec<(u128, u128)> = utils::adjacent_dead_cells((pos.0, pos.1), &live_cells);

            for adj_dead_cell in adjacent_dead_cells {
                if !all_adjacent_dead_cells.contains(&adj_dead_cell) {
                    all_adjacent_dead_cells.insert(adj_dead_cell);
                }
            }
        }

        for dead_cell in all_adjacent_dead_cells {
            let alive = utils::num_adjacent_cells(dead_cell, &live_cells) == 3;

            if alive {
                next_iteration_live_cells.push((dead_cell.0, dead_cell.1));
            }
        }
    }
}

impl<'a> System<'a> for Reproduction {
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

        self::Reproduction::perform_rule(&live_cells_map, next_iteration.live_cells.as_mut());
    }
}
