use std::ptr;
use std::cell::RefCell;
use std::rc::Rc;
use ash::vk;
use ash::version::{DeviceV1_0};
use log::{info, error, debug, trace, warn};
use super::{CmdBuffer, Device};

pub struct Queue {
    device : Rc<RefCell<Device>>,
    queue : vk::Queue,
    family_index : u32,
    submit_semaphore : vk::Semaphore,
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().queue_wait_idle(self.queue).unwrap();
            self.device.borrow().get_ash_device().destroy_semaphore(self.submit_semaphore, None);
        }
        info!("Dropped Queue")
    }
}

impl Queue {
    pub fn new(device : Rc<RefCell<Device>>, family_index : u32) -> Self {
        let queue = unsafe {
            device
                .borrow()
                .get_ash_device()
                .get_device_queue(family_index, 0)
        };

        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let submit_semaphore = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_semaphore(&semaphore_info, None)
                .expect("Failed to create semaphore")
        };
        Self { device, queue, family_index, submit_semaphore }
    }

    /// Submits the command buffer to the queue for execution. The `submit_semaphore` will be signaled when this operation is complete.
    /// If the queue needs to wait for some work to be done, use `signal_semaphore`. If you need the CPU to wait for the queue to finish,
    /// i.e acquiring images on the swapchain, use `signal_fence`.
    pub fn submit(&self, cmd_buffer : &CmdBuffer, wait_semaphore : Option<vk::Semaphore>, signal_fence : Option<vk::Fence>) {
        let submit_info = match wait_semaphore {
            Some(semaphore) => vk::SubmitInfo::builder()
                .command_buffers(&[cmd_buffer.get_cmd_buffer_raw()])
                // Signal this semaphore when submission is completed.
                .signal_semaphores(&[self.submit_semaphore])
                // Wait on the image to be acquired before submitting the command buffer.
                .wait_semaphores(&[semaphore])
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .build(),
            None => vk::SubmitInfo::builder()
                .command_buffers(&[cmd_buffer.get_cmd_buffer_raw()])
                // Signal this semaphore when submission is completed.
                .signal_semaphores(&[self.submit_semaphore])
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .build()
        };
        unsafe {
            match signal_fence {
                Some(fence) => self.device
                    .borrow()
                    .get_ash_device()
                    // Submits to the queue with the specified semaphores and a acquire fence to signal.
                    .queue_submit(self.queue, &[submit_info], fence)
                    .expect("Failed to submit command buffer."),
                None => self.device
                    .borrow()
                    .get_ash_device()
                    // Submits to the queue with the specified semaphores and a acquire fence to signal.
                    .queue_submit(self.queue, &[submit_info], vk::Fence::null())
                    .expect("Failed to submit command buffer."),
            }
        }
    }

    pub fn get_queue_raw(&self) -> vk::Queue {
        self.queue
    }

    pub fn get_family_index(&self) -> u32 {
        self.family_index
    }

    /// Returns a handle to a semaphore which is signalled when submission to the queue is completed.
    pub fn get_submit_semaphore(&self) -> vk::Semaphore { self.submit_semaphore }
}