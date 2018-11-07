use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use hal::command::{RawCommandBuffer, RawLevel};
use hal::pool::RawCommandPool;
use crate::gfx::{GfxCmdPool, GfxDevice};

pub struct GfxCmdBuffer<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_pool : Rc<RefCell<GfxCmdPool<B>>>,
    cmd_buffer : Option<B::CommandBuffer>,
    fence : Option<B::Fence>,
}

impl<B: Backend> GfxCmdBuffer<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, mut cmd_pool : Rc<RefCell<GfxCmdPool<B>>>) -> Self {
        Self { device, cmd_pool, cmd_buffer : None, fence: None }
    }

    pub fn begin() { unimplemented!() }

    pub fn end() { unimplemented!() }
}
