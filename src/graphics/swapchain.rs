use std::{cell::RefCell, iter, rc::Rc};
use ash::extensions::{khr::Surface as SurfaceLoader, khr::Swapchain as SwapchainLoader};
use ash::version::DeviceV1_0;
use ash::vk::{self, Result as VkResult};
use winit::window::Window;
use super::{Device, Instance, Queue, platform::create_surface, platform::get_required_instance_extensions, util::select_color_format};

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
    surface_format : vk::SurfaceFormatKHR,
    capabilities : vk::SurfaceCapabilitiesKHR,
    formats : Vec<vk::SurfaceFormatKHR>,
    present_modes : Vec<vk::PresentModeKHR>,
    swapchain_loader : SwapchainLoader,
    swapchain : vk::SwapchainKHR,
    acquire_semaphores : Vec<vk::Semaphore>,
    acquire_fences : Vec<vk::Fence>,
    images : Vec<vk::Image>,
    image_count : u32,
    current_frame : u32,
    current_image : u32,
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().ash_device().device_wait_idle().unwrap();
            for semaphore in self.acquire_semaphores.clone() {
                self.device.borrow().ash_device().destroy_semaphore(semaphore, None);
            }
            for fence in self.acquire_fences.clone() {
                self.device.borrow().ash_device().destroy_fence(fence, None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);
        }
        info!("Dropped Swapchain")
    }
}

