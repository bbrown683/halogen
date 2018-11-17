use ash::version::InstanceV1_0;
use ash::vk;

/// Attempts to select the preferred format provided. If it cannot, it will select the first format it can find.
pub fn select_color_format(formats : Vec<vk::SurfaceFormatKHR>, preferred : vk::Format) -> (vk::Format, vk::ColorSpaceKHR) {
    let returned_format = formats
        .iter()
        .find(|format| {
            format.format == preferred
        }).or(formats.get(0))
        .unwrap();
    (returned_format.format, returned_format.color_space)
}

/// Returns the optimal depth-stencil format, if one exists. Returns `Some(vk::Format)` when a format exists, and None if
/// there are no supported depth-stencil formats.
pub fn select_depth_stencil_format(instance : ash::Instance,
                                   physical_device : vk::PhysicalDevice,
                                   preferred : vk::Format) -> Option<vk::Format> {
    unsafe {
        let depth_stencil_formats = vec![vk::Format::D32_SFLOAT_S8_UINT,
                                         vk::Format::D32_SFLOAT,
                                         vk::Format::D24_UNORM_S8_UINT,
                                         vk::Format::D16_UNORM_S8_UINT,
                                         vk::Format::D16_UNORM];
        for format in depth_stencil_formats {
            let properties = instance.get_physical_device_format_properties(physical_device, format);
            if properties.optimal_tiling_features & vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT ==
                vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT  {
                return Some(format);
            }
        }
    }
    None
}