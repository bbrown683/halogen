use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk::{ShaderModule, ShaderModuleCreateInfo, VertexInputAttributeDescription, VertexInputBindingDescription,
              PipelineVertexInputStateCreateInfo, PipelineLayoutCreateInfo, PipelineShaderStageCreateInfo};
use nalgebra::{Vector2, Vector4};
use super::Device;

/// Holds on to a cache of shaders for usage during the runtime of the program.
pub struct ShaderCache {
    device : Rc<RefCell<Device>>,
    shaders : Vec<ShaderModule>,
}

impl Drop for ShaderCache {
    fn drop(&mut self) {

    }
}

impl ShaderCache {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        Self { device, shaders: Vec::new() }
    }

    /// Adds a shader directly from bytes. Useful when combined with the `include_bytes` macro
    pub fn add_shader_from_bytes(mut self, bytes : Vec<u8>) -> Self {
        let module_create_info = ShaderModuleCreateInfo {
            p_code: bytes.as_ptr() as *const u32,
            code_size: bytes.len(),
            ..Default::default()
        };

        let shader_module = unsafe {
            self.device
                .borrow()
                .get_ash_device()
                .create_shader_module(&module_create_info, None)
                .unwrap()
        };
        self
    }
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
/// In Vulkan these are represented by shaders.
pub trait Material {
    fn pipeline_shader_stages(&self) -> Vec<PipelineShaderStageCreateInfo>;
    fn pipeline_vertex_input_state(&self) ->  PipelineVertexInputStateCreateInfo;
}

/// Represents a material with only colors.
pub struct ColoredMaterial;

impl Material for ColoredMaterial {
    fn pipeline_shader_stages(&self) -> Vec<PipelineShaderStageCreateInfo> {
        unimplemented!()
    }

    fn pipeline_vertex_input_state(&self) -> PipelineVertexInputStateCreateInfo {
        unimplemented!()
    }
}

impl ColoredMaterial {
    pub fn new() -> Self { Self {} }
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