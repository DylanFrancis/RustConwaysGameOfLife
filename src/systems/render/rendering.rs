use ggez::{Context, graphics, nalgebra};
use specs::{System, ReadStorage, Join};
use specs::shred::DynamicSystemData;
use crate::components::{Position, Renderable};
use ggez::graphics::{Image, DrawParam};

const SIZE: f32 = 64.0;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context
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
            let asset = Image::new(self.context, to_render.dir.clone()).expect("expected image");

            let x = position.x as f32 * SIZE;
            let y = position.y as f32 * (SIZE / 4.0);

            let draw_params = DrawParam::new().dest(nalgebra::Point2::new(x, y));
            graphics::draw(self.context, &asset, draw_params).expect("expected to draw");
        }

        graphics::present(self.context).expect("expected to present")
    }
}
