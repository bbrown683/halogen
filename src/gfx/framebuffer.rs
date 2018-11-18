use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::Device;

pub struct Framebuffer {
    device : Rc<RefCell<Device>>,
    framebuffer : vk::Framebuffer,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_framebuffer(self.framebuffer, None);
            info!("Dropped Framebuffer")
        }
    }
}

impl Framebuffer {
    pub fn new(device : Rc<RefCell<Device>>, image : &vk::Image) -> Self {
        unsafe {
            let framebuffer_info = vk::FramebufferCreateInfo::builder()
                .build();
            let framebuffer = device
                .borrow()
                .get_ash_device()
                .create_framebuffer(&framebuffer_info, None)
                .expect("Failed to create framebuffer");
            Self { device, framebuffer }
        }
    }
}