use std::{cell::RefCell, rc::Rc};
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, RenderPass};

/// A framebuffer manages an image created by the swapchain.
pub struct Framebuffer {
    device : Rc<RefCell<Device>>,
    framebuffer : vk::Framebuffer,
    color_view : vk::ImageView,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().ash_device().device_wait_idle().unwrap();
            self.device.borrow().ash_device().destroy_framebuffer(self.framebuffer, None);
            self.device.borrow().ash_device().destroy_image_view(self.color_view, None);
        }
        info!("Dropped Framebuffer")
    }
}

impl Framebuffer {
    pub fn framebuffer_raw(&self) -> vk::Framebuffer {
        self.framebuffer
    }
}

pub struct FramebufferBuilder {
    device : Rc<RefCell<Device>>,
    render_pass : Rc<RefCell<RenderPass>>,
    extent : vk::Extent2D,
    color_view : vk::ImageView,
}

impl FramebufferBuilder {
    pub fn new(device : Rc<RefCell<Device>>,
               render_pass : Rc<RefCell<RenderPass>>,
               color_image : vk::Image,
               color_format : vk::Format,
               extent : vk::Extent2D) -> Self {
        let color_subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);

        let color_view_info = vk::ImageViewCreateInfo::builder()
            .format(color_format)
            .image(color_image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .subresource_range(color_subresource_range.build());
        let color_view = unsafe {
            device
                .borrow()
                .ash_device()
                .create_image_view(&color_view_info, None)
                .unwrap()
        };
        Self { device,
            render_pass,
            extent,
            color_view
        }
    }

    pub fn add_depth_stencil(self) -> Self {
        self
    }

    pub fn build(self) -> Framebuffer {
        let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .layers(1)
            .width(self.extent.width)
            .height(self.extent.height)
            .render_pass(self.render_pass.borrow().render_pass_raw())
            .attachments(&[self.color_view])
            .build();
        let framebuffer = unsafe {
            self.device
                .borrow()
                .ash_device()
                .create_framebuffer(&framebuffer_info, None)
                .expect("Failed to create framebuffer")
        };
        Framebuffer { device: Rc::clone(&self.device), framebuffer, color_view: self.color_view }
    }
}