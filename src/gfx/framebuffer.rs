use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device as DeviceTrait};
use hal::format::{Aspects, Format, Swizzle};
use hal::image::{Extent, SubresourceRange, ViewKind};
use crate::gfx::{Device, RenderPass};

pub struct Framebuffer<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    framebuffer : Option<B::Framebuffer>,
    image_view : Option<B::ImageView>,
}

impl<B: Backend> Drop for Framebuffer<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_image_view(self.image_view.take().unwrap());
        debug_assert!(self.image_view.is_none());
        &self.device.borrow().get_logical_device().destroy_framebuffer(self.framebuffer.take().unwrap());
        debug_assert!(self.framebuffer.is_none());
        info!("Dropped Framebuffer")
    }
}

impl<B: Backend> Framebuffer<B> {
    pub fn new(device : Rc<RefCell<Device<B>>>, render_pass : &RenderPass<B>,
           image : &B::Image, extent : Extent) -> Self {
        // TODO: Aspects needs to know if depth or stencil is enabled.
        // Gives validation warning if Aspects::all() is specified on a color image.
        let resources = SubresourceRange {
            aspects: Aspects::COLOR,
            levels: 0..1,
            layers: 0..1
        };

        let mut image_view = vec![device
                .borrow()
                .get_logical_device()
                .create_image_view(&image,
                                   ViewKind::D2,
                                   Format::Bgra8Srgb,
                                   Swizzle::NO,
                                   resources.clone())
                .expect("Failed to create image view.")];
        let framebuffer = Some(device
            .borrow()
            .get_logical_device()
            .create_framebuffer(render_pass.get_render_pass().as_ref().unwrap(),
                                image_view.as_slice(),
                                extent).expect("Failed to create Framebuffer"));
        Self { device, framebuffer, image_view: Some(image_view.remove(0)) }
    }
}