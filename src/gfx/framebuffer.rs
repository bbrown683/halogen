use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device as DeviceTrait};
use hal::pso::Viewport;
use crate::gfx::{Device, RenderPass};

pub struct Framebuffer<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    render_pass : Rc<RefCell<RenderPass<B>>>,
    framebuffer : Option<B::Framebuffer>,
    frame_image : Option<B::Image>,
    viewport : Viewport,
}

impl<B: Backend> Drop for Framebuffer<B> {
    fn drop(&mut self) {

    }
}

impl<B: Backend> Framebuffer<B> {
    /*
    fn new(device : Rc<RefCell<Device<B>>>, render_pass : Rc<RefCell<RenderPass<B>>>) -> Self {
        let framebuffer = device
            .borrow()
            .get_logical_device()
            .create_framebuffer(render_pass.borrow().get_render_pass().as_ref().unwrap(),
                render_pass.borrow().get_attachments());
        Self { device: device, render_pass: render_pass, framebuffer: None, frame_image: None }
    }
    */
}

pub struct FramebufferBuilder {

}

impl FramebufferBuilder {

}