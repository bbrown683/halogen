use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend};
use crate::gfx::{GfxDevice, GfxRenderPass};

pub struct GfxFramebuffer<B: Backend> {
    _device : Rc<RefCell<GfxDevice<B>>>,
    _render_pass : Rc<RefCell<GfxRenderPass<B>>>,
    _framebuffer : Option<B::Framebuffer>,
    _frame_image : Option<B::Image>
}

impl<B: Backend> Drop for GfxFramebuffer<B> {
    fn drop(&mut self) {

    }
}

impl<B: Backend> GfxFramebuffer<B> {
    fn new(device : Rc<RefCell<GfxDevice<B>>>, render_pass : Rc<RefCell<GfxRenderPass<B>>>) -> Self {
        Self { _device: device, _render_pass: render_pass, _framebuffer: None, _frame_image: None }
    }
}