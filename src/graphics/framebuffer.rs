use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, RenderPass};

/// A framebuffer manages an image created by the swapchain.
// TODO: Create builder for this object due to somewhat complicated state.
pub struct Framebuffer {
    device : Rc<RefCell<Device>>,
    framebuffer : vk::Framebuffer,
    color_view : vk::ImageView,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_framebuffer(self.framebuffer, None);
            self.device.borrow().get_ash_device().destroy_image_view(self.color_view, None);
            info!("Dropped Framebuffer")
        }
    }
}

impl Framebuffer {
    pub fn new(device : Rc<RefCell<Device>>,
               render_pass : &RenderPass,
               color_image : vk::Image,
               extent : vk::Extent2D) -> Self {
        let color_subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1)
            .build();

        let color_view_info = vk::ImageViewCreateInfo::builder()
            .format(vk::Format::B8G8R8A8_SRGB)
            .image(color_image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .subresource_range(color_subresource_range)
            .build();

        let color_view = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_image_view(&color_view_info, None)
                .unwrap()
        };

        let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .layers(1)
            .width(extent.width)
            .height(extent.height)
            .render_pass(render_pass.get_render_pass_raw())
            .attachments(&[color_view])
            .build();

        let framebuffer = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_framebuffer(&framebuffer_info, None)
                .expect("Failed to create framebuffer")
        };

        Self { device,
            framebuffer,
            color_view,
        }
    }

    pub fn get_framebuffer_raw(&self) -> vk::Framebuffer {
        self.framebuffer
    }
}