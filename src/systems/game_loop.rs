use specs::System;
use specs::shred::DynamicSystemData;

struct GameLoop {

}

impl<'a> System<'a> for GameLoop {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {

    }
}
