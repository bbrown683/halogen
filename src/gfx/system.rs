use std::cell::RefCell;
use std::rc::Rc;
use hal::{Compute, Graphics, Transfer};
use winit::dpi::{LogicalPosition, LogicalSize};
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxQueue, GfxSwapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `GfxSystem` manages all render state.
pub struct GfxSystem {
    backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    compute_queue : Option<Rc<RefCell<GfxQueue<GfxBackendType, Compute>>>>,
    graphics_queue : Option<Rc<RefCell<GfxQueue<GfxBackendType, Graphics>>>>,
    transfer_queue: Option<Rc<RefCell<GfxQueue<GfxBackendType, Transfer>>>>,
    swapchain : Option<GfxSwapchain<GfxBackendType, Graphics>>,
}

impl Drop for GfxSystem {
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

impl CapturedEvent for GfxSystem {
    fn on_resize(&mut self, size : LogicalSize) {
        println!("Swapchain was resized to {:?}", &size);
    }

    fn on_cursor_move(&mut self, position : LogicalPosition) {
        unimplemented!()
    }
}

impl GfxSystem {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = GfxBackend::new(window);
        let (device, compute_queue,
            graphics_queue, transfer_queue)
            = GfxDevice::new(backend.get_primary_adapter());

        // Create initial swapchain for rendering.
        let swapchain = Some(GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            Rc::clone(&graphics_queue.clone().unwrap()),
            backend.get_surface(),
            2).expect("Failed to create swapchain."));
        Self { backend, device, swapchain, compute_queue,
            graphics_queue, transfer_queue }
    }

    pub fn begin_frame(&self) { }

    pub fn end_frame(&self) { }
}
