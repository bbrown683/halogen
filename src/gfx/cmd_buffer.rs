use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, Device as LogicalDevice};
use hal::command::{CommandBuffer, Primary, MultiShot, Submittable};
use crate::gfx::{CmdPool, Device};

pub struct CmdBuffer<'a, B: Backend, C: Capability> {
    device : Rc<RefCell<Device<B>>>,
    cmd_buffer : Option<CommandBuffer<'a, B, C, MultiShot, Primary>>,
    fence : Option<B::Fence>,
}

impl<'a, B: Backend, C: Capability> Drop for CmdBuffer<'a, B, C> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_fence(self.fence.take().unwrap());
    }
}

impl<'a, B: Backend, C: 'static> CmdBuffer<'a, B, C> where C: Capability {
    pub fn new(device : Rc<RefCell<Device<B>>>, cmd_pool : &'a mut CmdPool<B, C>) -> Self {
            let cmd_buffer = Some(cmd_pool.get_cmd_pool().acquire_command_buffer(false));
            let fence = Some(device.borrow().get_logical_device().create_fence(true).expect("failed to create fence."));
        Self { device, cmd_buffer, fence }
    }

    pub fn begin_pass() { unimplemented!() }

    pub fn end_pass() { unimplemented!() }

    pub fn get_fence(&self) -> &Option<B::Fence> {
        &self.fence
    }

    /*
    pub fn get_submittable(&self) -> Submittable<B, C, Primary> {
        unimplemented!()
    }
    */
}
