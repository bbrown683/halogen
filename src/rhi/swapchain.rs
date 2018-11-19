use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use ash::extensions::{Surface as SurfaceLoader, Swapchain as SwapchainLoader};
use ash::version::DeviceV1_0;
use ash::vk;
use super::{Device, Instance, Queue};
use super::util::select_color_format;
use super::platform::{create_surface, get_required_instance_extensions};

/// Provides a brief overview of why a swapchain failed to be created.
pub enum SwapchainCreationError {
    /// Provided presentation queue does not support presentation.
    QueuePresentUnsupported,
    InvalidImageCount,
}

pub struct Swapchain {
    instance : Rc<RefCell<Instance>>,
    device : Rc<RefCell<Device>>,
    present_queue : Rc<RefCell<Queue>>,
    surface_loader : SurfaceLoader,
    surface : vk::SurfaceKHR,
    capabilities : vk::SurfaceCapabilitiesKHR,
    format : vk::Format,
    formats : Vec<vk::SurfaceFormatKHR>,
    present_modes : Vec<vk::PresentModeKHR>,
    swapchain_loader : SwapchainLoader,
    swapchain : vk::SwapchainKHR,
    acquire_semaphores : Vec<vk::Semaphore>,
    images : Vec<vk::Image>,
    image_count : u32,
    current_image : u32,
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            for semaphore in self.acquire_semaphores.clone() {
                self.device.borrow().get_ash_device().destroy_semaphore(semaphore, None);
            }
            self.swapchain_loader.destroy_swapchain_khr(self.swapchain, None);
            self.surface_loader.destroy_surface_khr(self.surface, None);
            info!("Dropped Swapchain")
        }
    }
}

