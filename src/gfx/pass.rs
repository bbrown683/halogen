use std::cell::RefCell;
use std::rc::Rc;
use hal::{format, image, pass};
use hal::pass::Attachment;
use hal::pso::PipelineStage;
use hal::{Backend, Device as LogicalDevice};
use crate::gfx::Device;

pub struct RenderPass<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    attachments : Vec<Attachment>,
    render_pass : Option<B::RenderPass>
}

impl<B: Backend> Drop for RenderPass<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_render_pass(self.render_pass.take().unwrap());
        debug_assert!(self.render_pass.is_none());
    }
}

impl<B: Backend> RenderPass<B> {
    pub fn get_render_pass(&self) -> &Option<B::RenderPass> {
        &self.render_pass
    }

    pub fn get_attachments(self) -> Vec<Attachment> {
        self.attachments.clone()
    }
}

pub struct RenderPassBuilder<B: Backend> {
    device : Rc<RefCell<Device<B>>>,
    depth_stencil : bool,
    sample_count : u8
}

impl<B: Backend> RenderPassBuilder<B> {
    pub fn new(device : Rc<RefCell<Device<B>>>) -> Self {
        Self { device, depth_stencil: false, sample_count: 1 }
    }

    pub fn with_depth_stencil(mut self) -> Self {
        self.depth_stencil = true;
        self
    }

    pub fn with_samples(mut self, sample_count : u8) -> Self {
        self.sample_count = sample_count;
        self
    }

    pub fn build(self) -> RenderPass<B> {
        let mut attachments = Vec::<pass::Attachment>::new();
        attachments.push(pass::Attachment {
            format: Some(format::Format::Bgra8Srgb),
            samples: self.sample_count,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store,
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: image::Layout::Undefined..image::Layout::Present,
        });

        let depth_stencil_reference = {
            if self.depth_stencil {
                // Add attachments for depth stencil.
                attachments.push(pass::Attachment {
                    format: Some(format::Format::D32FloatS8Uint),
                    samples: self.sample_count,
                    ops: pass::AttachmentOps::new(
                        pass::AttachmentLoadOp::Clear,
                        pass::AttachmentStoreOp::Store,
                    ),
                    stencil_ops: pass::AttachmentOps::new(
                        pass::AttachmentLoadOp::Clear,
                        pass::AttachmentStoreOp::DontCare),
                    layouts: image::Layout::Undefined..image::Layout::DepthStencilAttachmentOptimal,
                });
                // Return the depth stencil attachment reference.
                Some(&(1, image::Layout::DepthStencilAttachmentOptimal))
            } else {
                None
            }
        };

        let subpass = pass::SubpassDesc {
            colors: &[(0, image::Layout::ColorAttachmentOptimal)],
            depth_stencil: depth_stencil_reference,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let dependency = pass::SubpassDependency {
            passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: image::Access::empty()
                ..(image::Access::COLOR_ATTACHMENT_READ | image::Access::COLOR_ATTACHMENT_WRITE),
        };

        let render_pass = Some(self.device
            .borrow()
            .get_logical_device()
            .create_render_pass(attachments.as_slice(), &[subpass], &[dependency])
            .expect("Can't create render pass"));
        RenderPass { device: self.device, attachments, render_pass }
    }
}