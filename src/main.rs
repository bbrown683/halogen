#![feature(extern_crate_item_prelude,range_contains)]
extern crate alto;
extern crate clap;
extern crate directories;
extern crate flate2;
extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as back;
extern crate image;
extern crate imgui;
extern crate lewton;
extern crate log;
extern crate log4rs;
extern crate nalgebra;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate num_cpus;
extern crate rayon;
extern crate rspirv;
extern crate rusttype;
extern crate specs;
extern crate spirv_headers;
extern crate tokio;
extern crate toml;
extern crate winit;

/// Internal module handling the gfx-hal library.
mod gfx;
mod util;

use std::str::FromStr;
use clap::{Arg, App, crate_authors, crate_description, crate_name, crate_version};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use winit::WindowEvent;
use crate::util::CapturedEvent;

fn main() {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .takes_value(true))
        .get_matches();

    let width = u32::from_str(matches.value_of("width").unwrap_or("1024")).unwrap();
    let height = u32::from_str(matches.value_of("height").unwrap_or("768")).unwrap();

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(width as _, height as _))
        .with_title("halogen".to_string())
        .build(&events_loop)
        .expect("Failed to create window.");

    let mut renderer = gfx::Renderer::new(&window);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => running = false,
                    WindowEvent::CursorMoved {
                            device_id, position, modifiers
                    } => (),
                    WindowEvent::KeyboardInput {
                        device_id, input
                    } => renderer.on_keyboard_input(input),
                    WindowEvent::MouseInput {
                        device_id, state, button, modifiers
                    } => renderer.on_mouse_input(button),
                    WindowEvent::Resized(size) => renderer.on_resize(size),
                    _ => (),
                }
            }
        });

        &renderer.begin_frame();
        &renderer.end_frame();
    }
}