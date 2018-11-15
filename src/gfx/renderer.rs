use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use hal::{Compute, Graphics, Transfer};
use winit::dpi::{LogicalPosition, LogicalSize};
use crate::gfx::{Backend, BackendType, CmdBuffer, CmdPool, Device, Framebuffer, PoolSet,
                 Queue, QueueSet, RenderPass, Swapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `Renderer` manages all render state.
pub struct Renderer {
    backend : Backend,
    device : Option<Rc<RefCell<Device<BackendType>>>>,
    queue_set : Option<QueueSet<BackendType>>,
    pool_set : Option<PoolSet<BackendType>>,
    graphics_buffer : Option<Arc<Mutex<CmdBuffer<BackendType>>>>,
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
        self.graphics_buffer.take();
        debug_assert!(self.graphics_buffer.is_none());
        self.pool_set.take();
        debug_assert!(self.pool_set.is_none());
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
        let (device, queue_set) = Device::new(backend.get_primary_adapter());

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

        // Create list of framebuffers.
        let mut framebuffers = Vec::<Framebuffer<_>>::new();
        for image in swapchain.borrow().get_images() {
            framebuffers.push(Framebuffer::new(
                Rc::clone(&device),
                &default_render_pass,
                image,
                swapchain.borrow().get_swapchain_config().extent.to_extent()));
        }

        // Create pools for each queue. Store them in a set to reduce clutter.
        // TODO: Should this be a vector of pools for each frame to synchronize resources?
        let pool_set = PoolSet::new(
            CmdPool::new(Rc::clone(&device),
                         &mut queue_set.get_compute_queue().borrow_mut()),
            CmdPool::new(Rc::clone(&device),
                         &mut queue_set.get_graphics_queue().borrow_mut()),
            CmdPool::new(Rc::clone(&device),
                         &mut queue_set.get_transfer_queue().borrow_mut())
        );

        let graphics_buffer = Arc::new(Mutex::new(CmdBuffer::new(
            Rc::clone(&device),
            Rc::clone(&pool_set.get_graphics_pool()))));

        info!("Renderer has been initialized.");
        Self { backend,
            device: Some(device),
            queue_set: Some(queue_set),
            pool_set: Some(pool_set),
            swapchain: Some(swapchain),
            graphics_buffer: Some(graphics_buffer),
            default_render_pass: Some(default_render_pass),
            framebuffers: Some(framebuffers),
        }
    }

    pub fn build_cmd_buffers(&mut self) {
        // TODO: Use rayon to build cmd buffers in parallel.
//        self.graphics_buffer.as_mut().unwrap();
//        self.graphics_buffer.as_mut().unwrap();
    }

    pub fn begin_frame(&mut self) {
        let next_image = &self.swapchain.as_mut().unwrap().borrow_mut().get_next_image();
    }

    pub fn end_frame(&mut self) {
        &self.swapchain.as_mut().unwrap().borrow_mut().present();
    }
}
