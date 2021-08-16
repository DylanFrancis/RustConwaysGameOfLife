use specs::Component;
use specs::VecStorage;

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i64,
    pub y: i64
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub dir: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct LiveCell {
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct DeadCell {
}
