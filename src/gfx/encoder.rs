use std::cell::RefCell;
use std::rc::Rc;
use hal::{Backend, CommandPool, Device, DescriptorPool, Graphics, Primitive};
use hal::{buffer, format, image, memory};
use hal::command::{CommandBuffer, MultiShot, Primary};
use hal::image::Extent;
use hal::pass::{self, Subpass};
use hal::pso::{self, PipelineStage, ShaderStageFlags};
use hal::queue::Submission;
use crate::gfx::{GfxDevice, GfxSwapchain, GfxSync, Vertex};

// Encodes render state to a command buffer from a command pool.
pub struct GfxEncoder<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    vertex_buffer : Option<B::Buffer>,
    set_layout : Option<B::DescriptorSetLayout>,
    pipeline : Option<B::GraphicsPipeline>,
    pipeline_layout : Option<B::PipelineLayout>,
    render_pass : Option<B::RenderPass>,
    viewport : pso::Viewport,
}

impl<B: Backend> Drop for GfxEncoder<B> {
    fn drop(&mut self) {
        &self.device.borrow().logical_device.destroy_buffer(self.vertex_buffer.take().unwrap());
        debug_assert!(self.vertex_buffer.is_none());
        &self.device.borrow().logical_device.destroy_descriptor_set_layout(self.set_layout.take().unwrap());
        debug_assert!(self.set_layout.is_none());
        &self.device.borrow().logical_device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
        debug_assert!(self.pipeline_layout.is_none());
        &self.device.borrow().logical_device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
        debug_assert!(self.pipeline.is_none());
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
    vertex_module : Option<B::ShaderModule>,
    fragment_module : Option<B::ShaderModule>,
    vertices : Vec<Vertex>,
    indices : Vec<u16>,
    extent : Option<Extent>,
}

impl<B: Backend> EncoderBuilder<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>) -> Self {
        Self { device, vertex_module: None, fragment_module: None,
        vertices : Vec::new(), indices: Vec::new(), extent : None }
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
        self.vertices = vertices;
        self
    }

    pub fn with_indices(mut self, indices : Vec<u16>) -> Self {
        self.indices = indices;
        self
    }

    pub fn with_extent(mut self, extent : Extent) -> Self {
        self.extent = Some(extent);
        self
    }

    /// Builds the GfxEncoder object with the required state necessary to begin using it.
    pub fn build(self) -> GfxEncoder<B> {
        let vertex_buffer = {
            let device_borrow = self.device.borrow();

            let buffer_stride = std::mem::size_of::<Vertex>() as u64;
            let buffer_len = self.vertices.len() as u64 * buffer_stride;

            let buffer_unbound = device_borrow.logical_device
                .create_buffer(buffer_len, buffer::Usage::VERTEX)
                .unwrap();
            let buffer_req = device_borrow.logical_device
                .get_buffer_requirements(&buffer_unbound);

            let memory_types = device_borrow.memory_properties.memory_types.clone();
            let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    buffer_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(memory::Properties::CPU_VISIBLE)
                })
                .unwrap()
                .into();

            let buffer_memory = device_borrow.logical_device
                .allocate_memory(upload_type, buffer_req.size)
                .unwrap();

            device_borrow.logical_device
                .bind_buffer_memory(&buffer_memory, 0, buffer_unbound)
                .expect("Failed to create vertex buffer")
        };

        let set_layout = self.device.borrow().logical_device
            .create_descriptor_set_layout(&[], &[])
            .expect("Can't create descriptor set layout");

        let pipeline_layout = self.device.borrow().logical_device
            .create_pipeline_layout(std::iter::once(&set_layout), &[(pso::ShaderStageFlags::VERTEX, 0..8)])
            .expect("Can't create pipeline layout");

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

        GfxEncoder { device: self.device, set_layout : Some(set_layout), vertex_buffer: Some(vertex_buffer),
            pipeline : Some(pipeline), pipeline_layout: Some(pipeline_layout), render_pass: Some(render_pass), viewport }
    }
}