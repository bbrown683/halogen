use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use winit::dpi::{LogicalPosition, LogicalSize};
use super::{Device, Instance, Swapchain};
use crate::util::CapturedEvent;

/// The highest level of the gfx module, the `Renderer` manages all render state.
pub struct Renderer {
    instance : Option<Instance>,
    device : Option<Device>,
    swapchain : Option<Swapchain>,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.swapchain.take();
        debug_assert!(self.swapchain.is_none());
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
            let device = Device::new(&instance)
                .ok()
                .unwrap();
            let swapchain = Swapchain::new(&instance, &device, window)
                .ok()
                .unwrap();
            info!("Renderer has been initialized.");
            Self { instance: Some(instance),
                device: Some(device),
                swapchain: Some(swapchain)
            }
        }
    }

    pub fn begin_frame(&mut self) {
    }

    pub fn end_frame(&mut self) {

    }
}
