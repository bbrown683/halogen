use std::cell::RefCell;
use std::rc::Rc;
use hal::{buffer, memory, format, pso};
use hal::{Backend, Device, Primitive};
use hal::pass::Subpass;
use nalgebra::{Vector3};
use crate::gfx::GfxDevice;

/// Vertex structure for a `GfxRenderable`.
pub struct GfxVertex {
    /// 3-Dimensional coordinates in space denoted by x, y, and z.
    pub position : Vector3<f32>,
//    pub color : Vector4<f32>,
}

/// Specifies all relevant rendering state associated with an object or set of objects. Think of it
/// as an apple, with the properties defining how it appears. Located, color, and lighting properties
/// are an example of these properties.
pub struct GfxRenderable<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    pub vertex_buffer : Option<B::Buffer>,
    pub index_buffer : Option<B::Buffer>,
    pub desc_set_layout : Option<B::DescriptorSetLayout>,
    pub pipeline : Option<B::GraphicsPipeline>,
    pub pipeline_layout : Option<B::PipelineLayout>,
}

impl<B: Backend> Drop for GfxRenderable<B> {
    fn drop(&mut self) {
        if self.index_buffer.is_some() {
            &self.device.borrow().logical_device.destroy_buffer(self.index_buffer.take().unwrap());
            debug_assert!(self.index_buffer.is_none());
        }

        &self.device.borrow().logical_device.destroy_buffer(self.vertex_buffer.take().unwrap());
        debug_assert!(self.vertex_buffer.is_none());
        &self.device.borrow().logical_device.destroy_descriptor_set_layout(self.desc_set_layout.take().unwrap());
        debug_assert!(self.desc_set_layout.is_none());
        &self.device.borrow().logical_device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
        debug_assert!(self.pipeline_layout.is_none());
        &self.device.borrow().logical_device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
        debug_assert!(self.pipeline.is_none());
    }
}

impl<B: Backend> GfxRenderable<B> {
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               vertices : Vec<GfxVertex>,
               indices : Option<Vec<u16>>,
               vertex_bytes : Vec<u8>,
               fragment_bytes : Vec<u8>) -> Self {
        // Create vertex buffer.
        let vertex_buffer = {
            let device_borrow = device.borrow();

            let buffer_stride = std::mem::size_of::<GfxVertex>() as u64;
            let buffer_len = vertices.len() as u64 * buffer_stride;

            let buffer_unbound = device_borrow.logical_device
                .create_buffer(buffer_len, buffer::Usage::VERTEX)
                .unwrap();
            let buffer_req = device.borrow().logical_device
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

            Some(device_borrow.logical_device
                .bind_buffer_memory(&buffer_memory, 0, buffer_unbound)
                .expect("Failed to create vertex buffer"))
        };

        // Create index buffer.
        let index_buffer = {
            if indices.is_some() {
                let device_borrow = device.borrow();

                let buffer_stride = std::mem::size_of::<u16>() as u64;
                let buffer_len = indices.as_ref().unwrap().len() as u64 * buffer_stride;

                let buffer_unbound = device_borrow.logical_device
                    .create_buffer(buffer_len, buffer::Usage::INDEX)
                    .unwrap();
                let buffer_req = device.borrow().logical_device
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

                Some(device_borrow.logical_device
                    .bind_buffer_memory(&buffer_memory, 0, buffer_unbound)
                    .expect("Failed to create vertex buffer"))
            } else {
                None
            }
        };

        let desc_set_layout = device.borrow().logical_device
            .create_descriptor_set_layout(&[], &[])
            .expect("Can't create descriptor set layout");

        let pipeline_layout = device.borrow().logical_device
            .create_pipeline_layout(std::iter::once(&desc_set_layout), &[(pso::ShaderStageFlags::VERTEX, 0..8)])
            .expect("Can't create pipeline layout");

        // Render pipeline.
        let pipeline = {
            let device_borrow = device.borrow();

            // Create vertex and fragment shader modules.
            let vertex_module = device_borrow
                .logical_device
                .create_shader_module(&vertex_bytes)
                .expect("Failed to create shader module.");
            let fragment_module = device_borrow
                .logical_device
                .create_shader_module(&fragment_bytes)
                .expect("Failed to create shader module.");

            let pipeline = {
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint {
                        entry: "main",
                        module: &vertex_module,
                        specialization: pso::Specialization {
                            constants: &[
                                pso::SpecializationConstant {
                                    id: 0,
                                    range: 0..4,
                                },
                            ],
                            data: unsafe { std::mem::transmute::<&f32, &[u8; 4]>(&0.8f32) },
                        },
                    },
                    pso::EntryPoint {
                        entry: "main",
                        module: &fragment_module,
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
                    main_pass: device_borrow.render_pass.as_ref().unwrap(),
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
                    stride: std::mem::size_of::<GfxVertex>() as u32,
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

                device_borrow.logical_device.create_graphics_pipeline(&pipeline_desc, None)
            };

            device.borrow().logical_device.destroy_shader_module(vertex_module);
            device.borrow().logical_device.destroy_shader_module(fragment_module);
            pipeline.expect("Failed to create pipeline")
        };

        Self { device, vertex_buffer, index_buffer, desc_set_layout: Some(desc_set_layout),
            pipeline: Some(pipeline), pipeline_layout: Some(pipeline_layout) }
    }
}
