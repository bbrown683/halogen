use std::cell::RefCell;
use std::rc::Rc;
use hal::{Compute, Graphics, Transfer};
use winit::dpi::{LogicalPosition, LogicalSize};
use crate::gfx::{Backend, BackendType, CmdBuffer, CmdPool, Device, Queue, Swapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `GfxSystem` manages all render state.
pub struct Renderer<'a> {
    backend : Backend,
    device : Option<Rc<RefCell<Device<BackendType>>>>,
    compute_queue : Option<Rc<RefCell<Queue<BackendType, Compute>>>>,
    compute_pool : Option<Rc<RefCell<CmdPool<BackendType, Compute>>>>,
    graphics_queue : Option<Rc<RefCell<Queue<BackendType, Graphics>>>>,
    graphics_pool : Option<Rc<RefCell<CmdPool<BackendType, Graphics>>>>,
    graphics_buffers : Vec<CmdBuffer<'a, BackendType, Graphics>>,
    transfer_queue: Option<Rc<RefCell<Queue<BackendType, Transfer>>>>,
    transfer_pool : Option<Rc<RefCell<CmdPool<BackendType, Transfer>>>>,
    swapchain : Option<Swapchain<BackendType, Graphics>>,
}

impl<'a> Drop for Renderer<'a> {
    fn drop(&mut self) {
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.compute_pool.take();
        debug_assert!(self.compute_pool.is_none());
        self.compute_queue.take();
        debug_assert!(self.compute_queue.is_none());
        self.graphics_pool.take();
        debug_assert!(self.graphics_pool.is_none());
        self.graphics_queue.take();
        debug_assert!(self.graphics_queue.is_none());
        self.transfer_pool.take();
        debug_assert!(self.transfer_pool.is_none());
        self.transfer_queue.take();
        debug_assert!(self.transfer_queue.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
        info!("Dropped Renderer.")
    }
}

impl<'a> CapturedEvent for Renderer<'a> {
    fn on_resize(&mut self, size : LogicalSize) {
        self.swapchain.as_mut().unwrap().recreate(self.backend.get_surface());
    }
}

impl<'a> Renderer<'a> {
    /// Initializes the renderer for the specified window.
    pub fn new(window : &winit::Window) -> Self {
        info!("Initializing Renderer.");

        // Initialize backend.
        let mut backend = Backend::new(window);

        // Create device and all associated queues.
        let (device, mut compute_queue,
            mut graphics_queue, mut transfer_queue)
            = Device::new(backend.get_primary_adapter());

        // Create initial swapchain for rendering.
        let swapchain = Some(Swapchain::new(
            Rc::clone(&device.clone().unwrap()),
            Rc::clone(&graphics_queue.clone().unwrap()),
            backend.get_surface(),
            2).expect("Failed to create swapchain."));

        // Create pools for each queue.
        // TODO: Should this be a vector of pools for each frame to synchronize resources?
        let compute_pool = Some(Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device.clone().unwrap()),
            &mut compute_queue.as_ref().unwrap().borrow_mut()))));
        let graphics_pool = Some(Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device.clone().unwrap()),
            &mut graphics_queue.as_ref().unwrap().borrow_mut()))));
        let transfer_pool = Some(Rc::new(RefCell::new(CmdPool::new(
            Rc::clone(&device.clone().unwrap()),
            &mut transfer_queue.as_ref().unwrap().borrow_mut()))));

        let graphics_buffers = Vec::<CmdBuffer<'a, _, _>>::new();

        info!("Renderer has been initialized.");
        Self { backend, device, swapchain, compute_queue, compute_pool, graphics_queue, graphics_pool,
            graphics_buffers, transfer_queue, transfer_pool }
    }

    /*
    pub fn build_command_buffers(&mut self) {
        let mut graphics_pool = self.graphics_pool.as_mut().unwrap().borrow_mut();
        self.graphics_buffers.push(CmdBuffer::new(
            Rc::clone(&self.device.clone().unwrap()),
            &mut graphics_pool
        ));
    }
    */

    pub fn begin_frame(&mut self) {
        let next_image = &self.swapchain.as_mut().unwrap().get_next_image();
    }

    pub fn end_frame(&mut self) {
        &self.swapchain.as_mut().unwrap().present();
    }
}
