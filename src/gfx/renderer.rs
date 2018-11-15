use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use hal::{Compute, Graphics, Transfer};
use winit::dpi::{LogicalPosition, LogicalSize};
use crate::gfx::{Backend, BackendType, CmdBuffer, CmdPool, Device, Framebuffer, Queue, QueueSet,
                 RenderPass, Swapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `Renderer` manages all render state.
pub struct Renderer {
    backend : Backend,
    device : Option<Rc<RefCell<Device<BackendType>>>>,
    queue_set : Option<QueueSet<BackendType>>,
    compute_pool : Option<Rc<RefCell<CmdPool<BackendType>>>>,
    graphics_pool : Option<Rc<RefCell<CmdPool<BackendType>>>>,
    graphics_buffer : Option<CmdBuffer<BackendType>>,
    transfer_pool : Option<Rc<RefCell<CmdPool<BackendType>>>>,
    swapchain : Option<Rc<RefCell<Swapchain<BackendType, Graphics>>>>,
    default_render_pass : Option<RenderPass<BackendType>>,
    framebuffers : Option<Vec<Framebuffer<BackendType>>>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.default_render_pass.take();
        debug_assert!(self.default_render_pass.is_none());
        self.framebuffers.take();
        debug_assert!(self.framebuffers.is_none());
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.compute_pool.take();
        debug_assert!(self.compute_pool.is_none());
        self.graphics_buffer.take();
        debug_assert!(self.graphics_buffer.is_none());
        self.graphics_pool.take();
        debug_assert!(self.graphics_pool.is_none());
        self.transfer_pool.take();
        debug_assert!(self.transfer_pool.is_none());
        self.queue_set.take();
        debug_assert!(self.queue_set.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
        info!("Dropped Renderer.")
    }
}

impl CapturedEvent for Renderer {
    /// When this event is captured, the swapchain is recreated, and regenerates all framebuffers from the swapchain images.
    fn on_resize(&mut self, _size : LogicalSize) {
        self.swapchain.as_mut().unwrap().borrow_mut().recreate(self.backend.get_surface_mut());
        self.framebuffers.as_mut().unwrap().clear();
        for image in self.swapchain.as_ref().unwrap().borrow().get_images() {
            self.framebuffers.as_mut().unwrap().push(Framebuffer::new(
                Rc::clone(&self.device.clone().unwrap()),
                &self.default_render_pass.as_ref().unwrap(),
                image,
                self.swapchain.as_ref().unwrap().borrow().get_swapchain_config().extent.to_extent()));
        }
    }
}

impl Renderer {
    /// Initializes the renderer for the specified window.
    pub fn new(window : &winit::Window) -> Self {
        info!("Initializing Renderer.");
        // Initialize backend.
        let mut backend = Backend::new(window);

        // Create device and all associated queues.
        let (device, mut queue_set) = Device::new(
            backend.get_primary_adapter(),
            backend.get_surface());

        // Create initial swapchain for rendering.
        let swapchain = Rc::new(RefCell::new(Swapchain::new(
            Rc::clone(&device),
            Rc::clone(queue_set.get_graphics_queue()),
            backend.get_surface_mut(),
            2).expect("Failed to create swapchain.")));

        let default_render_pass = RenderPass::new(
            Rc::clone(&device),
            false,
            1);

        let mut framebuffers = Vec::<Framebuffer<_>>::new();
        for image in swapchain.borrow().get_images() {
            framebuffers.push(Framebuffer::new(
                Rc::clone(&device),
                &default_render_pass,
                image,
                swapchain.borrow().get_swapchain_config().extent.to_extent()));
        }

        // Create pools for each queue.
        // TODO: Should this be a vector of pools for each frame to synchronize resources?
        let compute_pool = Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device),
            &mut queue_set.get_compute_queue().borrow_mut())));
        let mut graphics_pool = Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device),
            &mut queue_set.get_graphics_queue().borrow_mut())));
        let transfer_pool = Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device),
            &mut queue_set.get_transfer_queue().borrow_mut())));

        let graphics_buffer = CmdBuffer::new(
            Rc::clone(&device),
            Rc::clone(&graphics_pool));

        info!("Renderer has been initialized.");
        Self { backend,
            device: Some(device),
            queue_set : Some(queue_set),
            swapchain: Some(swapchain),
            compute_pool : Some(compute_pool),
            graphics_pool: Some(graphics_pool),
            graphics_buffer : Some(graphics_buffer),
            transfer_pool : Some(transfer_pool),
            default_render_pass: Some(default_render_pass),
            framebuffers : Some(framebuffers),
        }
    }

    pub fn build_cmd_buffers(&mut self) {
        self.graphics_buffer.as_mut().unwrap().begin_pass();
        self.graphics_buffer.as_mut().unwrap().end_pass();
    }

    pub fn begin_frame(&mut self) {
        let next_image = &self.swapchain.as_mut().unwrap().borrow_mut().get_next_image();
    }

    pub fn end_frame(&mut self) {
        &self.swapchain.as_mut().unwrap().borrow_mut().present();
    }
}
