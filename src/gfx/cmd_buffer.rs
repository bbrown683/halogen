use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use crate::gfx::{GfxCmdPool, GfxDevice};

pub struct GfxCmdBuffer<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    cmd_pool : Rc<RefCell<GfxCmdPool<B>>>
}

impl<B: Backend> GfxCmdBuffer<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>, cmd_pool : Rc<RefCell<GfxCmdPool<B>>>) -> Self {
        Self { device, cmd_pool }
    }

    pub fn begin() { unimplemented!() }

    pub fn end() { unimplemented!() }
}
