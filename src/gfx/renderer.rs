use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use winit::dpi::{LogicalPosition, LogicalSize};
use super::instance::Instance;
use crate::util::CapturedEvent;



/// The highest level of the gfx module, the `Renderer` manages all render state.
pub struct Renderer {
    instance : Option<Instance>
}

impl Drop for Renderer {
    fn drop(&mut self) {
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
            let instance = Instance::new(window);
            info!("Renderer has been initialized.");
            Self { instance: Some(instance) }
        }
    }

    pub fn begin_frame(&mut self) {
    }

    pub fn end_frame(&mut self) {

    }
}
