use specs::{System, Write};
use specs::shred::DynamicSystemData;
use ggez::input::keyboard::KeyCode;
use crate::resources::render_position::RenderPosition;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>
}

pub struct InputSystem {
    pub move_distance: u128
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, RenderPosition>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut render_position) = data;

        let key = input_queue.keys_pressed.pop();

        if key.is_some() {
            match key.unwrap() {
                KeyCode::Up => {
                    if render_position.pos.1 >= self.move_distance {
                        render_position.pos.1 = render_position.pos.1 - self.move_distance;
                    }
                },
                KeyCode::Down => {
                    if render_position.pos.1 <= u128::max_value() - self.move_distance {
                        render_position.pos.1 = render_position.pos.1 + self.move_distance;
                    }
                },
                KeyCode::Left => {
                    if render_position.pos.0 > self.move_distance {
                        render_position.pos.0 = render_position.pos.0 - self.move_distance;
                    }
                },
                KeyCode::Right => {
                    if render_position.pos.0 <= u128::max_value() - self.move_distance {
                        render_position.pos.0 = render_position.pos.0 + self.move_distance;
                    }
                },
                _ => {}
            }
        }
    }
}
