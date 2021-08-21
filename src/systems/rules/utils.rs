use std::collections::HashMap;
use crate::components::LiveCell;

pub fn adjacent_dead_cells(pos: (u128, u128), other_live_cells: &HashMap<(u128, u128), &LiveCell>) -> Vec<(u128, u128)> {
    let mut dead_cells = Vec::new();

    let top = (pos.0 - 1, pos.1);
    if !other_live_cells.contains_key(&top) {
        dead_cells.push(top);
    }

    let top_right = (pos.0 - 1, pos.1 + 1);
    if !other_live_cells.contains_key(&top_right) {
        dead_cells.push(top_right);
    }

    let right = (pos.0, pos.1 + 1);
    if !other_live_cells.contains_key(&right) {
        dead_cells.push(right);
    }

    let bot_right = (pos.0 + 1, pos.1 + 1);
    if !other_live_cells.contains_key(&bot_right) {
        dead_cells.push(bot_right);
    }

    let bot = (pos.0 + 1, pos.1);
    if !other_live_cells.contains_key(&bot) {
        dead_cells.push(bot);
    }

    let bot_left = (pos.0 + 1, pos.1 - 1);
    if !other_live_cells.contains_key(&bot_left) {
        dead_cells.push(bot_left);
    }

    let left = (pos.0, pos.1 - 1);
    if !other_live_cells.contains_key(&left) {
        dead_cells.push(left);
    }

    let top_left = (pos.0 - 1, pos.1 - 1);
    if !other_live_cells.contains_key(&top_left) {
        dead_cells.push(top_left);
    }

    dead_cells
}

pub fn num_adjacent_cells(pos: (u128, u128), other_live_cells: &HashMap<(u128, u128), &LiveCell>) -> u8 {
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

    adjacent
}