impl Swapchain {
    /// Creates a new swapchain with the given surface. This function will only need to be called once.
    /// Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(instance : Rc<RefCell<Instance>>,
               device : Rc<RefCell<Device>>,
               present_queue : Rc<RefCell<Queue>>,
               window : &winit::Window,
               image_count : u32) -> Result<Self,SwapchainCreationError> {
        // Initializes surface entry points and creates one.
        let surface_loader = SurfaceLoader::new(
            instance.borrow().get_ash_entry(),
            instance.borrow().get_ash_instance());
        let surface = create_surface(
            instance.borrow().get_ash_entry(),
            instance.borrow().get_ash_instance(), window);

        let supports_present = unsafe {
            surface_loader.get_physical_device_surface_support_khr(
                device.borrow().get_physical_device(),
                0,
                surface)
        };

        // Verifies that the device supports presentation.
        if !supports_present {
            return Err(SwapchainCreationError::QueuePresentUnsupported);
        }

        // Grab surface capabilities, formats, and present modes.
        let (capabilities, formats, present_modes) = unsafe {
            let capabilities = surface_loader
                .get_physical_device_surface_capabilities_khr(
                    device.borrow().get_physical_device(),
                    surface)
                .unwrap();
            let formats = surface_loader
                .get_physical_device_surface_formats_khr(
                    device.borrow().get_physical_device(),
                    surface)
                .unwrap();
            let present_modes = surface_loader
                .get_physical_device_surface_present_modes_khr(
                    device.borrow().get_physical_device(),
                    surface)
                .unwrap();
            (capabilities, formats, present_modes)
        };

        let swapchain_loader = SwapchainLoader::new(
            instance.borrow().get_ash_instance(),
            device.borrow().get_ash_device());

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
            .min_image_count(image_count)
            .clipped(true)
            .build();

        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain_khr(&swapchain_info, None)
                .expect("Failed to create swapchain")
        };

        // Initialize our acquire semaphores.
        let semaphore_info = vk::SemaphoreCreateInfo::builder()
            .build();
        let acquire_semaphores = iter::repeat_with(||
            unsafe {
                device
                    .borrow()
                    .get_ash_device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore")
            })
            .take(image_count as _)
            .collect();

        let images = unsafe {
            swapchain_loader
                .get_swapchain_images_khr(swapchain)
                .unwrap()
        };

        Ok(Self { instance,
            device,
            present_queue,
            surface_loader,
            surface,
            capabilities,
            format,
            formats,
            present_modes,
            swapchain_loader,
            swapchain,
            acquire_semaphores,
            images,
            image_count,
            current_image: 0,
        })
    }

    /// Returns the next image index in the swapchain. This is typically used at the beginning of a render pass.
    pub fn get_next_image(&mut self) -> u32 {
        let acquire_result = unsafe {
            self.swapchain_loader
                .acquire_next_image_khr(
                    self.swapchain,
                    u64::max_value(),
                    self.acquire_semaphores
                        .get(self.current_image as usize)
                        .unwrap()
                        .clone(),
                    vk::Fence::null())
        };
        if acquire_result.is_err() {
            error!("Failed to acquire image!");
        } else {
            self.current_image = acquire_result.unwrap().0;
        }
        self.current_image
    }

    /// Presents the image to the screen, using the specified present queue. The present queue can be any queue
    /// graphics, transfer, compute which supports present operations.
    pub fn present(&self) {
        // TODO: Allow usage of wait semaphores.
        let present_info = vk::PresentInfoKHR::builder()
            .image_indices(&[self.current_image])
            .swapchains(&[self.swapchain])
            .build();
        // TODO: Use value to validate present status.
        let present_status = unsafe {
            self.swapchain_loader.queue_present_khr(
                self.present_queue.borrow().get_queue_raw(),
                &present_info)
        };
    }

    /// Recreates the swapchain. This is particularly useful in the event of resizes.
    pub fn recreate(&mut self) {
        unsafe {
            self.capabilities = self.surface_loader
                .get_physical_device_surface_capabilities_khr(
                    self.device.borrow().get_physical_device(),
                    self.surface)
                .unwrap();
            self.formats = self.surface_loader
                .get_physical_device_surface_formats_khr(
                    self.device.borrow().get_physical_device(),
                    self.surface)
                .unwrap();
            self.present_modes = self.surface_loader
                .get_physical_device_surface_present_modes_khr(
                    self.device.borrow().get_physical_device(),
                    self.surface)
                .unwrap();
        }

        let (format, color_space) = select_color_format(
            self.formats.clone(),
            vk::Format::B8G8R8A8_SRGB);

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .old_swapchain(self.swapchain)
            .image_extent(self.capabilities.current_extent)
            .image_format(format)
            .image_color_space(color_space)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .image_array_layers(1)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .min_image_count(self.image_count)
            .clipped(true)
            .build();

        self.swapchain = unsafe {
            let new_swapchain = self.swapchain_loader
                .create_swapchain_khr(&swapchain_info, None)
                .expect("Failed to create swapchain");
            self.swapchain_loader.destroy_swapchain_khr(self.swapchain, None);
            (new_swapchain)
        };

        self.format = format;
        self.images = unsafe {
            self.swapchain_loader
                .get_swapchain_images_khr(self.swapchain)
                .unwrap()
        };
    }

    /// Returns the images associated with this Swapchain, used in the creation of a Framebuffer.
    pub fn get_images(&self) -> Vec<vk::Image> {
        self.images.clone()
    }

    /// Returns the current image index which the swapchain is referring to.
    pub fn get_current_image(&self) -> u32 {
        self.current_image
    }

    /// Returns the capabilities provided by the surface which initialized this Swapchain.
    pub fn get_capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        self.capabilities
    }

    /// Returns all formats supported by the surface initialized with the Swapchain.
    pub fn get_supported_formats(&self) -> Vec<vk::SurfaceFormatKHR> {
        self.formats.clone()
    }

    pub fn get_format(&self) -> vk::Format {
        self.format
    }

    /// Returns all present modes supported by the surface initialized with the Swapchain.
    pub fn get_supported_present_modes(&self) -> Vec<vk::PresentModeKHR> {
        self.present_modes.clone()
    }
}