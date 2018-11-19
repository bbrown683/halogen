use std::cell::RefCell;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::Device;

pub struct RenderPass {
    device : Rc<RefCell<Device>>,
    render_pass : vk::RenderPass,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_render_pass(self.render_pass, None);
        }
    }
}

impl RenderPass {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        let color_attachment = vk::AttachmentDescription::builder()
            .format(vk::Format::B8G8R8A8_SRGB)
            .samples(vk::SampleCountFlags::TYPE_1)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .build();

        let color_reference = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();

        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&[color_reference])
            .build();

        let dependency = vk::SubpassDependency::builder()
            .dst_subpass(0)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_READ |
                vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .build();

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&[color_attachment])
            .subpasses(&[subpass])
            .dependencies(&[dependency])
            .build();

        let render_pass = unsafe {
             device
                 .borrow()
                 .get_ash_device()
                 .create_render_pass(&render_pass_info, None)
                 .expect("Failed to create render pass")
        };
        Self { device, render_pass }
    }

    pub fn get_render_pass_raw(&self) -> vk::RenderPass {
        self.render_pass
    }
}
