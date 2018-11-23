use ash::version::InstanceV1_0;
use ash::vk;

/// Attempts to select the preferred format provided. If it cannot, it will select the first format it can find.
pub fn select_color_format(formats : Vec<vk::SurfaceFormatKHR>, preferred : vk::Format) -> vk::SurfaceFormatKHR {
    let returned_format = formats
        .iter()
        .find(|format| {
            format.format == preferred
        }).or(formats.get(0))
        .unwrap();
    returned_format.clone()
}

/// Returns the optimal depth-stencil format, if one exists. Returns `Some(vk::Format)` when a format exists, and None if
/// there are no supported depth-stencil formats.
pub fn select_depth_stencil_format(instance : ash::Instance,
                                   physical_device : vk::PhysicalDevice,
                                   preferred : vk::Format) -> Option<vk::Format> {
    let depth_stencil_formats = vec![vk::Format::D32_SFLOAT_S8_UINT,
                                     vk::Format::D32_SFLOAT,
                                     vk::Format::D24_UNORM_S8_UINT,
                                     vk::Format::D16_UNORM_S8_UINT,
                                     vk::Format::D16_UNORM];
    for format in depth_stencil_formats {
        let properties = unsafe { instance.get_physical_device_format_properties(physical_device, format) };
        if properties.optimal_tiling_features & vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT ==
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT  {
            return Some(format);
        }
    }
    None
}

pub fn find_memory_type_index(memory_req: &vk::MemoryRequirements,
                             memory_prop: &vk::PhysicalDeviceMemoryProperties,
                             flags: vk::MemoryPropertyFlags) -> Option<u32> {
    let mut memory_type_bits = memory_req.memory_type_bits;
    for (index, ref memory_type) in memory_prop.memory_types.iter().enumerate() {
        if memory_type_bits & 1 == 1 {
            return Some(index as u32);
        }
        memory_type_bits = memory_type_bits >> 1;
    }
    None
}

/*
pub fn get_max_multisampling_value(limits : vk::PhysicalDeviceLimits) -> vk::SampleCountFlags {
    let minimum_samples = limits.framebuffer_color_sample_counts.min(limits.framebuffer_depth_sample_counts);
    if minimum_samples & vk::SampleCountFlags::TYPE_64 = vk::SampleCountFlags::TYPE_64 {
        return vk::SampleCountFlags::TYPE_64
    } else if minimum_samples & vk::SampleCountFlags::TYPE_32 {
        return vk::SampleCountFlags::TYPE_32
    } else if vk::SampleCountFlags::TYPE_16 {
        return vk::SampleCountFlags::TYPE_16
    } else if vk::SampleCountFlags::TYPE_8 {
        return vk::SampleCountFlags::TYPE_8
    } else if vk::SampleCountFlags::TYPE_4 {
        return vk::SampleCountFlags::TYPE_4
    } else if vk::SampleCountFlags::TYPE_2 {
        return vk::SampleCountFlags::TYPE_2
    }
    vk::SampleCountFlags::TYPE_1
}
*/