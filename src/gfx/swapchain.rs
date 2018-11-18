use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;
use ash::extensions::{Surface as SurfaceLoader, Swapchain as SwapchainLoader};
use ash::vk;
use super::{Device, Instance, Queue};
use super::util::select_color_format;
use super::platform::{create_surface, get_required_instance_extensions};

/// Provides a brief overview of why a swapchain failed to be created.
pub enum SwapchainCreationError {
    /// Provided presentation queue does not support presentation.
    QueuePresentUnsupported,
}

pub struct Swapchain {
    surface_loader : SurfaceLoader,
    surface : vk::SurfaceKHR,
    capabilities : vk::SurfaceCapabilitiesKHR,
    formats : Vec<vk::SurfaceFormatKHR>,
    present_modes : Vec<vk::PresentModeKHR>,
    swapchain_loader : SwapchainLoader,
    swapchain : vk::SwapchainKHR,
    images : Vec<vk::Image>,
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.swapchain_loader.destroy_swapchain_khr(self.swapchain, None);
            self.surface_loader.destroy_surface_khr(self.surface, None);
            info!("Dropped Swapchain")
        }
    }
}

impl Swapchain {
    pub fn new(instance : &Instance,
               device : &Device,
               present_queue : Rc<RefCell<Queue>>,
               window : &winit::Window,
               images : u32) -> Result<Self,SwapchainCreationError> {
        unsafe {
            let surface_loader = SurfaceLoader::new(
                instance.get_ash_entry(),
                instance.get_ash_instance());
            let surface = create_surface(
                instance.get_ash_entry(),
                instance.get_ash_instance(), window);

            if !surface_loader.get_physical_device_surface_support_khr(
                device.get_physical_device(),
                0,
                surface.clone()) {
                return Err(SwapchainCreationError::QueuePresentUnsupported);
            }

            let capabilities = surface_loader
                .get_physical_device_surface_capabilities_khr(
                    device.get_physical_device(),
                    surface.clone())
                .unwrap();

            let formats = surface_loader
                .get_physical_device_surface_formats_khr(
                    device.get_physical_device(),
                surface.clone())
                .unwrap();

            let present_modes = surface_loader
                .get_physical_device_surface_present_modes_khr(
                    device.get_physical_device(),
                    surface.clone())
                .unwrap();

            let swapchain_loader = SwapchainLoader::new(
                instance.get_ash_instance(),
                device.get_ash_device());

            let (format, color_space) = select_color_format(
                formats.clone(),
                vk::Format::B8G8R8A8_SRGB);

            let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface)
                .image_extent(capabilities.current_extent)
                .image_format(format)
                .image_color_space(color_space)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
                .image_array_layers(1)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .min_image_count(2)
                .clipped(true)
                .build();

            let swapchain = swapchain_loader
                .create_swapchain_khr(&swapchain_info, None)
                .expect("Failed to create swapchain");
            let images = swapchain_loader
                .get_swapchain_images_khr(swapchain)
                .unwrap();
            Ok(Self { surface_loader,
                surface,
                capabilities,
                formats,
                present_modes,
                swapchain_loader,
                swapchain,
                images,
            })
        }
    }

    pub fn present(&self) {

    }

    pub fn get_images(&self) -> Vec<vk::Image> {
        self.images.clone()
    }

    pub fn get_extent(&self) -> vk::Extent2D {
        self.capabilities.current_extent
    }
}