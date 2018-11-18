use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, Queue};

pub struct CmdBuffer {
    device : Rc<RefCell<Device>>,
    cmd_buffer : vk::CommandBuffer,
}

impl Drop for CmdBuffer {
    fn drop(&mut self) {

    }
}

/// A recorder for graphics, compute, or transfer operations.
impl CmdBuffer {
    pub fn new(device : Rc<RefCell<Device>>, cmd_pool : &CmdPool) -> Self {
        unsafe {
            let cmd_buffer_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(cmd_pool.get_cmd_pool_ptr())
                .command_buffer_count(1)
                .level(vk::CommandBufferLevel::PRIMARY)
                .build();
            let cmd_buffer = device
                .borrow()
                .get_ash_device()
                .allocate_command_buffers(&cmd_buffer_info)
                .expect("Failed to create command buffer")
                .remove(0);
            Self { device, cmd_buffer }
        }
    }
}

/// Allocates the command buffers into memory for reuse.
pub struct CmdPool {
    device : Rc<RefCell<Device>>,
    cmd_pool : vk::CommandPool,
}

impl Drop for CmdPool {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_command_pool(self.cmd_pool, None);
            info!("Dropped CmdPool")
        }
    }
}

impl CmdPool {
    pub fn new(device : Rc<RefCell<Device>>, queue : &Queue) -> Self {
        unsafe {
            let cmd_pool_info = vk::CommandPoolCreateInfo::builder()
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                .queue_family_index(queue.get_family_index())
                .build();

            let cmd_pool = device
                .borrow()
                .get_ash_device()
                .create_command_pool(&cmd_pool_info, None)
                .expect("Failed to create command pool");
            Self { device, cmd_pool }
        }
    }

    pub fn reset(&self) {
        unsafe {
            self.device
                .borrow()
                .get_ash_device()
                .reset_command_pool(
                    self.cmd_pool,
                    vk::CommandPoolResetFlags::RELEASE_RESOURCES)
                .unwrap();
        }
    }

    pub fn get_cmd_pool_ptr(&self) -> vk::CommandPool {
        self.cmd_pool
    }
}