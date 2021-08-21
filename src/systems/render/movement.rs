use specs::{System, ReadStorage, Read};
use specs::shred::DynamicSystemData;

use crate::resources::render_position::RenderPosition;

pub struct MovementSystem {

}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (Read<'a, RenderPosition>);

    fn run(&mut self, data: Self::SystemData) {

    }
}