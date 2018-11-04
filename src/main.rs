#![feature(extern_crate_item_prelude,range_contains)]
extern crate env_logger;
extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as back;
extern crate gltf;
extern crate image;
extern crate imgui;
extern crate log;
extern crate nalgebra;
extern crate num_cpus;
extern crate rayon;
extern crate rusttype;
extern crate specs;
extern crate winit;

mod gfx;
mod render;

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::new(1024 as _, 768 as _))
        .with_title("gfx-rs-example test".to_string())
        .build(&events_loop)
        .expect("Failed to create window.");

    let system = render::RenderSystem::new(&window);

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