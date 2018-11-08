use std::cell::RefCell;
use std::rc::Rc;
use hal::{Compute, Graphics, Transfer};
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxQueue, GfxSwapchain};

pub struct GfxRenderSystem {
    _backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    compute_queue : Option<GfxQueue<GfxBackendType, Compute>>,
    graphics_queue : Option<GfxQueue<GfxBackendType, Graphics>>,
    transfer_queue: Option<GfxQueue<GfxBackendType, Transfer>>,
    swapchain : Option<GfxSwapchain<GfxBackendType>>,
}

impl Drop for GfxRenderSystem {
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

impl GfxRenderSystem {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = GfxBackend::new(window);
        let (device,
            compute_queue,
            graphics_queue,
            transfer_queue) = GfxDevice::new(
            backend.get_primary_adapter()
        );

        // Create initial swapchain for rendering.
        let swapchain = GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            graphics_queue.as_ref().unwrap(),
            backend.get_surface(),
            2).ok();
        Self { _backend: backend, device, swapchain, compute_queue,
            graphics_queue, transfer_queue }
    }
}
