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
use crate::util::CapturedEvent;

fn main() {
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

    let mut system = gfx::Renderer::new(&window);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => running = false,
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Resized(size),
                    ..
                } => system.on_resize(size),
                _ => (),
            }
        });

        &system.begin_frame();
        &system.end_frame();
    }
}