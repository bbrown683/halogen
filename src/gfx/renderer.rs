use std::cell::RefCell;
use std::rc::Rc;
use hal::{Compute, Graphics, Transfer};
use winit::dpi::{LogicalPosition, LogicalSize};
use crate::gfx::{Backend, BackendType, Device, Queue, Swapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `GfxSystem` manages all render state.
pub struct Renderer {
    backend : Backend,
    device : Option<Rc<RefCell<Device<BackendType>>>>,
    compute_queue : Option<Rc<RefCell<Queue<BackendType, Compute>>>>,
    graphics_queue : Option<Rc<RefCell<Queue<BackendType, Graphics>>>>,
    transfer_queue: Option<Rc<RefCell<Queue<BackendType, Transfer>>>>,
    swapchain : Option<Swapchain<BackendType, Graphics>>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
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
    }
}

impl CapturedEvent for Renderer {
    fn on_resize(&mut self, size : LogicalSize) {
        self.swapchain.as_mut().unwrap().recreate(self.backend.get_surface());
    }
}

impl Renderer {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = Backend::new(window);
        let (device, compute_queue,
            graphics_queue, transfer_queue)
            = Device::new(backend.get_primary_adapter());

        // Create initial swapchain for rendering.
        let swapchain = Some(Swapchain::new(
            Rc::clone(&device.clone().unwrap()),
            Rc::clone(&graphics_queue.clone().unwrap()),
            backend.get_surface(),
            2).expect("Failed to create swapchain."));
        Self { backend, device, swapchain, compute_queue,
            graphics_queue, transfer_queue }
    }

    pub fn begin_frame(&self) {

    }

    pub fn end_frame(&mut self) {
        &self.swapchain.as_mut().unwrap().present();
    }
}
