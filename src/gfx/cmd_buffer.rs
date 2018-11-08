use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Capability, CommandPool, Device};
use hal::command::CommandBuffer;
use crate::gfx::{GfxCmdPool, GfxDevice};

pub struct GfxCmdBuffer<B: Backend, C: Capability> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_pool : Rc<RefCell<GfxCmdPool<B, C>>>,
    cmd_buffer : Option<B::CommandBuffer>,
    fence : Option<B::Fence>,
}

/*
impl<B: Backend, C: Capability> GfxCmdBuffer<B, C> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, cmd_pool : Rc<RefCell<GfxCmdPool<B, C>>>) -> Self {
        let cmd_buffer = Some(cmd_pool.borrow_mut().get_cmd_pool().allocate(1, RawLevel::Primary).remove(0));
        let fence = Some(device.borrow().get_device().create_fence(true).expect("failed to create fence."));
        Self { device, cmd_pool, cmd_buffer : None, fence }
    }

    pub fn begin() { unimplemented!() }

    pub fn end() { unimplemented!() }
}
*/