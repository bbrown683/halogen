#![feature(range_contains)]
extern crate alto;
extern crate ash;
extern crate lewton;
#[macro_use] extern crate log;
extern crate log4rs;
extern crate nalgebra;
extern crate num_cpus;
extern crate rayon;
extern crate spirv_reflect;
extern crate winit;

///
pub mod audio;
/// High-level wrapper for [ash](https://github.com/MaikKlein/ash) around typical types.
pub mod graphics;
pub mod util;

use log::{LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use winit::WindowEvent;
use crate::util::CapturedEvent;

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout_appender", Box::new(stdout)))
        .build(Root::builder().appender("stdout_appender").build(LevelFilter::Debug))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(1024 as _, 768 as _))
        .with_title("Halogen".to_string())
        .with_resizable(false)
        .build(&events_loop)
        .expect("Failed to create window.");

    let mut renderer = graphics::Renderer::new(&window);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::Resized(size) => renderer.on_resize(size),
                    _ => (),
                }
            }
        });
        &renderer.begin_frame();
        &renderer.end_frame();
    }
}