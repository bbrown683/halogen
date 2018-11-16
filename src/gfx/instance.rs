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

pub struct Instance {
    entry : ash::Entry,
    instance : ash::Instance,
    debug_report_loader : Option<DebugReport>,
    debug_report : Option<vk::DebugReportCallbackEXT>,
    surface_loader : Surface,
    surface : vk::SurfaceKHR,
    physical_devices : Vec<vk::PhysicalDevice>,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface_khr(self.surface, None);
            self.debug_report_loader.take().unwrap().destroy_debug_report_callback_ext(
                self.debug_report.take().unwrap(), None);
            self.instance.destroy_instance(None);
            info!("Dropped Instance.")
        }
    }
}

impl Instance {
    pub fn new(window : &winit::Window) -> Self {
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

            let debug_info = vk::DebugReportCallbackCreateInfoEXT {
                s_type: vk::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
                p_next: ptr::null(),
                flags: vk::DebugReportFlagsEXT::ERROR
                    | vk::DebugReportFlagsEXT::WARNING
                    | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
                pfn_callback: Some(debug_callback),
                p_user_data: ptr::null_mut(),
            };

            let debug_report_loader = DebugReport::new(&entry, &instance);
            let debug_report = debug_report_loader.create_debug_report_callback_ext(
                &debug_info,
                None)
                .unwrap();

            let surface_loader = Surface::new(&entry, &instance);
            let surface = create_surface(&entry, &instance, window);

            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Failed to retrieve physical devices.");

            Self {
                entry,
                instance,
                debug_report_loader: Some(debug_report_loader),
                debug_report: Some(debug_report),
                surface_loader,
                surface,
                physical_devices
            }
        }
    }

    pub fn select_physical_device(mut self) -> vk::PhysicalDevice {
        self.physical_devices.remove(0)
    }
}
