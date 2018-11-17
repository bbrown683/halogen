use std::default::Default;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::os::raw::{c_char, c_void};
use std::ptr;

use ash::vk;
use ash::extensions::{DebugReport, Surface};
use ash::version::{EntryV1_0, InstanceV1_0};

use super::platform::{create_surface, get_required_instance_extensions};
use super::debug::debug_callback;

/// Provides a brief overview of why an instance failed to be created.
pub enum InstanceCreationError {
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
    debug_report_loader : Option<DebugReport>,
    debug_report : Option<vk::DebugReportCallbackEXT>,
    physical_devices : Vec<vk::PhysicalDevice>,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            // Check if debug report extension was toggled.
            if self.debug_report_loader.is_some() {
                self.debug_report_loader.take().unwrap().destroy_debug_report_callback_ext(
                    self.debug_report.take().unwrap(), None);
            }
            self.instance.destroy_instance(None);
            info!("Dropped Instance.")
        }
    }
}

impl Instance {
    pub fn new() -> Self {
        unsafe {
            let entry = ash::Entry::new().unwrap();

            let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
            let layers_names_raw: Vec<*const i8> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let extension_names = get_required_instance_extensions();
            let instance_info = vk::InstanceCreateInfo {
                s_type: vk::StructureType::INSTANCE_CREATE_INFO,
                p_next: ptr::null(),
                flags: Default::default(),
                p_application_info: &Default::default(),
                pp_enabled_layer_names: layers_names_raw.as_ptr(),
                enabled_layer_count: layers_names_raw.len() as u32,
                pp_enabled_extension_names: extension_names.as_ptr(),
                enabled_extension_count: extension_names.len() as u32,
            };

            let instance = entry.create_instance(&instance_info, None)
                .expect("Failed to create vulkan instance.");

            // Only enable the report callback on debug builds.
            let (debug_report_loader, debug_report) = if cfg!(debug_assertions) {
                let debug_info = vk::DebugReportCallbackCreateInfoEXT {
                    s_type: vk::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
                    p_next: ptr::null(),
                    flags: vk::DebugReportFlagsEXT::ERROR
                        | vk::DebugReportFlagsEXT::WARNING
                        | vk::DebugReportFlagsEXT::DEBUG
                        | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
                        | vk::DebugReportFlagsEXT::INFORMATION,
                    pfn_callback: Some(debug_callback),
                    p_user_data: ptr::null_mut(),
                };
                let debug_report_loader = DebugReport::new(&entry, &instance);
                let debug_report = debug_report_loader.create_debug_report_callback_ext(
                    &debug_info,
                    None)
                    .unwrap();
                (Some(debug_report_loader), Some(debug_report))
            } else {
                (None, None)
            };

            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Failed to retrieve physical devices.");

            Self {
                entry,
                instance,
                debug_report_loader,
                debug_report,
                physical_devices
            }
        }
    }

    /// Returns the ash entrypoint.
    pub fn get_ash_entry(&self) -> &ash::Entry {
        &self.entry
    }

    /// Returns the ash instance.
    pub fn get_ash_instance(&self) -> &ash::Instance {
        &self.instance
    }

    /// Returns all physical devices.
    pub fn get_physical_devices(&self) -> Vec<vk::PhysicalDevice> {
        self.physical_devices.clone()
    }

    /// Returns the first adapter in the sequence.
    pub fn select_primary_physical_device(&self) -> vk::PhysicalDevice {
        self.physical_devices.clone().remove(0)
    }
}
