use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device as DeviceTrait};
use hal::image::Extent;
use hal::pso::Viewport;
use crate::gfx::{Device, RenderPass};

pub struct Framebuffer<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    render_pass : Rc<RefCell<RenderPass<B>>>,
    framebuffer : Option<B::Framebuffer>,
    frame_image : Option<B::Image>,
}

impl<B: Backend> Drop for Framebuffer<B> {
    fn drop(&mut self) {
        println!("Dropped Framebuffer")
    }
}

impl<B: Backend> Framebuffer<B> {
    /*
    fn new(device : Rc<RefCell<Device<B>>>, render_pass : Rc<RefCell<RenderPass<B>>>, ) -> Self {
        let framebuffer = device
            .borrow()
            .get_logical_device()
            .create_framebuffer(render_pass.borrow().get_render_pass().as_ref().unwrap(),
                render_pass.borrow().get_attachments(), extent);
        Self { device, render_pass, framebuffer }
    }
    */
}

pub struct FramebufferBuilder {

}

impl FramebufferBuilder {
    pub fn with_extent(extent : Extent) {

    }
}