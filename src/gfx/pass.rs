use std::cell::RefCell;
use std::rc::Rc;
use hal::{format, image, pass};
use hal::pso::PipelineStage;
use hal::{Backend, Device};
use crate::gfx::{GfxDevice};

pub struct GfxRenderPass<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    render_pass : Option<B::RenderPass>
}

impl<B: Backend> Drop for GfxRenderPass<B> {
    fn drop(&mut self) {
        &self.device.borrow().get_device().destroy_render_pass(self.render_pass.take().unwrap());
    }
}

impl<B: Backend> GfxRenderPass<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>) -> Self {
        let attachment = pass::Attachment {
            format: Some(format::Format::Rgba8Unorm),
            samples: 1,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store,
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: image::Layout::Undefined..image::Layout::Present,
        };

        let subpass = pass::SubpassDesc {
            colors: &[(0, image::Layout::ColorAttachmentOptimal)],
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
            .get_device()
            .create_render_pass(&[attachment], &[subpass], &[dependency])
            .expect("Can't create render pass"));
        Self { device, render_pass }
    }
}

pub struct RenderPassBuilder {}