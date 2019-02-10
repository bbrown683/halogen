use std::cell::RefCell;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk::{Format, PipelineVertexInputStateCreateInfo, PipelineLayoutCreateInfo, PipelineShaderStageCreateInfo,
              ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags, VertexInputAttributeDescription,
              VertexInputBindingDescription, VertexInputRate};
use nalgebra::{Vector2, Vector4};
use super::Device;

/// Creates a shader module with the provided device and bytes.
fn create_shader_module(device : &Rc<RefCell<Device>>, bytes : Vec<u8>) -> ShaderModule {
    let module_create_info = ShaderModuleCreateInfo {
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

pub enum VertexType {
    Colored(ColoredVertex),
    Textured(TexturedVertex)
}

pub struct ColoredVertex {
    colors : Vec<Vector4<f32>>,
}

pub struct TexturedVertex {
    texture_coords : Vec<Vector2<f32>>,
}

pub enum MaterialType {
    Colored(ColoredMaterial),
    Textured(TexturedMaterial)
}

/// A material describes the appearance of an object in a rendered space.
pub trait Material {
    fn pipeline_shader_stages(&self) -> Vec<PipelineShaderStageCreateInfo>;
    fn pipeline_vertex_input_state(&self) ->  PipelineVertexInputStateCreateInfo;
}

/// Represents a material with only color.
pub struct ColoredMaterial {
    device : Rc<RefCell<Device>>,
    entry_point : CString,
    vertex_module : ShaderModule,
    fragment_module : ShaderModule,
    pipeline_shader_stages : Vec<PipelineShaderStageCreateInfo>,
    pipeline_vertex_input_state : PipelineVertexInputStateCreateInfo,
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
    fn pipeline_shader_stages(&self) -> Vec<PipelineShaderStageCreateInfo> { self.pipeline_shader_stages.clone() }

    fn pipeline_vertex_input_state(&self) -> PipelineVertexInputStateCreateInfo { self.pipeline_vertex_input_state }
}

impl ColoredMaterial {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        // Have to keep this pointer alive.
        let entry_point = CString::new("main").unwrap();

        let vertex_module = create_shader_module(&device, include_bytes!("../assets/shaders/vert.spv").to_vec());
        let vertex_pipeline_stage = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::VERTEX)
            .module(vertex_module)
            .name(entry_point.as_c_str());

        let fragment_module = create_shader_module(&device, include_bytes!("../assets/shaders/frag.spv").to_vec());
        let fragment_pipeline_stage = PipelineShaderStageCreateInfo::builder()
            .stage(ShaderStageFlags::FRAGMENT)
            .module(fragment_module)
            .name(entry_point.as_c_str());

        let pipeline_shader_stages = vec![vertex_pipeline_stage.build(), fragment_pipeline_stage.build()];
        let pipeline_vertex_input_state = PipelineVertexInputStateCreateInfo::builder()
            .build();
        Self { device, entry_point, vertex_module, fragment_module, pipeline_shader_stages, pipeline_vertex_input_state }
    }
}

/// Represents a material with only a texture.
pub struct TexturedMaterial;

impl Material for TexturedMaterial {
    fn pipeline_shader_stages(&self) -> Vec<PipelineShaderStageCreateInfo> {
        unimplemented!()
    }

    fn pipeline_vertex_input_state(&self) -> PipelineVertexInputStateCreateInfo {
        unimplemented!()
    }
}

impl TexturedMaterial {
    pub fn new() -> Self { Self {} }
}