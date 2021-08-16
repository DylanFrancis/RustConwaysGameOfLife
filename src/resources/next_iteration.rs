use std::collections::HashMap;
use crate::components::{LiveCell, DeadCell};


#[derive(Default)]
pub struct NextIteration {
    pub live_cells: Vec<(i64, i64)>,
    pub dead_cells: Vec<(i64, i64)>
}

