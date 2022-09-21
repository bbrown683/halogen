use std::{cell::RefCell, ffi::CString, fs::File, io::Read, mem::size_of, rc::Rc};
use ash::vk;
use nalgebra::{Vector2, Vector3, Vector4};
use super::Device;

/// Creates a shader module with the provided device and bytes.
fn create_shader_module(device : &Rc<RefCell<Device>>, bytes : Vec<u8>) -> vk::ShaderModule {
    let module_create_info = vk::ShaderModuleCreateInfo {
        p_code: bytes.as_ptr() as *const u32,
        code_size: bytes.len(),
        ..Default::default()
    };

    unsafe {
        device
            .borrow()
            .ash_device()
            .create_shader_module(&module_create_info, None)
            .unwrap()
    }
}

/// Stores the vertex information associated.
pub struct Vertex {
    position : Vector3<f32>,
    color : Vector4<f32>,
    texture_coord : Vector2<f32>,
}

/// A material describes the appearance of an object in a rendered space.
pub struct Material {
    device : Rc<RefCell<Device>>,
    entry_point : CString,
    vertex_module : vk::ShaderModule,
    fragment_module : vk::ShaderModule,
    pipeline_shader_stages : Vec<vk::PipelineShaderStageCreateInfo>,
    pipeline_vertex_input_state : vk::PipelineVertexInputStateCreateInfo,
}

impl Drop for Material {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().ash_device().destroy_shader_module(self.vertex_module, None);
            self.device.borrow().ash_device().destroy_shader_module(self.fragment_module, None);
        }
        info!("Dropped Material")
    }
}

impl Material {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        // Have to keep this pointer alive.
        let entry_point = CString::new("main").unwrap();

        let vertex_module = create_shader_module(&device, include_bytes!("../assets/shaders/vert.spv").to_vec());
        let vertex_pipeline_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertex_module)
            .name(entry_point.as_c_str());

        let fragment_module = create_shader_module(&device, include_bytes!("../assets/shaders/frag.spv").to_vec());
        let fragment_pipeline_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragment_module)
            .name(entry_point.as_c_str());

        let pipeline_shader_stages = vec![vertex_pipeline_stage.build(), fragment_pipeline_stage.build()];
        let pipeline_vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .build();
        Self { device, entry_point, vertex_module, fragment_module, pipeline_shader_stages, pipeline_vertex_input_state }
    }

    pub fn vertex_buffer_size(&self) -> vk::DeviceSize { size_of::<Vertex>() as vk::DeviceSize }

    pub fn pipeline_shader_stages(&self) -> Vec<vk::PipelineShaderStageCreateInfo> { self.pipeline_shader_stages.clone() }

    pub fn pipeline_vertex_input_state(&self) -> vk::PipelineVertexInputStateCreateInfo { self.pipeline_vertex_input_state }
}