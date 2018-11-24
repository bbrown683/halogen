use std::cell::RefCell;
use std::rc::Rc;
use ash::extensions::Swapchain;
use ash::version::{InstanceV1_0, DeviceV1_0};
use ash::vk;
use super::{Instance, Queue};

pub enum DeviceCreationError {
    MissingExtensions
}

pub struct Device {
    physical_device : vk::PhysicalDevice,
    properties : vk::PhysicalDeviceProperties,
    limits : vk::PhysicalDeviceLimits,
    memory_properties : vk::PhysicalDeviceMemoryProperties,
    device : ash::Device,
    compute_index : u32,
    graphics_index : u32,
    transfer_index : u32,
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
        }
        info!("Dropped Device")
    }
}

impl Device {
    pub fn new(instance: &Instance) -> Result<Self,DeviceCreationError> {
        let physical_device = instance.select_primary_physical_device();
        let (properties, features, limits, memory_properties, queue_families) = unsafe {
            let properties = instance
                .get_ash_instance()
                .get_physical_device_properties(physical_device);
            let features = instance
                .get_ash_instance()
                .get_physical_device_features(physical_device);
            let limits = properties.limits;
            let memory_properties = instance
                .get_ash_instance()
                .get_physical_device_memory_properties(physical_device);
            let queue_families = instance
                .get_ash_instance()
                .get_physical_device_queue_family_properties(physical_device);
            (properties, features, limits, memory_properties, queue_families)
        };

        let mut compute_index = 0;
        let mut graphics_index = 0;
        let mut transfer_index = 0;
        for index in 0..queue_families.len() {
            let queue_family = queue_families.get(index).unwrap();
            if queue_family.queue_flags & vk::QueueFlags::GRAPHICS
                == vk::QueueFlags::GRAPHICS {
                graphics_index = index as u32;
            }
            if queue_family.queue_flags & vk::QueueFlags::COMPUTE
                == vk::QueueFlags::COMPUTE {
                compute_index = index as u32;
            }
            if queue_family.queue_flags & vk::QueueFlags::TRANSFER
                == vk::QueueFlags::TRANSFER {
                transfer_index = index as u32;
            }
        }

        let priorities = [1.0];

        let compute_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(compute_index)
            .queue_priorities(&priorities)
            .build();
        let graphics_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(graphics_index)
            .queue_priorities(&priorities)
            .build();
        let transfer_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(transfer_index)
            .queue_priorities(&priorities)
            .build();

        let enabled_features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .fill_mode_non_solid(true)
            .build();

        let queue_infos = vec![compute_info, graphics_info, transfer_info];
        let device_extensions = [Swapchain::name().as_ptr()];
        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(queue_infos.as_slice())
            .enabled_extension_names(device_extensions.as_ref())
            .enabled_features(&enabled_features)
            .build();

        let device = unsafe {
            instance
                .get_ash_instance()
                .create_device(physical_device, &device_info, None)
                .unwrap()
        };

        Ok(Self {
            physical_device,
            properties,
            limits,
            memory_properties,
            device,
            compute_index,
            graphics_index,
            transfer_index,
        })
    }

    pub fn get_ash_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    pub fn get_compute_queue_index(&self) -> u32 {
        self.compute_index
    }

    pub fn get_graphics_queue_index(&self) -> u32 {
        self.graphics_index
    }

    pub fn get_transfer_queue_index(&self) -> u32 {
        self.transfer_index
    }

    pub fn get_properties(&self) -> vk::PhysicalDeviceProperties {
        self.properties
    }

    pub fn get_limits(&self) -> vk::PhysicalDeviceLimits {
        self.limits
    }

    pub fn get_memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        self.memory_properties
    }
}