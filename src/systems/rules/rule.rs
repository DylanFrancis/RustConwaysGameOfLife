use crate::components::{Position, LiveCell};
use std::collections::HashMap;
use specs::join::JoinIter;

pub trait Rule {
    fn perform_rule(live_cells: &HashMap<(u128, u128), &LiveCell>, next_iteration_cells: &mut Vec<(u128, u128)>);
}