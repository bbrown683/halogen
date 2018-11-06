use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device};
use crate::gfx::{GfxDevice, GfxRenderPass};

pub struct GfxFramebuffer<B: Backend> {
    framebuffer : Option<B::Framebuffer>,
    frame_image : Option<B::Image>
}

impl<B: Backend> Drop for GfxFramebuffer<B> {
    fn drop(&mut self) {

    }
}

impl<B: Backend> GfxFramebuffer<B> {
    fn new(device : Rc<RefCell<GfxDevice<B>>>, render_pass : Rc<RefCell<GfxRenderPass<B>>>) {

    }
}