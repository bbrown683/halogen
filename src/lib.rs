#![feature(extern_crate_item_prelude)]
extern crate env_logger;
extern crate gfx_hal as hal;
extern crate gfx_backend_vulkan as back;
extern crate log;
extern crate nalgebra;
extern crate num_cpus;
extern crate winit;

use std::cell::RefCell;
use std::rc::Rc;
use hal::{Adapter, Backbuffer, Backend, SurfaceCapabilities, CommandPool, Device, Features,
          Instance, Limits, MemoryProperties, PhysicalDevice, PresentMode, QueueGroup, Surface,
          SwapchainConfig};
use hal::format::{self, ChannelType};
use hal::{image, pool};

// Core backend type for gfx based on the crate.
type GfxBackend = back::Backend;

// Prepare for hotswapping backends.
//type GfxVulkanBackend = vk::Backend;
//type GfxDx12Backend = dx12::Backend;

pub trait Example {
    fn render(self);
    fn build_command_buffers(self);
    fn setup_depth_stencil(self);
    fn setup_framebuffer(self);
    fn setup_render_pass(self);
    fn get_enabled_features(self);
}

pub fn hotswap_backend<B>() where B: Backend {

}

pub struct GfxExample<E: Example> {
    events_loop : winit::EventsLoop,
    window : winit::Window,
    example : Rc<RefCell<E>>,
    instance : back::Instance,
    surface : <GfxBackend as Backend>::Surface,
    adapters : Vec<Adapter<GfxBackend>>,
    device : Option<Rc<RefCell<GfxDevice<GfxBackend>>>>,
    swapchain : Option<GfxSwapchain<GfxBackend>>,
}

impl<E: Example> Drop for GfxExample<E> {
    fn drop(&mut self) {
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
    }
}

impl<E: Example> GfxExample<E> {
    pub fn new(example : Rc<RefCell<E>>) -> Self {
        let events_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_dimensions(winit::dpi::LogicalSize::new(1024 as _, 768 as _))
            .with_title("gfx-rs-example test".to_string())
            .build(&events_loop)
            .expect("Failed to create window.");
        let instance = back::Instance::create("gfx-rs-examples", 1);
        let mut surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();
        let device = Some(Rc::new(RefCell::new(GfxDevice::new(
            adapters.remove(0),
            &surface
        ))));
        let swapchain = GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            &mut surface,
            2);
        Self { window, events_loop, example, instance, surface, adapters, device, swapchain: Some(swapchain) }
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            self.events_loop.poll_events(|event| {
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
}

pub struct GfxDevice<B: Backend> {
    physical_device : B::PhysicalDevice,
    enabled_features : Features,
    memory_properties : MemoryProperties,
    limits : Limits,
    logical_device : B::Device,
    queue_group : QueueGroup<B, hal::Graphics>,
    command_pool : Option<CommandPool<B, hal::Graphics>>
}

impl<B: Backend> Drop for GfxDevice<B> {
    fn drop(&mut self) {
        &self.logical_device.destroy_command_pool(self.command_pool.take().unwrap().into_raw());
        debug_assert!(self.command_pool.is_none());
    }
}

impl<B: Backend> GfxDevice<B> {
    pub fn new(adapter : Adapter<B>, surface : &B::Surface) -> Self {
        let (logical_device, queue_group) = adapter
            .open_with::<_, hal::Graphics>(1, |family | surface.supports_queue_family(family))
            .expect("Failed to create device and queue group.");
        let command_pool = logical_device
            .create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), num_cpus::get())
            .expect("Failed to create command pool");

        let physical_device  = adapter.physical_device;
        let enabled_features = physical_device.features();
        let memory_properties = physical_device.memory_properties();
        let limits = physical_device.limits();
        Self { physical_device, enabled_features, memory_properties, limits, logical_device, queue_group,
            command_pool: Some(command_pool) }
    }
}

// Represents the Swapchain parameters for presenting to the screen.
pub struct GfxSwapchain<B: Backend> {
    caps : SurfaceCapabilities,
    swap_config : SwapchainConfig,
    device : Rc<RefCell<GfxDevice<B>>>,
    swapchain : Option<B::Swapchain>,
    backbuffer : Option<Backbuffer<B>>
}

impl<B: Backend> GfxSwapchain<B> {
    // Creates a new swapchain with the given surface. This function will only need to be called once.
    // Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               mut surface : &mut B::Surface,
               image_count : u32) -> Self {
        let (caps, formats, _present_modes) = surface.compatibility(&device.borrow().physical_device);
        let format = formats
            .map_or(format::Format::Rgba8Srgb, |formats| {
                formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .map(|format| *format)
                    .unwrap_or(formats[0])
            });

        println!("{:?}", caps);
        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Rgba8Unorm,
            image_count)
            .with_mode(PresentMode::Fifo);
        println!("{:?}", swap_config);
        let (swapchain, backbuffer) = device.borrow().logical_device
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");
        Self { caps, swap_config, device,
            swapchain: Some(swapchain), backbuffer: Some(backbuffer) }
    }

    pub fn recreate(self) {
        unimplemented!()
    }
}

impl<B: Backend> Drop for GfxSwapchain<B> {
    fn drop(&mut self) {
        &self.device.borrow().logical_device.destroy_swapchain(self.swapchain.take().unwrap());
        debug_assert!(self.swapchain.is_none());
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::{Example, GfxExample};

    pub struct EmptyExample {}
    impl EmptyExample {
        pub fn new() -> Self { Self {} }
    }
    impl Example for EmptyExample {
        fn render(self) {}
        fn build_command_buffers(self) {}
        fn setup_depth_stencil(self) {}
        fn setup_framebuffer(self) {}
        fn setup_render_pass(self) {}
        fn get_enabled_features(self) {}
    }

    #[test]
    fn empty_example() {
        println!();
        // Create an implementation of the example. For this test it will be empty to validate the processes.
        let example_impl = EmptyExample::new();
        let mut example = GfxExample::<EmptyExample>::new(
            Rc::new(RefCell::new(example_impl)));
        example.run();
    }
}