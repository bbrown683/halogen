extern crate ash;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate nalgebra;
extern crate winit;

/// High-level wrapper for [ash](https://github.com/MaikKlein/ash) around typical types.
pub mod graphics;
pub mod util;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::util::CapturedEvent;

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout_appender", Box::new(stdout)))
        .build(Root::builder().appender("stdout_appender").build(LevelFilter::Info))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();

    let events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1024, 768))
        .with_title("Halogen".to_string())
        .with_resizable(true)
        .build(&events_loop)
        .expect("Failed to create window.");

    let mut renderer = graphics::Renderer::new(&window);

    events_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => renderer.on_resize(size),
                _ => (),
            }
        }
        renderer.draw_frame();
    });
}