use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use ash::vk;
use ash::version::{DeviceV1_0};
use super::{CmdBuffer, Device};

pub struct Queue {
    device : Rc<RefCell<Device>>,
    queue : vk::Queue,
    family_index : u32,
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().queue_wait_idle(self.queue).unwrap();
            info!("Dropped Queue")
        }
    }
}

impl Queue {
    pub fn new(device : Rc<RefCell<Device>>, family_index : u32) -> Self {
        unsafe {
            let queue = device
                .borrow()
                .get_ash_device()
                .get_device_queue(family_index, 0);
            Self { device, queue, family_index }
        }
    }

    pub fn submit(&self, cmd_buffer : &CmdBuffer) {
        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&[cmd_buffer.get_cmd_buffer_raw()])
            .build();
        unsafe {
            self.device
                .borrow()
                .get_ash_device()
                .queue_submit(self.queue, &[submit_info], cmd_buffer.get_fence_raw())
                .expect("Failed to submit command buffer.");
            self.device
                .borrow()
                .get_ash_device()
                .wait_for_fences(&[cmd_buffer.get_fence_raw()], true, u64::max_value())
                .unwrap();
        }
    }

    pub fn get_queue_raw(&self) -> vk::Queue {
        self.queue
    }

    pub fn get_family_index(&self) -> u32 {
        self.family_index
    }
}