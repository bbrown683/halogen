use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, Device as DeviceTrait};
use hal::format::{Aspects, Format, Swizzle};
use hal::image::{Extent, SubresourceRange, ViewKind};
use hal::pso::Viewport;
use crate::gfx::{Device, RenderPass};

pub struct Framebuffer<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    render_pass : Rc<RefCell<RenderPass<B>>>,
    framebuffer : Option<B::Framebuffer>,
    image_views : Option<Vec<B::ImageView>>,
}

impl<B: Backend> Drop for Framebuffer<B> {
    fn drop(&mut self) {

        println!("Dropped Framebuffer")
    }
}

impl<B: Backend> Framebuffer<B> {
    fn new(device : Rc<RefCell<Device<B>>>, render_pass : Rc<RefCell<RenderPass<B>>>,
           images : &Vec<B::Image>, extent : Extent) -> Self {
        let resources = SubresourceRange {
            aspects: Aspects::all(),
            levels: 0..1,
            layers: 0..1
        };

        let mut image_views = Vec::<B::ImageView>::new();
        for i in 0..images.len() {
            image_views.push(device
                .borrow()
                .get_logical_device()
                .create_image_view(images.get(i).unwrap(), ViewKind::D2, Format::Bgra8Srgb,
                                   Swizzle::NO, resources.clone())
                .expect("Failed to create image view."));
        }
        let framebuffer = Some(device
            .borrow()
            .get_logical_device()
            .create_framebuffer(render_pass.borrow().get_render_pass().as_ref().unwrap(),
                                image_views.as_slice(),
                                extent).expect("Failed to create Framebuffer"));
        Self { device, render_pass, framebuffer, image_views: Some(image_views) }
    }
}