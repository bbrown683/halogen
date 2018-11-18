use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use winit::dpi::{LogicalPosition, LogicalSize};
use super::{Device, Framebuffer, Instance, RenderPass, Swapchain, Queue};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `Renderer` manages all render state.
pub struct Renderer {
    instance : Option<Instance>,
    device : Option<Rc<RefCell<Device>>>,
    compute_queue : Option<Rc<RefCell<Queue>>>,
    graphics_queue : Option<Rc<RefCell<Queue>>>,
    transfer_queue : Option<Rc<RefCell<Queue>>>,
    swapchain : Option<Swapchain>,
    default_render_pass : Option<RenderPass>,
    framebuffers : Option<Vec<Framebuffer>>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.framebuffers.take();
        debug_assert!(self.framebuffers.is_none());
        self.default_render_pass.take();
        debug_assert!(self.default_render_pass.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.compute_queue.take();
        debug_assert!(self.compute_queue.is_none());
        self.graphics_queue.take();
        debug_assert!(self.graphics_queue.is_none());
        self.transfer_queue.take();
        debug_assert!(self.transfer_queue.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
        self.instance.take();
        debug_assert!(self.instance.is_none());
        info!("Dropped Renderer.");
    }
}

impl CapturedEvent for Renderer {
    /// When this event is captured, the swapchain is recreated, and regenerates all framebuffers from the swapchain images.
    fn on_resize(&mut self, _size : LogicalSize) {

    }
}

impl Renderer {
    /// Initializes the renderer for the specified window.
    pub fn new(window : &winit::Window) -> Self {
        unsafe {
            info!("Initializing Renderer.");
            // TODO: Properly handle errors here and present them to the output.

            let instance = Instance::new();
            let device = Rc::new(RefCell::new(Device::new(&instance)
                .ok()
                .unwrap()));

            // Create our queues.
            let compute_queue = Rc::new(RefCell::new(Queue::new(
                Rc::clone(&device),
                device.borrow().get_compute_queue_index())));
            let graphics_queue = Rc::new(RefCell::new(Queue::new(
                Rc::clone(&device),
                device.borrow().get_graphics_queue_index())));
            let transfer_queue = Rc::new(RefCell::new(Queue::new(
                Rc::clone(&device),
                device.borrow().get_transfer_queue_index())));

            // Create the swapchain.
            let swapchain = Swapchain::new(
                &instance,
                &device.borrow(),
                Rc::clone(&graphics_queue),
                window,
                2).ok()
                .unwrap();

            let default_render_pass = RenderPass::new(
                Rc::clone(&device));

            // Grab the swapchain images to create the framebuffers.
            let mut framebuffers = Vec::<Framebuffer>::new();
            for image in swapchain.get_images() {
                framebuffers.push(Framebuffer::new(
                    Rc::clone(&device),
                    &default_render_pass,
                    image,
                    swapchain.get_extent()
                ));
            }

            info!("Renderer has been initialized.");
            Self { instance: Some(instance),
                device: Some(device),
                compute_queue: Some(compute_queue),
                graphics_queue: Some(graphics_queue),
                transfer_queue: Some(transfer_queue),
                swapchain: Some(swapchain),
                default_render_pass: Some(default_render_pass),
                framebuffers: Some(framebuffers)
            }
        }
    }

    pub fn begin_frame(&mut self) {
    }

    pub fn end_frame(&mut self) {

    }
}
