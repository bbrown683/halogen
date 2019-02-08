use ash::vk::{ShaderModule, VertexInputAttributeDescription, VertexInputBindingDescription};
use nalgebra::{Vector2, Vector4};

pub struct ShaderCache {

}

pub trait Material {
    fn vertex_input_attribute_description() -> VertexInputAttributeDescription;
    fn vertex_input_binding_description() -> VertexInputBindingDescription;
    fn vertex_shader_module() -> ShaderModule;
    fn geometry_shader_module() -> Option<ShaderModule>;
    fn tesselation_control_shader_module() -> Option<ShaderModule>;
    fn tesselation_evaluation_shader_module() -> Option<ShaderModule>;
    fn fragment_shader_module() -> ShaderModule;
    fn fragment_shader_sampler_description();
}

/// Represents a material with only colors.
pub struct ColoredMaterial {
    colors : Vec<Vector4<f32>>,
}

impl Material for ColoredMaterial {
    fn vertex_input_attribute_description() -> VertexInputAttributeDescription {
        unimplemented!()
    }

    fn vertex_input_binding_description() -> VertexInputBindingDescription {
        unimplemented!()
    }

    fn vertex_shader_module() -> ShaderModule {
        unimplemented!()
    }

    fn geometry_shader_module() -> Option<ShaderModule> {
        None
    }

    fn tesselation_control_shader_module() -> Option<ShaderModule> {
        None
    }

    fn tesselation_evaluation_shader_module() -> Option<ShaderModule> {
        None
    }

    fn fragment_shader_module() -> ShaderModule {
        unimplemented!()
    }

    fn fragment_shader_sampler_description() {
        unimplemented!()
    }
}

/// Represents a material with only a texture.
pub struct TexturedMaterial {
    texture_coords : Vec<Vector2<f32>>,
}

impl Material for TexturedMaterial {
    fn vertex_input_attribute_description() -> VertexInputAttributeDescription {
        unimplemented!()
    }

    fn vertex_input_binding_description() -> VertexInputBindingDescription {
        unimplemented!()
    }

    fn vertex_shader_module() -> ShaderModule {
        unimplemented!()
    }

    fn geometry_shader_module() -> Option<ShaderModule> {
        None
    }

    fn tesselation_control_shader_module() -> Option<ShaderModule> {
        None
    }

    fn tesselation_evaluation_shader_module() -> Option<ShaderModule> {
        None
    }

    fn fragment_shader_module() -> ShaderModule {
        unimplemented!()
    }

    fn fragment_shader_sampler_description() {
        unimplemented!()
    }
}