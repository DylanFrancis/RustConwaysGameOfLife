use specs::{System, WriteStorage, Join, ReadStorage, Write};
use specs::shred::DynamicSystemData;
use crate::components::{Position, LiveCell, DeadCell};
use crate::resources::next_iteration::NextIteration;
use std::collections::HashMap;

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

        let live_cells_map: HashMap<(i64, i64), &LiveCell> = (&positions, &live_cells)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect();

        for (pos, live_cell) in (&positions, &live_cells).join() {
            let survives = will_survive((pos.x, pos.y), &live_cells_map);

            if survives {
                next_iteration.live_cells.push((pos.x, pos.y));
            }
        }
    }
}

pub fn will_survive(pos: (i64, i64), other_live_cells: &HashMap<(i64, i64), &LiveCell>) -> bool {
    let mut adjacent: u8 = 0;

    let top = (pos.0 - 1, pos.1);
    if other_live_cells.contains_key(&top) {
        adjacent += 1;
    }

    let top_right = (pos.0 - 1, pos.1 + 1);
    if other_live_cells.contains_key(&top_right) {
        adjacent += 1;
    }

    let right = (pos.0, pos.1 + 1);
    if other_live_cells.contains_key(&right) {
        adjacent += 1;
    }

    let bot_right = (pos.0 + 1, pos.1 + 1);
    if other_live_cells.contains_key(&bot_right) {
        adjacent += 1;
    }

    let bot = (pos.0 + 1, pos.1);
    if other_live_cells.contains_key(&bot) {
        adjacent += 1;
    }

    let bot_left = (pos.0 + 1, pos.1 - 1);
    if other_live_cells.contains_key(&bot_left) {
        adjacent += 1;
    }

    let left = (pos.0, pos.1 - 1);
    if other_live_cells.contains_key(&left) {
        adjacent += 1;
    }

    let top_left = (pos.0 - 1, pos.1 - 1);
    if other_live_cells.contains_key(&top_left) {
        adjacent += 1;
    }

    adjacent >= 2 || adjacent <= 3
}
