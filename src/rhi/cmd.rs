use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, Framebuffer, Queue, RenderPass};

pub struct CmdState {
    format : vk::Format,
    extent : vk::Extent2D,
}

pub struct CmdBuffer {
    device : Rc<RefCell<Device>>,
    cmd_pool : Rc<RefCell<CmdPool>>,
    cmd_buffer : vk::CommandBuffer,
}

impl Drop for CmdBuffer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .borrow()
                .get_ash_device()
                .free_command_buffers(
                    self.cmd_pool.borrow().get_cmd_pool_raw(),
                    &[self.cmd_buffer]);
        }
    }
}

/// A recorder for graphics, compute, or transfer operations.
impl CmdBuffer {
    pub fn new(device : Rc<RefCell<Device>>,
               cmd_pool : Rc<RefCell<CmdPool>>) -> Self {
        let cmd_buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(cmd_pool.borrow().get_cmd_pool_raw())
            .command_buffer_count(1)
            .level(vk::CommandBufferLevel::PRIMARY)
            .build();

        let cmd_buffer = unsafe {
            device
                .borrow()
                .get_ash_device()
                .allocate_command_buffers(&cmd_buffer_info)
                .expect("Failed to create command buffer")
                .remove(0)
        };

        Self { device, cmd_pool, cmd_buffer }
    }

    pub fn begin_pass(&self, state : CmdState, render_pass : &RenderPass, framebuffer : &Framebuffer) {
        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue { float32: [0.39, 0.58, 0.93, 1.0] }
        };

        let begin_pass_info = vk::RenderPassBeginInfo::builder()
            .clear_values(&[clear_value])
            .framebuffer(framebuffer.get_framebuffer_raw())
            .render_pass(render_pass.get_render_pass_raw())
            .render_area(vk::Rect2D::builder()
                .extent(state.extent)
                .build())
            .build();

        unsafe {
            self.device.borrow().get_ash_device().cmd_begin_render_pass(
                self.cmd_buffer,
                &begin_pass_info,
                vk::SubpassContents::INLINE)
        }
    }

    pub fn end_pass(&self) {

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
    pub fn new(device : Rc<RefCell<Device>>,
               queue : &Queue) -> Self {
        let cmd_pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue.get_family_index())
            .build();

        let cmd_pool = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_command_pool(&cmd_pool_info, None)
                .expect("Failed to create command pool")
        };

        Self { device, cmd_pool }
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

    pub fn get_cmd_pool_raw(&self) -> vk::CommandPool {
        self.cmd_pool
    }
}