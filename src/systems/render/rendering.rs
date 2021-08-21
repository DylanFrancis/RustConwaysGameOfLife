use ggez::{Context, graphics, nalgebra};
use specs::{System, ReadStorage, Join};
use specs::shred::DynamicSystemData;
use crate::components::{Position, Renderable};
use ggez::graphics::{Image, DrawParam};

const _SIZE: f32 = 64.0;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
    pub pos: (u128, u128),
    pub size: (u128, u128)
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.0, 0.0, 0.0, 1.0));

        let mut rendering_data = (&positions, &renderables)
            .join()
            .collect::<Vec<_>>();

        for (position, to_render) in rendering_data {

            if in_view(self.size, self.pos, (position.x, position.y)) {
                let asset = Image::new(self.context, to_render.dir.clone()).expect("expected image");

                let render_x = (position.x - self.pos.0) as f32 * _SIZE;
                let render_y = (position.y - self.pos.1) as f32 * (_SIZE / 4.0);

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
