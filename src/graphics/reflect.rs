use ash::vk;
use spirv_reflect::types::{ReflectFormat, ReflectShaderStageFlags};

/// Returns the type which spirv-reflect maps to in Vulkan.
pub fn get_shader_stage(flag : ReflectShaderStageFlags) -> vk::ShaderStageFlags {
    match flag {
        ReflectShaderStageFlags::VERTEX => return vk::ShaderStageFlags::VERTEX,
        ReflectShaderStageFlags::FRAGMENT => return vk::ShaderStageFlags::FRAGMENT,
        ReflectShaderStageFlags::COMPUTE => return vk::ShaderStageFlags::COMPUTE,
        _ => return vk::ShaderStageFlags::ALL
    }
}

pub fn get_shader_input_format(format : ReflectFormat) -> vk::Format {
    match format {
        ReflectFormat::R32_SINT => vk::Format::R32_SINT,
        ReflectFormat::R32_UINT => vk::Format::R32_UINT,
        ReflectFormat::R32_SFLOAT => vk::Format::R32_SFLOAT,
        ReflectFormat::R32G32_UINT => vk::Format::R32G32_UINT,
        ReflectFormat::R32G32_SINT => vk::Format::R32G32_SINT,
        ReflectFormat::R32G32_SFLOAT => vk::Format::R32G32B32_UINT,
        ReflectFormat::R32G32B32_UINT => vk::Format::R32G32B32_UINT,
        ReflectFormat::R32G32B32_SINT => vk::Format::R32G32B32_SINT,
        ReflectFormat::R32G32B32_SFLOAT => vk::Format::R32G32B32_SFLOAT,
        ReflectFormat::R32G32B32A32_UINT => vk::Format::R32G32B32A32_UINT,
        ReflectFormat::R32G32B32A32_SINT => vk::Format::R32G32B32A32_SINT,
        ReflectFormat::R32G32B32A32_SFLOAT => vk::Format::R32G32B32A32_SFLOAT,
        _ => vk::Format::UNDEFINED
    }
}