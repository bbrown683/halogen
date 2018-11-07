use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use hal::format;
use hal::{Backend, Device, PresentMode, Surface, SurfaceCapabilities,
          SwapchainConfig};
use hal::pso::{Rect, Viewport};
use crate::gfx::{GfxDevice};

/// Controls the presentation to a surface.
pub struct GfxSwapchain<B: Backend> {
    device : Rc<RefCell<GfxDevice<B>>>,
    current_image : u32,
    caps : SurfaceCapabilities,
    swap_config : SwapchainConfig,
    viewport : Viewport,
    swapchain : Option<B::Swapchain>,
    acquire_semaphores : Option<Vec<B::Semaphore>>
}

impl<B: Backend> Drop for GfxSwapchain<B>{
    fn drop(&mut self) {
        for acquire_semaphore in self.acquire_semaphores.take().unwrap() {
            &self.device.borrow().get_device().destroy_semaphore(acquire_semaphore);
        }
        debug_assert!(self.acquire_semaphores.is_none());
        &self.device.borrow().get_device().destroy_swapchain(self.swapchain.take().unwrap());
        debug_assert!(self.swapchain.is_none());
    }
}

impl<B: Backend> GfxSwapchain<B> {
    /// Creates a new swapchain with the given surface. This function will only need to be called once.
    /// Any events that break the existing swapchain `should` call `recreate`.
    pub fn new(device : Rc<RefCell<GfxDevice<B>>>,
               mut surface : &mut B::Surface,
               image_count : u32) -> Result<Self,String> {
        // Check to see if queue supports presentation.
        if !surface.supports_queue_family(device.borrow().get_graphics_queue_family()) {
            return Err("graphics queue does not support presenting to swapchain.".to_string());
        }

        // Grab surface capabilities, formats, and present modes.
        // TODO: find best format and present mode from iterator. we are using selected defaults currently.
        let (caps, _formats, _present_modes) = surface.compatibility(device.borrow().get_physical_device());
        if !caps.image_count.contains(&image_count) {
            return Err("image_count parameter was not within valid boundaries.".to_string());
        }

        let extent = caps.current_extent.unwrap().to_extent();
        let swap_config = SwapchainConfig::new(
            extent.width,
            extent.height,
            format::Format::Rgba8Unorm,
            image_count)
            .with_mode(PresentMode::Fifo); // Vulkan spec guarantee's this mode.

        let viewport = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: extent.width as _,
                h: extent.height as _,
            },
            depth: 0.0..1.0,
        };

        let (swapchain, _backbuffer) = device
            .borrow()
            .get_device()
            .create_swapchain(&mut surface, swap_config.clone(), None)
            .expect("Failed to create swapchain.");

        // Initialize our acquire semaphores.
        let acquire_semaphores = iter::repeat_with(
            ||device.borrow().get_device().create_semaphore().expect("Failed to create semaphore."))
            .take(image_count as _)
            .collect();

        Ok(Self { device, current_image: 0, caps, swap_config, viewport, swapchain: Some(swapchain),
            acquire_semaphores: Some(acquire_semaphores) })
    }

    pub fn get_current_image(&self) -> u32 {
        self.current_image.clone()
    }

    pub fn get_surface_capabilities(&self) -> SurfaceCapabilities {
        self.caps.clone()
    }

    pub fn get_swapchain_config(&self) -> SwapchainConfig {
        self.swap_config.clone()
    }

    pub fn get_viewport(&self) -> Viewport {
        self.viewport.clone()
    }

    pub fn get_swapchain(&self) -> &Option<B::Swapchain> {
       &self.swapchain
    }
}