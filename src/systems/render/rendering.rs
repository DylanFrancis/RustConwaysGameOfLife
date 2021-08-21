use ggez::{Context, graphics, nalgebra};
use ggez::graphics::{Image, DrawParam};
use specs::{System, ReadStorage, Join, Read};
use specs::shred::DynamicSystemData;
use crate::resources::render_position::RenderPosition;
use crate::components::{Position, Renderable, LiveCell};

const _SIZE: f32 = 64.0;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
    pub size: (u128, u128)
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        Read<'a, RenderPosition>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, render_position) = data;

        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut rendering_data = (&positions, &renderables)
            .join()
            .collect::<Vec<_>>();

        for (position, to_render) in rendering_data {
            if in_view(self.size, render_position.pos, (position.x, position.y)) {
                let asset = Image::new(self.context, to_render.dir.clone()).expect("expected image");

                let render_x = (position.x - render_position.pos.0) as f32 * _SIZE;
                let render_y = (position.y - render_position.pos.1) as f32 * _SIZE;

                let draw_params = DrawParam::new().dest(nalgebra::Point2::new(render_x, render_y));
                graphics::draw(self.context, &asset, draw_params).expect("expected to draw");
            }
        }

        graphics::present(self.context).expect("expected to present")
    }
}

fn in_view(render_size: (u128, u128), render_pos: (u128, u128), object_pos: (u128, u128)) -> bool {
    render_pos.0 + render_size.0 >= object_pos.0 &&
        render_pos.0 <= object_pos.0 &&
        render_pos.1 + render_size.1 >= object_pos.1 &&
        render_pos.1 <= object_pos.1
}
