use std::ffi::CString;
use ash::vk::{self, Result as VkResult};
use ash::extensions::ext::DebugUtils;

use super::platform::get_required_instance_extensions;
use super::debug::debug_utils_callback;

/// Provides a brief overview of why an instance failed to be created.
pub enum InstanceCreationError {
    /// Unknown or uncaptured error.
    Unknown,
    /// Triggered if there is no [Vulkan ICD](https://github.com/KhronosGroup/Vulkan-LoaderAndValidationLayers/blob/master/loader/LoaderAndLayerInterface.md#installable-client-drivers).
    MissingDriver,
    /// There are required extensions which were not found.
    MissingExtensions,
    /// There are required layers which were not found. This would only be triggered in debug mode.
    MissingLayers,
}

/// Contains vulkan instance-level loaders and handles.
pub struct Instance {
    entry : ash::Entry,
    instance : ash::Instance,
    debug_utils_loader : Option<DebugUtils>,
    debug_utils : Option<vk::DebugUtilsMessengerEXT>,
    physical_devices : Vec<vk::PhysicalDevice>,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            // Check if debug report extension was toggled.
            if self.debug_utils_loader.is_some() {
                self.debug_utils_loader.take().unwrap().destroy_debug_utils_messenger(
                    self.debug_utils.take().unwrap(), None);
            }
            self.instance.destroy_instance(None);
        }
        info!("Dropped Instance")
    }
}

impl Instance {
    pub fn new() -> Result<Self,InstanceCreationError> {
        let entry = unsafe { 
            let entry_result = ash::Entry::load();
            match entry_result {
                Ok(entry) => (entry),
                Err(_error) => return Err(InstanceCreationError::MissingDriver)
            } 
        };

        let layer_names = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        let layer_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        let extension_names = get_required_instance_extensions();

        // Enable validation layer only on debug builds.
        let instance_info = if cfg!(debug_assertions) {
            vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&extension_names)
                .enabled_layer_names(&layer_names_raw)
                .build()
        } else {
            vk::InstanceCreateInfo::builder()
                .enabled_extension_names(&extension_names)
                .build()
        };

        let instance = unsafe {
            let instance_result = entry.create_instance(&instance_info, None);
            match instance_result {
                Ok(instance) => (instance),
                Err(error) => match error {
                    VkResult::ERROR_INCOMPATIBLE_DRIVER => return Err(InstanceCreationError::MissingDriver),
                    VkResult::ERROR_EXTENSION_NOT_PRESENT => return Err(InstanceCreationError::MissingExtensions),
                    VkResult::ERROR_LAYER_NOT_PRESENT => return Err(InstanceCreationError::MissingLayers),
                    _ => return Err(InstanceCreationError::Unknown),
                }
            }
        };

        // Only enable the debug utils callback on debug builds.
        let (debug_utils_loader, debug_utils) = if cfg!(debug_assertions) {
            let debug_utils_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO)
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL 
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE 
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
                .pfn_user_callback(Some(debug_utils_callback));

                let debug_utils_loader = DebugUtils::new(&entry, &instance);
                let debug_utils = unsafe {
                    debug_utils_loader
                    .create_debug_utils_messenger(&debug_utils_info, None)
                    .unwrap()
                };

                (Some(debug_utils_loader), Some(debug_utils))
        } else {
            (None, None)
        };

        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to retrieve physical devices.")
        };

        Ok(Self { entry,
            instance,
            debug_utils_loader,
            debug_utils,
            physical_devices
        })
    }

    /// Returns the ash entrypoint.
    pub fn ash_entry(&self) -> &ash::Entry {
        &self.entry
    }

    /// Returns the ash instance.
    pub fn ash_instance(&self) -> &ash::Instance {
        &self.instance
    }

    /// Returns all physical devices.
    pub fn physical_devices(&self) -> Vec<vk::PhysicalDevice> {
        self.physical_devices.clone()
    }

    /// Returns the first adapter in the sequence.
    pub fn select_primary_physical_device(&self) -> vk::PhysicalDevice {
        self.physical_devices.clone().remove(0)
    }
}
