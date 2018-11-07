use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use hal::queue::{QueueFamily, Submission};
use crate::gfx::{GfxCmdBuffer, GfxDevice};

pub struct GfxQueue<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>
}

impl<B: Backend> GfxQueue<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>) -> Self {
        Self { device }
    }

    pub fn submit(cmd_buffer : Rc<RefCell<GfxCmdBuffer<B>>>) {

    }
}