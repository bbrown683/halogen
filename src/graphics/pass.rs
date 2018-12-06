use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use log::{info, error, debug, trace, warn};
use super::Device;

/// Represents how the begin to end state for rendering should occur.
// TODO: Create builder for this object due to somewhat complicated state.
pub struct RenderPass {
    device : Rc<RefCell<Device>>,
    render_pass : vk::RenderPass,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().device_wait_idle().unwrap();
            self.device.borrow().get_ash_device().destroy_render_pass(self.render_pass, None);
        }
        info!("Dropped RenderPass")
    }
}

impl RenderPass {
    pub fn get_render_pass_raw(&self) -> vk::RenderPass {
        self.render_pass
    }
}

pub struct RenderPassBuilder {
    device : Rc<RefCell<Device>>,
    color_attachments : Vec<vk::AttachmentDescription>,
    color_references : Vec<vk::AttachmentReference>,
    depth_stencil_attachment : Option<vk::AttachmentDescription>,
}

impl RenderPassBuilder {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        Self { device,
            color_attachments: Vec::new(),
            color_references: Vec::new(),
            depth_stencil_attachment: None }
    }

    pub fn add_color_attachment(mut self, format : vk::Format) -> Self {
        self.color_attachments.push(vk::AttachmentDescription::builder()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .build());
        self.color_references.push(vk::AttachmentReference::builder()
            .attachment((self.color_attachments.len() - 1) as u32)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build());
        self
    }

    /// Adds a depth stencil attachment to the renderpass. There can only be a single depth-stencil attachment.
    pub fn add_depth_attachment(mut self, format : vk::Format) -> Self {
        self.depth_stencil_attachment = Some(vk::AttachmentDescription::builder()
            .format(format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .build());
        self
    }

    pub fn build(self) -> RenderPass {
        let subpass = if self.depth_stencil_attachment.is_some(){
            let depth_stencil_reference = vk::AttachmentReference::builder()
                .attachment(self.color_attachments.len() as u32)
                .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
            vk::SubpassDescription::builder()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(self.color_references.as_slice())
                .depth_stencil_attachment(&depth_stencil_reference)
                .build()
        } else {
            vk::SubpassDescription::builder()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(self.color_references.as_slice())
                .build()
        };

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(self.color_attachments.as_slice())
            .subpasses(&[subpass])
            .build();
        let render_pass = unsafe {
            self.device
                .borrow()
                .get_ash_device()
                .create_render_pass(&render_pass_info, None)
                .expect("Failed to create render pass")
        };
        RenderPass { device: Rc::clone(&self.device), render_pass }
    }
}