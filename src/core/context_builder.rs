use ggez::event::EventsLoop;
use ggez::{Context, conf};

use std::path;

pub fn build() -> (Context, EventsLoop) {
    let context_builder = ggez::ContextBuilder::new("rust_cgl", "cgl")
        .window_setup(conf::WindowSetup::default().title("Rust Life!"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 1000.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    context_builder.build().unwrap()
}
