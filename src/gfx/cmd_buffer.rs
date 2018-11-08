use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, CommandPool, Device};
use hal::command::{CommandBuffer, Primary, MultiShot};
use crate::gfx::{GfxCmdPool, GfxDevice};

pub struct GfxCmdBuffer<'a, B: Backend, C: Capability> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_buffer : Option<CommandBuffer<'a, B, C, MultiShot, Primary>>,
    fence : Option<B::Fence>,
}

impl<'a, B: Backend, C: Capability> Drop for GfxCmdBuffer<'a, B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_device().destroy_fence(self.fence.take().unwrap());
    }
}

impl<'a, B: Backend, C: 'static> GfxCmdBuffer<'a, B, C> where C: Capability {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, mut cmd_pool : &'a mut GfxCmdPool<B, C>) -> Self {
            let cmd_buffer = Some(cmd_pool.get_cmd_pool().acquire_command_buffer(false));
            let fence = Some(device.borrow().get_device().create_fence(true).expect("failed to create fence."));
        Self { device, cmd_buffer, fence }
    }

    pub fn begin() { unimplemented!() }

    pub fn end() { unimplemented!() }
}
