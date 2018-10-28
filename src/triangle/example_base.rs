use std::borrow::Borrow;
use std::fmt::{self, Debug};
use back;
use hal::{Adapter, CommandPool, Backbuffer, Device, Instance, QueueGroup, Surface, Swapchain, SwapchainConfig};
use hal::format::{self, ChannelType};
use hal::{pool};
use num_cpus;

pub trait Example {
    fn render();
    fn build_command_buffers();
    fn setup_depth_stencil();
    fn setup_frame_buffer();
    fn setup_render_pass();
    fn get_enabled_features();
}

pub struct App {
    events_loop : winit::EventsLoop,
    window : winit::Window,
    instance : back::Instance,
    surface : Box<Surface<back::Backend>>,
    adapters : Vec<Adapter<back::Backend>>,
}

impl App {
    pub fn new() -> Result<App, ()> {
        // Initialize winit objects:
        let events_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(1024 as f64, 768 as f64))
            .with_title("gfx-rs application example".to_string()).build(&events_loop)
            .expect("Failed to create window.");

        // Initialize gfx core objects:
        let instance = back::Instance::create("gfx-rs application example", 1);
        let surface = Box::new(instance.create_surface(&window));
        let mut adapters = instance.enumerate_adapters();
        Ok(Self { events_loop, window, instance, surface, adapters })
    }

    // Creates the swapchain and the backbuffer it is associated with.
    /*
    fn create_swapchain_and_backbuffer(mut self, mut surface : <back::Backend as hal::Backend>::Surface) -> (Box<Swapchain<back::Backend>>, Box<Backbuffer<back::Backend>>) {
        // Grab capabilities from surface.
        let (caps, formats, _present_modes) = self.surface.compatibility(&mut self.adapter.physical_device);
        let format = formats.map_or(format::Format::Rgba8Srgb, |formats| {
            formats
                .iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .map(|format| *format)
                .unwrap_or(formats[0])
        });
        // Prepare to create swapchain.
        let swap_config = SwapchainConfig::from_caps(&caps, format);
        let (swapchain, backbuffer) = self.device
            .create_swapchain(&mut surface, swap_config, None)
            .expect("Failed to create swapchain");
        (Box::new(swapchain), Box::new(backbuffer))
    }
    */

    pub fn render_loop(&mut self) {
        // Create device using default adapter.
        let adapter = &self.adapters[0];
        let (device, queue_group) = adapter
            .open_with::<_, hal::Graphics>(1, |family| self.surface.supports_queue_family(family))
            .unwrap();

        // Create a command pool allowing up a number of command buffers according to the number of cpu cores.
        let command_pool = device
            .create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::RESET_INDIVIDUAL, num_cpus::get())
            .unwrap();
        self.events_loop.run_forever(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => winit::ControlFlow::Break,
                _ => winit::ControlFlow::Continue,
            }
        });

        device.destroy_command_pool(command_pool.into_raw());
    }
}