use std::cell::RefCell;
use std::rc::Rc;
use hal::pool;
use hal::{Backend, CommandPool, Device, Graphics, QueueFamily};
use crate::gfx::GfxDevice;

pub struct GfxCmdPool<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_pool : Option<B::CommandPool>
}

impl<B: Backend> Drop for GfxCmdPool<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_device().destroy_command_pool(self.cmd_pool.take().unwrap());
        debug_assert!(self.cmd_pool.is_none());
    }
}

impl<B: Backend> GfxCmdPool<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>) -> Self {
        let cmd_pool = Some(device
            .borrow()
            .get_device()
            .create_command_pool(device.borrow().get_graphics_queue_family().id(), pool::CommandPoolCreateFlags::RESET_INDIVIDUAL)
            .expect("Failed to create command pool"));
        Self { device, cmd_pool }
    }
}