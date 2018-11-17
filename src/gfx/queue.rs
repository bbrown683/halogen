use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use ash::vk;
use ash::version::{DeviceV1_0};
use super::Device;

pub struct Queue {
    device : Rc<RefCell<Device>>,
    queue : vk::Queue,
    queue_family_index : u32,
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().queue_wait_idle(self.queue).unwrap();
        }
    }
}

impl Queue {
    pub fn new(device : Rc<RefCell<Device>>, queue_family_index : u32) -> Self {
        unsafe {
            let queue = device
                .borrow()
                .get_ash_device()
                .get_device_queue(queue_family_index, 0);
            Self { device, queue, queue_family_index }
        }
    }
}