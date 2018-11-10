use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use hal::format::{self, Format};
use hal::{AcquireError, Backbuffer, Backend, Capability, Device as LogicalDevice, FrameSync, PresentMode, Surface, SurfaceCapabilities,
         Swapchain as GfxSwapchain, SwapchainConfig, SwapImageIndex};
use crate::gfx::{Device, Queue};

/// Controls the presentation to a surface.
pub struct Swapchain<B: Backend, C: Capability> {
    device : Rc<RefCell<Device<B>>>,
    present_queue : Rc<RefCell<Queue<B, C>>>,
    current_image : SwapImageIndex,
    image_count : u32,
    caps : SurfaceCapabilities,
    formats : Vec<Format>,
    present_modes : Vec<PresentMode>,
    swap_config : SwapchainConfig,
    swapchain : Option<B::Swapchain>,
    backbuffer : Backbuffer<B>,
    acquire_semaphores : Option<Vec<B::Semaphore>>
}

impl<B: Backend, C: Capability> Drop for Swapchain<B, C>{
    fn drop(&mut self) {
        for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
            &self.device.borrow().get_logical_device().destroy_semaphore(acquire_semaphore);
        }
        debug_assert!(self.acquire_semaphores.is_none());
        &self.device.borrow().get_logical_device().destroy_swapchain(self.swapchain.take().unwrap());
        debug_assert!(self.swapchain.is_none());
        println!("Dropped Swapchain.")
    }
}

impl<B: Backend, C: Capability> Swapchain<B, C> {
    /// Creates a new swapchain with the given surface. This function will only need to be called once.
    /// Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(device : Rc<RefCell<Device<B>>>,
               present_queue : Rc<RefCell<Queue<B, C>>>,
               mut surface : &mut B::Surface,
               image_count : u32) -> Result<Self,String> {
        // Check to see if queue supports presentation.
        if !surface.supports_queue_family(device.borrow().get_queue_family(present_queue.borrow().get_queue_group().family())) {
            return Err("graphics queue does not support presenting to swapchain.".to_string());
        }

        // Grab surface capabilities, formats, and present modes.
        // TODO: find best format and present mode from iterator. we are using selected defaults currently.
        let (caps, formats, present_modes) = surface.compatibility(device.borrow().get_physical_device());
        if !caps.image_count.contains(&image_count) {
            return Err("image_count parameter was not within valid boundaries.".to_string());
        }

        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Bgra8Srgb, // Most common supported format according to gpuinfo.
            image_count)
            .with_mode(PresentMode::Fifo); // Vulkan spec guarantee's this mode.

        let (swapchain, backbuffer) = device
            .borrow()
            .get_logical_device()
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");

        // Initialize our acquire semaphores.
        let acquire_semaphores = iter::repeat_with(
            ||device.borrow().get_logical_device().create_semaphore().expect("Failed to create semaphore."))
            .take(image_count as _)
            .collect();

        Ok(Self { device, present_queue, current_image: 0, image_count, caps, formats: formats.unwrap(),
            present_modes, swap_config, swapchain: Some(swapchain), backbuffer, acquire_semaphores: Some(acquire_semaphores) })
    }

    /// Picks the color format for the swapchain.
    fn select_color_format(self, formats : Vec<Format>, preferred : Option<Format>) -> Format { unimplemented!() }

    /// Selects the present mode to use for the swapchain.
    fn select_present_mode(self, present_modes : Vec<PresentMode>) -> PresentMode { unimplemented!()}

    /// Presents the image to the screen, using the specified present queue. The present queue can be any queue
    /// graphics, transfer, compute which supports present operations.
    pub fn present(&mut self) {
        let present = self.swapchain
            .as_ref()
            .unwrap()
            .present(&mut self.present_queue.borrow_mut().get_queue_group_mut().queues[0], self.current_image, &[])
            .unwrap();
    }

    pub fn recreate(&mut self, mut surface : &mut B::Surface) {
        &self.device.borrow().get_logical_device().wait_idle().unwrap();

        let (caps, formats, present_modes) = surface.compatibility(self.device.borrow().get_physical_device());
        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Bgra8Srgb, // Most common supported format according to gpuinfo.
            self.image_count)
            .with_mode(PresentMode::Fifo); // Vulkan spec guarantee's this mode.

        let (swapchain, backbuffer) = self.device
            .borrow()
            .get_logical_device()
            .create_swapchain(&mut surface, swap_config.clone(), self.swapchain.take())
            .expect("Failed to recreate swapchain.");

        // Update our parameters to their new values.
        self.caps = caps;
        self.formats = formats.unwrap();
        self.present_modes = present_modes;
        self.swap_config = swap_config;
        self.swapchain = Some(swapchain);
        self.backbuffer = backbuffer;
    }

    pub fn get_current_image(&self) -> SwapImageIndex {
        self.current_image.clone()
    }

    pub fn get_surface_capabilities(&self) -> SurfaceCapabilities {
        self.caps.clone()
    }

    pub fn get_swapchain_config(&self) -> SwapchainConfig {
        self.swap_config.clone()
    }

    pub fn get_supported_formats(&self) -> Vec<Format> { self.formats.clone() }

    pub fn get_supported_present_modes(&self) -> Vec<PresentMode> { self.present_modes.clone() }

    pub fn get_swapchain(&self) -> &Option<B::Swapchain> {
       &self.swapchain
    }
}