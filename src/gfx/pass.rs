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
    render_pass : Option<B::RenderPass>,
    depth_stencil : bool,
    samples : u8
}

impl<B: Backend> Drop for RenderPass<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_logical_device().destroy_render_pass(self.render_pass.take().unwrap());
        debug_assert!(self.render_pass.is_none());
    }
}

impl<B: Backend> RenderPass<B> {
    pub fn new(device : Rc<RefCell<Device<B>>>,
               depth_stencil : bool,
               samples : u8) -> Self {
        let mut attachments = Vec::<pass::Attachment>::new();
        attachments.push(pass::Attachment {
            format: Some(format::Format::Bgra8Srgb),
            samples,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store,
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: image::Layout::Undefined..image::Layout::Present,
        });

        /*
        let depth_stencil_reference = {
            if depth_stencil {
                // Add attachments for depth stencil.
                attachments.push(pass::Attachment {
                    format: Some(format::Format::D32FloatS8Uint),
                    samples,
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
        */

        let subpass = pass::SubpassDesc {
            colors: &[(0, image::Layout::ColorAttachmentOptimal)],
//            depth_stencil: depth_stencil_reference,
            depth_stencil: None,
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

        let render_pass = Some(device
            .borrow()
            .get_logical_device()
            .create_render_pass(attachments.as_slice(), &[subpass], &[dependency])
            .expect("Can't create render pass"));
        RenderPass { device, attachments, render_pass, depth_stencil, samples }
    }

    pub fn get_render_pass(&self) -> &Option<B::RenderPass> {
        &self.render_pass
    }

    pub fn get_attachments(self) -> Vec<Attachment> {
        self.attachments.clone()
    }
}