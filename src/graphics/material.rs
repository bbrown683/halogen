use std::{cell::RefCell, ffi::CString, fs::File, io::Read, rc::Rc};
use ash::version::DeviceV1_0;
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

    let shader_module = unsafe {
        device
            .borrow()
            .get_ash_device()
            .create_shader_module(&module_create_info, None)
            .unwrap()
    };
    shader_module
}

pub trait Vertex {}

pub struct ColoredVertex {
    position : Vector3<f32>,
    color : Vector4<f32>,
}

impl Vertex for ColoredVertex {}

pub struct TexturedVertex {
    position : Vector3<f32>,
    texture_coord : Vector2<f32>,
}

impl Vertex for TexturedVertex {}

/// A material describes the appearance of an object in a rendered space.
pub trait Material {
    fn pipeline_shader_stages(&self) -> Vec<vk::PipelineShaderStageCreateInfo>;
    fn pipeline_vertex_input_state(&self) ->  vk::PipelineVertexInputStateCreateInfo;
}

/// Represents a material with only color.
pub struct ColoredMaterial {
    device : Rc<RefCell<Device>>,
    entry_point : CString,
    vertex_module : vk::ShaderModule,
    fragment_module : vk::ShaderModule,
    pipeline_shader_stages : Vec<vk::PipelineShaderStageCreateInfo>,
    pipeline_vertex_input_state : vk::PipelineVertexInputStateCreateInfo,
}

impl Drop for ColoredMaterial {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_shader_module(self.vertex_module, None);
            self.device.borrow().get_ash_device().destroy_shader_module(self.fragment_module, None);
        }
        info!("Dropped ColoredMaterial")
    }
}

impl Material for ColoredMaterial {
    fn pipeline_shader_stages(&self) -> Vec<vk::PipelineShaderStageCreateInfo> { self.pipeline_shader_stages.clone() }

    fn pipeline_vertex_input_state(&self) -> vk::PipelineVertexInputStateCreateInfo { self.pipeline_vertex_input_state }
}

impl ColoredMaterial {
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
}

/// Represents a material with only a texture.
pub struct TexturedMaterial;

impl Material for TexturedMaterial {
    fn pipeline_shader_stages(&self) -> Vec<vk::PipelineShaderStageCreateInfo> {
        unimplemented!()
    }

    fn pipeline_vertex_input_state(&self) -> vk::PipelineVertexInputStateCreateInfo {
        unimplemented!()
    }
}

impl TexturedMaterial {
    pub fn new() -> Self { Self {} }
}