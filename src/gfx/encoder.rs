use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, CommandPool, Device, DescriptorPool, Graphics, Primitive};
use hal::{format, image};
use hal::command::{CommandBuffer, MultiShot, Primary};
use hal::pass::{self, Subpass};
use hal::pso::{self, PipelineStage, ShaderStageFlags};
use hal::queue::Submission;
use crate::gfx::{GfxDevice, GfxSwapchain, GfxSync, Vertex};

// Encodes render state to a command buffer from a command pool.
pub struct GfxEncoder<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    sync : Rc<RefCell<GfxSync<B>>>,
    swapchain : Rc<RefCell<GfxSwapchain<B>>>,
    set_layout : Option<B::DescriptorSetLayout>,
    pipeline : Option<B::GraphicsPipeline>,
    pipeline_layout : Option<B::PipelineLayout>,
    render_pass : Option<B::RenderPass>,
    viewport : pso::Viewport,
}

impl<B: Backend> Drop for GfxEncoder<B> {
    fn drop(&mut self) {
        &self.device.borrow().logical_device.destroy_descriptor_set_layout(self.set_layout.take().unwrap());
        debug_assert!(self.set_layout.is_none());
        &self.device.borrow().logical_device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
        debug_assert!(self.pipeline.is_none());
        &self.device.borrow().logical_device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
        debug_assert!(self.pipeline_layout.is_none());
        &self.device.borrow().logical_device.destroy_render_pass(self.render_pass.take().unwrap());
        debug_assert!(self.render_pass.is_none());
    }
}

impl<B: Backend> GfxEncoder<B> {
    pub fn get_submission(self) {

    }
}

pub struct EncoderBuilder<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    sync : Rc<RefCell<GfxSync<B>>>,
    swapchain : Rc<RefCell<GfxSwapchain<B>>>,
    vertex_module : Option<B::ShaderModule>,
    fragment_module : Option<B::ShaderModule>,
    vertices : Option<Vec<Vertex>>,
}

impl<B: Backend> EncoderBuilder<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               sync : Rc<RefCell<GfxSync<B>>>,
               swapchain : Rc<RefCell<GfxSwapchain<B>>>) -> Self {
        Self { device, sync, swapchain, vertex_module: None, fragment_module: None,
        vertices : None }
    }

    pub fn with_vertex_shader(mut self, bytes : Vec<u8>) -> Self {
        self.vertex_module = Some(self.device.borrow()
            .logical_device
            .create_shader_module(&bytes)
            .expect("Failed to create shader module."));
        self
    }

    pub fn with_fragment_shader(mut self, bytes : Vec<u8>) -> Self {
        self.fragment_module = Some(self.device.borrow()
            .logical_device
            .create_shader_module(&bytes)
            .expect("Failed to create shader module."));
        self
    }

    pub fn with_vertices(mut self, vertices : Vec<Vertex>) -> Self {
        self.vertices = Some(vertices);
        self
    }

    /// Builds the GfxEncoder object with the required state necessary to begin using it.
    pub fn build(self) -> GfxEncoder<B> {
        // Setup renderpass and pipeline
        let set_layout = self.device.borrow().logical_device
            .create_descriptor_set_layout(&[], &[])
            .expect("Can't create descriptor set layout");

        let render_pass = {
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

            self.device.borrow().logical_device.create_render_pass(&[attachment], &[subpass], &[dependency]).expect("Can't create render pass")
        };

        // Render pipeline.
        let pipeline = {
            let (vs_entry, fs_entry) = (
                pso::EntryPoint {
                    entry: "main",
                    module: &self.vertex_module.unwrap(),
                    specialization: pso::Specialization {
                        constants: &[
                            pso::SpecializationConstant {
                                id: 0,
                                range: 0 .. 4,
                            },
                        ],
                        data: unsafe { std::mem::transmute::<&f32, &[u8; 4]>(&0.8f32) },
                    },
                },

                pso::EntryPoint {
                    entry: "main",
                    module: &self.fragment_module.unwrap(),
                    specialization: pso::Specialization::default(),
                },
            );

            let pipeline_layout = self.device.borrow().logical_device
                .create_pipeline_layout(std::iter::once(&set_layout), &[(pso::ShaderStageFlags::VERTEX, 0..8)])
                .expect("Can't create pipeline layout");

            let shader_entries = pso::GraphicsShaderSet {
                vertex: vs_entry,
                hull: None,
                domain: None,
                geometry: None,
                fragment: Some(fs_entry),
            };

            let subpass = Subpass {
                index: 0,
                main_pass: &render_pass,
            };

            let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                shader_entries,
                Primitive::TriangleList,
                pso::Rasterizer::FILL,
                &pipeline_layout,
                subpass,
            );

            pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                pso::ColorMask::ALL,
                pso::BlendState::ALPHA,
            ));

            pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                binding: 0,
                stride: std::mem::size_of::<Vertex>() as u32,
                rate: 0,
            });

            pipeline_desc.attributes.push(pso::AttributeDesc {
                location: 0,
                binding: 0,
                element: pso::Element {
                    format: format::Format::Rgb32Float,
                    offset: 0,
                },
            });
            pipeline_desc.attributes.push(pso::AttributeDesc {
                location: 1,
                binding: 0,
                element: pso::Element {
                    format: format::Format::Rgba32Float,
                    offset: 8,
                },
            });

            self.device.borrow().logical_device.create_graphics_pipeline(&pipeline_desc, None)
                .expect("Failed to create graphics pipeline.")
        };

        let viewport = pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: 1024 as _,
                h: 768 as _,
            },
            depth: 0.0..1.0,
        };

        GfxEncoder { device: self.device, sync: self.sync, swapchain: self.swapchain, set_layout : Some(set_layout),
            pipeline : Some(pipeline), pipeline_layout: None, render_pass: Some(render_pass), viewport }
    }
}