impl Swapchain {
    /// Creates a new swapchain with the given surface. This function will only need to be called once.
    /// Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(instance : Rc<RefCell<Instance>>,
               device : Rc<RefCell<Device>>,
               present_queue : Rc<RefCell<Queue>>,
               window : &Window,
               image_count : u32) -> Result<Self,SwapchainCreationError> {
        // Initializes surface entry points and creates one.
        let surface_loader = SurfaceLoader::new(
            instance.borrow().ash_entry(),
            instance.borrow().ash_instance());
        let surface = create_surface(
            instance.borrow().ash_entry(),
            instance.borrow().ash_instance(), window);

        let supports_present = unsafe {
            surface_loader.get_physical_device_surface_support(
                device.borrow().physical_device(),
                0,
                surface)
        };

        // Verifies that the device supports presentation.
        if !supports_present.unwrap() {
            return Err(SwapchainCreationError::QueuePresentUnsupported);
        }

        // Grab surface capabilities, formats, and present modes.
        let (capabilities, formats, present_modes) = unsafe {
            let capabilities = surface_loader
                .get_physical_device_surface_capabilities(
                    device.borrow().physical_device(),
                    surface)
                .unwrap();
            let formats = surface_loader
                .get_physical_device_surface_formats(
                    device.borrow().physical_device(),
                    surface)
                .unwrap();
            let present_modes = surface_loader
                .get_physical_device_surface_present_modes(
                    device.borrow().physical_device(),
                    surface)
                .unwrap();
            (capabilities, formats, present_modes)
        };

        let swapchain_loader = SwapchainLoader::new(
            instance.borrow().ash_instance(),
            device.borrow().ash_device());

        let surface_format = select_color_format(
            formats.clone(),
            vk::Format::B8G8R8A8_SRGB);

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .image_extent(capabilities.current_extent)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .image_array_layers(1)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .min_image_count(image_count)
            .clipped(true);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_info, None)
                .expect("Failed to create swapchain")
        };

        // Initialize our acquire semaphores.
        let semaphore_info = vk::SemaphoreCreateInfo::builder()
            .build();
        let acquire_semaphores = iter::repeat_with(||
            unsafe {
                device
                    .borrow()
                    .ash_device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create semaphore")
            })
            .take(image_count as _)
            .collect();

        let fence_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED);
        let acquire_fences = iter::repeat_with(||
            unsafe {
                device
                    .borrow()
                    .ash_device()
                    .create_fence(&fence_info, None)
                    .expect("Failed to create fence")
            })
            .take(image_count as _)
            .collect();

        let images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .unwrap()
        };

        Ok(Self { instance,
            device,
            present_queue,
            surface_loader,
            surface,
            surface_format,
            capabilities,
            formats,
            present_modes,
            swapchain_loader,
            swapchain,
            acquire_semaphores,
            acquire_fences,
            images,
            image_count,
            current_frame: 0,
            current_image: 0,
        })
    }

    /// Returns the next image index in the swapchain. This is typically used at the beginning of a render pass.
    pub fn acquire_next_image(&mut self) -> u32 {
        self.current_frame = (self.current_frame + 1) % self.image_count;
        let acquire_result = unsafe {
            // Wait for these fences to be signalled then reset them to a non-signalled state.
            self.device
                .borrow()
                .ash_device()
                .wait_for_fences(&[self.acquire_fences.get(self.current_frame as usize).unwrap().clone()], true, u64::max_value())
                .unwrap();
            self.device
                .borrow()
                .ash_device()
                .reset_fences(&[self.acquire_fences.get(self.current_frame as usize).unwrap().clone()])
                .unwrap();
            // Attempt to acquire the next image from the swapchain.
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    u64::max_value(),
                    // Signal this semaphore on completion. Present queue waits for this to complete before submission.
                    self.acquire_semaphores.get(self.current_frame as usize).unwrap().clone(),
                    vk::Fence::null())
        };
        match acquire_result {
            Ok(index) => self.current_image = index.0,
            // TODO: Handle these events.
            Err(error) => match error {
                VkResult::ERROR_SURFACE_LOST_KHR => error!("Lost surface"),
                VkResult::ERROR_OUT_OF_DATE_KHR => error!("Images are out of date"),
                _ => (),
            }
        }
        self.current_image
    }

    /// Presents the image to the screen, using the specified present queue. The present queue can be any queue
    /// graphics, transfer, compute which supports present operations.
    pub fn present(&self) {
        let present_info = vk::PresentInfoKHR::builder()
            .image_indices(&[self.current_image])
            .swapchains(&[self.swapchain])
            // Wait on submission to be completed before presenting.
            .wait_semaphores(&[self.present_queue.borrow().submit_semaphore_raw()])
            .build();
        // TODO: Use value to validate present status.
        let present_status = unsafe {
            self.swapchain_loader.queue_present(
                self.present_queue.borrow().queue_raw(),
                &present_info)
        };
        match present_status {
            Ok(status) => (),
            Err(error) => (),
        }
    }

    /// Recreates the swapchain. This is particularly useful in the event of resizes.
    pub fn recreate(&mut self) {
        unsafe {
            self.capabilities = self.surface_loader
                .get_physical_device_surface_capabilities(
                    self.device.borrow().physical_device(),
                    self.surface)
                .unwrap();
            self.formats = self.surface_loader
                .get_physical_device_surface_formats(
                    self.device.borrow().physical_device(),
                    self.surface)
                .unwrap();
            self.present_modes = self.surface_loader
                .get_physical_device_surface_present_modes(
                    self.device.borrow().physical_device(),
                    self.surface)
                .unwrap();
        }

        self.surface_format = select_color_format(
            self.formats.clone(),
            vk::Format::B8G8R8A8_SRGB);

        let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .old_swapchain(self.swapchain)
            .image_extent(self.capabilities.current_extent)
            .image_format(self.surface_format.format)
            .image_color_space(self.surface_format.color_space)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .image_array_layers(1)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .min_image_count(self.image_count)
            .clipped(true);
        self.swapchain = unsafe {
            let new_swapchain = self.swapchain_loader
                .create_swapchain(&swapchain_info, None)
                .expect("Failed to create swapchain");
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            new_swapchain
        };

        self.images = unsafe {
            self.swapchain_loader
                .get_swapchain_images(self.swapchain)
                .unwrap()
        };
        info!("Recreated Swapchain")
    }

    /// Returns the images associated with this Swapchain, used in the creation of a Framebuffer.
    pub fn images(&self) -> Vec<vk::Image> {
        self.images.clone()
    }

    /// Returns the current image index which the swapchain is referring to.
    pub fn current_image(&self) -> u32 {
        self.current_image
    }

    /// Returns the capabilities provided by the surface which initialized this Swapchain.
    pub fn capabilities(&self) -> vk::SurfaceCapabilitiesKHR {
        self.capabilities
    }

    /// Returns all formats supported by the surface initialized with the Swapchain.
    pub fn supported_formats(&self) -> Vec<vk::SurfaceFormatKHR> {
        self.formats.clone()
    }

    /// Returns the surface format which was selected for the swapchain.
    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        self.surface_format
    }

    /// Returns all present modes supported by the surface initialized with the Swapchain.
    pub fn supported_present_modes(&self) -> Vec<vk::PresentModeKHR> {
        self.present_modes.clone()
    }

    /// Returns the semaphore being used by the swapchain.
    pub fn current_acquire_semaphore(&self) -> vk::Semaphore {
        self.acquire_semaphores
            .get(self.current_frame as usize)
            .unwrap()
            .clone()
    }

    /// Returns the current fence being used by the swapchain.
    pub fn current_acquire_fence(&self) -> vk::Fence {
        self.acquire_fences
            .get(self.current_frame as usize)
            .unwrap()
            .clone()
    }
}