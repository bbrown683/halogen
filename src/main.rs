#![feature(extern_crate_item_prelude,range_contains)]
extern crate clap;
extern crate directories;
extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as back;
extern crate gltf;
extern crate image;
extern crate imgui;
extern crate log;
extern crate log4rs;
extern crate nalgebra;
extern crate num_cpus;
extern crate rayon;
extern crate rusttype;
extern crate specs;
extern crate winit;

use clap::{ Arg, App, crate_authors, crate_description, crate_name, crate_version };

mod gfx;
mod render;

fn main() {
    let _matches = App::new(crate_name!())
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

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(1024 as _, 768 as _))
        .with_title("halogen".to_string())
        .build(&events_loop)
        .expect("Failed to create window.");

    let _system = render::RenderSystem::new(&window);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => running = false,
                _ => (),
            }
        });
    }
}