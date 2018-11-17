use std::default::Default;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::os::raw::{c_char, c_void};
use std::ptr;
use ash::extensions::Swapchain;
use ash::version::{InstanceV1_0, DeviceV1_0};
use ash::vk;
use super::instance::Instance;

pub enum DeviceCreationError {
    MissingExtensions
}

pub struct Device {
    physical_device : vk::PhysicalDevice,
    device : ash::Device,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
        }
    }
}

impl Device {
    pub fn new(instance: &Instance) -> Result<Self,DeviceCreationError> {
        unsafe {
            let physical_device = instance.select_primary_physical_device();
            let graphics_queue_index = instance
                .get_ash_instance()
                .get_physical_device_queue_family_properties(physical_device.clone())
                .iter()
                .enumerate()
                .filter_map(|(index, ref info)| {
                    let supports_graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                    match supports_graphics {
                        true => Some(index),
                        _ => None
                    }
                }).nth(0)
                .unwrap() as u32;

            let device_extension_names = [Swapchain::name().as_ptr()];
            let priorities = [1.0];

            let queue_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_family_index: graphics_queue_index,
                p_queue_priorities: priorities.as_ptr(),
                queue_count: priorities.len() as u32,
            };

            let device_info = vk::DeviceCreateInfo {
                s_type: vk::StructureType::DEVICE_CREATE_INFO,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_create_info_count: 1,
                p_queue_create_infos: &queue_info,
                enabled_layer_count: 0,
                pp_enabled_layer_names: ptr::null(),
                enabled_extension_count: device_extension_names.len() as u32,
                pp_enabled_extension_names: device_extension_names.as_ptr(),
                p_enabled_features: &Default::default(),
            };

            let device = instance
                .get_ash_instance()
                .create_device(physical_device, &device_info, None)
                .unwrap();
            Ok(Self { physical_device,
                device,
            })
        }
    }

    pub fn get_ash_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device.clone()
    }
}