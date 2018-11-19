use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, RenderPass};

pub struct Framebuffer {
    device : Rc<RefCell<Device>>,
    framebuffer : vk::Framebuffer,
    color_view : vk::ImageView,
    depth_stencil_image : vk::Image,
    depth_stencil_view : vk::ImageView,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_image_view(self.depth_stencil_view, None);
            self.device.borrow().get_ash_device().destroy_image(self.depth_stencil_image, None);
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

        let depth_stencil_subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::DEPTH)
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

        let depth_stencil_image_info = vk::ImageCreateInfo::builder()
            .samples(vk::SampleCountFlags::TYPE_1)
            .format(vk::Format::D32_SFLOAT)
            .extent(vk::Extent3D { width: extent.width, height: extent.height, depth: 1 })
            .image_type(vk::ImageType::TYPE_2D)
            .array_layers(1)
            .mip_levels(1)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
            .build();

        let depth_stencil_image = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_image(&depth_stencil_image_info, None)
                .expect("Failed to create depth-stencil image.")
        };



        let depth_stencil_view_info = vk::ImageViewCreateInfo::builder()
            .format(vk::Format::D32_SFLOAT)
            .image(depth_stencil_image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .subresource_range(depth_stencil_subresource_range)
            .build();

        let depth_stencil_view = unsafe {
            device
                .borrow()
                .get_ash_device()
                .create_image_view(&depth_stencil_view_info, None)
                .unwrap()
        };

        let framebuffer_info = vk::FramebufferCreateInfo::builder()
            .layers(1)
            .width(extent.width)
            .height(extent.height)
            .render_pass(render_pass.get_render_pass_raw())
            .attachments(&[color_view, depth_stencil_view])
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
            depth_stencil_image,
            depth_stencil_view
        }
    }

    pub fn get_framebuffer_raw(&self) -> vk::Framebuffer {
        self.framebuffer
    }
}