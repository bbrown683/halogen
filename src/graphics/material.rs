use std::mem;
use ash::vk;
use nalgebra::{Vector2, Vector3, Vector4};
use super::mesh::VertexDescription;

pub struct MaterialDescription {
    pub bytecodes : Vec<(vk::ShaderStageFlags, Vec<u8>)>,
    pub vertex_attributes : Vec<vk::VertexInputAttributeDescription>,
    pub vertex_bindings : Vec<vk::VertexInputBindingDescription>,
    pub descriptor_sets : Vec<vk::DescriptorSetLayoutBinding>,
}

pub enum Material {
    Colored,
    Textured,
}

impl Material {
    pub fn get_description(&self) -> MaterialDescription {
        match self {
            Material::Colored => {
                let mut bytecodes = Vec::new();
                bytecodes.push((vk::ShaderStageFlags::VERTEX, include_bytes!("../assets/shaders/vert.spv").to_vec()));
                bytecodes.push((vk::ShaderStageFlags::FRAGMENT, include_bytes!("../assets/shaders/frag.spv").to_vec()));

                let mut vertex_attributes = Vec::new();
                vertex_attributes.push(vk::VertexInputAttributeDescription::builder()
                    .binding(0)
                    .format(vk::Format::R32G32B32_SFLOAT)
                    .location(0)
                    .offset(0)
                    .build());
                vertex_attributes.push(vk::VertexInputAttributeDescription::builder()
                    .binding(1)
                    .format(vk::Format::R32G32B32_SFLOAT)
                    .location(1)
                    .offset(mem::size_of::<Vector3<f32>>() as u32)
                    .build());

                let mut vertex_bindings = Vec::new();
                vertex_bindings.push(vk::VertexInputBindingDescription::builder()
                    .binding(0)
                    .stride(mem::size_of::<VertexDescription>() as u32)
                    .input_rate(vk::VertexInputRate::VERTEX)
                    .build());
                vertex_bindings.push(vk::VertexInputBindingDescription::builder()
                    .binding(1)
                    .stride(mem::size_of::<VertexDescription>() as u32)
                    .input_rate(vk::VertexInputRate::VERTEX)
                    .build());

                let mut descriptor_sets = Vec::new();
                descriptor_sets.push(vk::DescriptorSetLayoutBinding::builder()
                    .build());

                MaterialDescription {
                    bytecodes,
                    vertex_attributes,
                    vertex_bindings,
                    descriptor_sets,
                }
            },
            Material::Textured => {
                let mut bytecodes = Vec::new();
                bytecodes.push((vk::ShaderStageFlags::VERTEX, include_bytes!("../assets/shaders/vert.spv").to_vec()));
                bytecodes.push((vk::ShaderStageFlags::FRAGMENT, include_bytes!("../assets/shaders/frag.spv").to_vec()));

                let mut vertex_attributes = Vec::new();
                vertex_attributes.push(vk::VertexInputAttributeDescription::builder()
                    .binding(0)
                    .format(vk::Format::R32G32B32_SFLOAT)
                    .location(0)
                    .offset(0)
                    .build());
                vertex_attributes.push(vk::VertexInputAttributeDescription::builder()
                    .binding(1)
                    .format(vk::Format::R32G32_SFLOAT)
                    .location(1)
                    .offset(mem::size_of::<Vector3<f32>>() as u32)
                    .build());

                let mut vertex_bindings = Vec::new();
                vertex_bindings.push(vk::VertexInputBindingDescription::builder()
                    .binding(0)
                    .stride(mem::size_of::<VertexDescription>() as u32)
                    .input_rate(vk::VertexInputRate::VERTEX)
                    .build());
                vertex_bindings.push(vk::VertexInputBindingDescription::builder()
                    .binding(1)
                    .stride(mem::size_of::<VertexDescription>() as u32)
                    .input_rate(vk::VertexInputRate::VERTEX)
                    .build());

                let mut descriptor_sets = Vec::new();
                descriptor_sets.push(vk::DescriptorSetLayoutBinding::builder()
                    .binding(1)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                    .build());

                MaterialDescription {
                    bytecodes,
                    vertex_attributes,
                    vertex_bindings,
                    descriptor_sets,
                }
            }
        }
    }
}