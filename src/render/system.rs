use std::cell::RefCell;
use std::rc::Rc;
use crate::gfx::{GfxBackend, GfxBackendType, GfxDevice, GfxSwapchain};

pub struct RenderSystem {
    _backend : GfxBackend,
    device : Option<Rc<RefCell<GfxDevice<GfxBackendType>>>>,
    swapchain : Option<GfxSwapchain<GfxBackendType>>,
}

impl Drop for RenderSystem {
    fn drop(&mut self) {
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
        self.device.take();
        debug_assert!(self.device.is_none());
    }
}

impl RenderSystem {
    pub fn new(window : &winit::Window) -> Self {
        let mut backend = GfxBackend::new(window);
        let device = Some(Rc::new(RefCell::new(GfxDevice::new(
            backend.get_primary_adapter()
        ))));

        // Create initial swapchain for rendering.
        let swapchain = GfxSwapchain::new(
            Rc::clone(&device.clone().unwrap()),
            &mut backend.get_surface().unwrap(),
            2).ok();
        Self { _backend: backend, device, swapchain }
    }
}
