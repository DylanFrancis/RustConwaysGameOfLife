use std::collections::HashMap;
use crate::components::{LiveCell, DeadCell};


#[derive(Default)]
pub struct NextIteration {
    pub live_cells: Vec<(u128, u128)>,
    pub dead_cells: Vec<(u128, u128)>
}

